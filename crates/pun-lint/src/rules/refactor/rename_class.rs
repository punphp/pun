//! `lint/refactor/rename-class`, ported from Rector 2.6.1's `RenameClassRector` (rules/Renaming/Rector/Name) with its
//! `ClassRenamer`, `DocBlockClassRenamer` (`ClassRenamePhpDocNodeVisitor`) and `PhpDocClassRenamer`, then the post
//! rectors Rector runs on the file (`rector_names::post`).
//!
//! Options: a map of old class names to new ones (`configure()` checks only that both are strings), `{ "App\\OldClass" = "App\\NewClass" }`, and the import options of
//! Rector's configuration beside it, which no class name can be: `import-names` (`RectorConfig::importNames()`, off),
//! `import-short-classes` (`importShortClasses()`, on) and `remove-unused-imports` (`removeUnusedImports()`, off).
//!
//! A fully qualified class name (every class name, once php-parser's `NameResolver` ran) that is an old one becomes
//! the new one, fully qualified, unless the change would make invalid code: `new` of an interface, a class extending
//! an interface, a final class or itself, a class implementing a class, a static call on an interface, or a constant
//! the new class lacks. The docs of properties, functions, expression statements, class-likes and `if` statements
//! get the new name where their types (and the Doctrine `targetEntity`, JMS `Type` and Symfony `Choice` callback
//! annotations) name an old class. Rector runs the rule once per file.

use mago_syntax::cst::{Node, NodeKind};
use pun_problem::{Edit, Span};
use serde_json::Value;

use super::report;
use crate::rules::rector_names::doc::ItemValue;
use crate::rules::rector_names::post::{Options, Session, doc_type};
use crate::rules::rector_names::resolve::{has_class, resolve_tag};
use crate::rules::types::facts::facts;
use crate::{Context, Report, Rule};

/// `NodeTypes::TYPE_AWARE_DOCTRINE_ANNOTATION_CLASSES`.
const TYPE_AWARE_ANNOTATIONS: &[&str] = &[
    JMS_TYPE,
    "Doctrine\\ORM\\Mapping\\OneToMany",
    "Symfony\\Component\\Validator\\Constraints\\Choice",
    "Symfony\\Component\\Validator\\Constraints\\Email",
    "Symfony\\Component\\Validator\\Constraints\\Range",
];
const JMS_TYPE: &str = "JMS\\Serializer\\Annotation\\Type";

#[derive(Default)]
pub struct RenameClass {
    renames: Vec<(String, String)>,
    options: Options,
}

impl Rule for RenameClass {
    fn id(&self) -> &'static str {
        "lint/refactor/rename-class"
    }

    /// Rector runs the rule once per file; a rename to a class that is itself renamed would chain.
    fn once(&self) -> bool {
        true
    }

    fn configure(&mut self, options: &Value) -> Result<(), String> {
        self.renames.clear();
        self.options = Options {
            import_short_classes: true,
            ..Options::default()
        };
        let map = match options {
            Value::Null => return Ok(()),
            Value::Object(map) => map,
            _ => return Err("options must be a table".into()),
        };
        for (key, value) in map {
            let flag = match key.as_str() {
                "import-names" => &mut self.options.import_names,
                "import-short-classes" => &mut self.options.import_short_classes,
                "remove-unused-imports" => &mut self.options.remove_unused_imports,
                _ => {
                    let Some(new) = value.as_str() else {
                        return Err(format!("`{key}` must be a string"));
                    };
                    self.renames.push((key.clone(), new.to_string()));
                    continue;
                }
            };
            *flag = value.as_bool().ok_or_else(|| format!("`{key}` must be a boolean"))?;
        }
        Ok(())
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Program])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Program(program) = node else {
            return;
        };
        if self.renames.is_empty() {
            return;
        }
        let mut session = Session::new(ctx, program);
        self.rename_names(ctx, &mut session);
        self.rename_docs(ctx, &mut session);
        dedupe_implements(&mut session);
        let Some(fixed) = session.finish(self.options, true) else {
            return;
        };
        let source = ctx.source;
        let prefix = source.bytes().zip(fixed.bytes()).take_while(|(a, b)| a == b).count();
        let suffix = source[prefix..]
            .bytes()
            .rev()
            .zip(fixed[prefix..].bytes().rev())
            .take_while(|(a, b)| a == b)
            .count();
        let (mut start, mut end) = (prefix, source.len() - suffix);
        let mut new_end = fixed.len() - suffix;
        while !source.is_char_boundary(start) || !fixed.is_char_boundary(start) {
            start -= 1;
        }
        while !source.is_char_boundary(end) || !fixed.is_char_boundary(new_end) {
            end += 1;
            new_end += 1;
        }
        out.push(report(
            Span::new(start, end),
            "Replace the renamed classes by their new names.".into(),
            vec![Edit::new(Span::new(start, end), fixed[start..new_end].to_string())],
        ));
    }
}

impl RenameClass {
    /// `ClassRenamer::refactorName()` on each fully qualified name.
    fn rename_names(&self, ctx: &Context<'_>, session: &mut Session<'_>) {
        let codebase = &facts(ctx).codebase;
        for i in 0..session.name_count() {
            let name = session.view.names[i].clone();
            if name.kind == crate::rules::rector_names::view::Kind::Function {
                continue;
            }
            let Some(old) = session.fully_qualified(i).map(str::to_string) else {
                continue;
            };
            // `shouldSkipClassConstFetchForMissingConstantName()`
            if has_class(ctx, &old)
                && let Some(constant) = &name.class_const_fetch
            {
                let missing = self.renames.iter().any(|(from, to)| {
                    old.eq_ignore_ascii_case(from)
                        && has_class(ctx, to)
                        && codebase.class_constant_exists(from.as_bytes(), constant.as_bytes())
                        && !codebase.class_constant_exists(to.as_bytes(), constant.as_bytes())
                });
                if missing {
                    continue;
                }
            }
            let Some((_, new)) = self.renames.iter().find(|(from, _)| *from == old) else {
                continue;
            };
            if let Some(class) = codebase.get_class_like(new.as_bytes()) {
                let is_interface = class.kind.is_interface();
                // `isClassToInterfaceValidChange()`
                if name.new && is_interface {
                    continue;
                }
                if !name.new && name.extends && name.class.as_deref() == Some(new.as_str()) {
                    continue;
                }
                // `isValidClassNameChange()`
                if name.extends && (is_interface || class.flags.is_final()) {
                    continue;
                }
                if name.implements && class.kind.is_class() {
                    continue;
                }
                // `shouldSkip()`: a static call on an interface
                if name.static_call && is_interface {
                    continue;
                }
            }
            session.collect_renamed(&old);
            session.rename(i, new);
        }
    }

    /// `ClassRenamer::refactorPhpDoc()` on the docs of the nodes the rule reads.
    fn rename_docs(&self, ctx: &Context<'_>, session: &mut Session<'_>) {
        for d in session.doc_indexes() {
            let Some(view) = session.doc_view(d) else {
                continue;
            };
            if !view.owners.renamed || !session.doc_is_last(d) {
                continue;
            }
            let Some(doc) = session.doc(d) else {
                session.unsupported();
                continue;
            };
            if !doc.has_type_aware() && doc.annotation(TYPE_AWARE_ANNOTATIONS).is_none() {
                continue;
            }
            let mut doc = doc.clone();
            let mut collected = Vec::new();
            let scope = session.doc_scope(d);
            // `ClassRenamePhpDocNodeVisitor`: each identifier whose object type is an old class
            for ident in doc.idents() {
                let Some(class) = doc_type(ctx, scope, &ident.name).class_name().map(str::to_string) else {
                    continue;
                };
                if let Some((old, new)) = self.renames.iter().find(|(old, _)| *old == class) {
                    doc.set_ident(&ident, &format!("\\{new}"));
                    collected.push(old.clone());
                }
            }
            // `PhpDocClassRenamer`
            let mut unsupported = false;
            for class in [
                "Symfony\\Component\\Validator\\Constraints\\Choice",
                "Doctrine\\ORM\\Mapping\\OneToMany",
                JMS_TYPE,
            ] {
                let classes: &[&str] = match class {
                    "Doctrine\\ORM\\Mapping\\OneToMany" => &[
                        "Doctrine\\ORM\\Mapping\\OneToMany",
                        "Doctrine\\ORM\\Mapping\\ManyToMany",
                        "Doctrine\\ORM\\Mapping\\Embedded",
                    ],
                    _ => &[class],
                };
                let Some(child) = doc.annotation(classes) else {
                    continue;
                };
                let embedded = doc.annotation(&["Doctrine\\ORM\\Mapping\\Embedded"]) == Some(child);
                let Some(items) = doc.items_mut(child) else {
                    unsupported = true;
                    continue;
                };
                let mut changed = false;
                match class {
                    // a callback is a `{...}` list, which Pun does not read
                    "Symfony\\Component\\Validator\\Constraints\\Choice" => {}
                    "Doctrine\\ORM\\Mapping\\OneToMany" => {
                        let key = if embedded { "class" } else { "targetEntity" };
                        if let Some(item) = items.iter_mut().find(|i| i.key.as_deref() == Some(key))
                            && let ItemValue::String(target) = &item.value
                        {
                            let resolved = resolve_tag(ctx, scope, target);
                            for (old, new) in &self.renames {
                                if resolved == *old {
                                    collected.push(old.clone());
                                    item.value = ItemValue::String(new.clone());
                                    changed = true;
                                }
                            }
                        }
                    }
                    _ => {
                        for (old, new) in &self.renames {
                            if let Some(item) = items.iter_mut().find(|i| i.key.is_none())
                                && let ItemValue::String(value) = &item.value
                            {
                                if value == old {
                                    item.value = ItemValue::String(new.clone());
                                    continue;
                                }
                                collected.push(old.clone());
                                // `'#\b' . preg_quote($oldClass, '#') . 'b#'`: the old name followed by a `b`
                                let replaced = replace_followed_by_b(value, old, new);
                                item.value = ItemValue::String(replaced);
                                changed = true;
                            }
                            if let Some(item) = items.iter_mut().find(|i| i.key.as_deref() == Some("type"))
                                && let ItemValue::String(value) = &item.value
                                && value == old
                            {
                                item.value = ItemValue::String(new.clone());
                                changed = true;
                            }
                        }
                    }
                }
                if changed {
                    doc.set_changed(child);
                }
            }
            if unsupported {
                session.unsupported();
                continue;
            }
            for old in collected {
                session.collect_renamed(&old);
            }
            if doc.is_changed() {
                *session.doc_mut(d).unwrap() = doc;
            }
        }
    }
}

/// `Strings::replace($value, '#\b' . preg_quote($old, '#') . 'b#', $new)`.
fn replace_followed_by_b(value: &str, old: &str, new: &str) -> String {
    let word = |c: u8| c.is_ascii_alphanumeric() || c == b'_';
    let pattern = format!("{old}b");
    let mut out = String::new();
    let mut at = 0;
    while let Some(found) = value[at..].find(&pattern) {
        let start = at + found;
        let first = old.as_bytes().first().copied().unwrap_or(b' ');
        let before = start.checked_sub(1).map(|p| value.as_bytes()[p]);
        // `\b` before the name: a word character on one side only
        let boundary = word(first) != before.is_some_and(word);
        out.push_str(&value[at..start]);
        if boundary {
            out.push_str(new);
        } else {
            out.push_str(&pattern);
        }
        at = start + pattern.len();
    }
    out.push_str(&value[at..]);
    out
}

/// `RenameClassRector::refactorClassLike()`'s `array_unique($class->implements)`, on the next pass of the file.
fn dedupe_implements(session: &mut Session<'_>) {
    let mut removed = Vec::new();
    for class in session.view.class_likes.iter().filter(|c| c.is_class) {
        let mut seen: Vec<String> = Vec::new();
        for (span, _) in &class.implements {
            let Some(i) = session.view.names.iter().position(|n| n.span == *span) else {
                continue;
            };
            let name = session.name_string(i);
            if seen.contains(&name) {
                removed.push(*span);
            } else {
                seen.push(name);
            }
        }
    }
    for span in removed {
        session.remove_implements(span);
    }
}
