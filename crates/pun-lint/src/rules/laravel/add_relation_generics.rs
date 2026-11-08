//! `lint/laravel/add-relation-generics`, ported from rector-laravel 2.7.3's `AddGenericReturnTypeToRelationsRector`
//! (src/Rector/ClassMethod).
//!
//! A method with a body, in a subclass of `Illuminate\Database\Eloquent\Model` or in a trait, whose return type is a
//! class name of an `Illuminate\Database\Eloquent\Relations\Relation`, and whose first `return` (outside nested
//! functions and classes) holds a relation call (`$x->hasMany(Foo::class)`, the first method call in it followed down
//! its chain), gets `@return \Relation<\Foo, ...>`: the related model, the intermediate model of a `*Through()` relation, `$this`,
//! and `\Illuminate\Database\Eloquent\Relations\Pivot` for a `BelongsToMany`. Since 2.6 the rule writes these
//! generics always; Rector runs it only with laravel/framework 12.3 or later installed, which Pun leaves to the rule's
//! set. The older generic forms are `AddLegacyGenericReturnTypeToRelationsRector`'s and
//! `AddNewGenericReturnTypeToRelationsRector`'s. An existing `@return` stays
//! when its type is not the return type (generics aside), or already has the generics the rule would write.
//!
//! A `@return` type without generics resolves as Rector's `ObjectTypeSpecifier` does it ([`resolve_doc_class_name`]);
//! a generic type and its arguments go to PHPStan's `TypeNodeResolver` ([`resolve_doc_string_name`]). Pun leaves the
//! method alone where it does not read the `@return` tag as phpdoc-parser does: a value over several lines, or a type
//! that is not a name, maybe with generics that are names or `$this`.

use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::reference::TReference;
use mago_syntax::cst::{
    Argument, Call, ClassLikeMemberSelector, Expression, Hint, MethodBody, Node, NodeKind,
};
use pun_problem::Safety;

use super::add_factory_extends_phpdoc::tag_name;
use super::names::{resolve_class_name, resolve_doc_class_name, resolve_doc_string_name};
use super::object_type::class_string_class;
use super::{is_a, member_name};
use crate::rules::class_method::qualified;
use crate::rules::lint_b_doc::Doc;
use crate::rules::phpunit::annotations::Annotated;
use crate::rules::types::facts::facts;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

const MODEL: &str = "Illuminate\\Database\\Eloquent\\Model";
const RELATION: &str = "Illuminate\\Database\\Eloquent\\Relations\\Relation";
const BELONGS_TO_MANY: &str = "Illuminate\\Database\\Eloquent\\Relations\\BelongsToMany";
const MORPH_TO_MANY: &str = "Illuminate\\Database\\Eloquent\\Relations\\MorphToMany";
const PIVOT: &str = "Illuminate\\Database\\Eloquent\\Relations\\Pivot";

/// `RELATION_METHODS`.
const RELATION_METHODS: &[&str] = &[
    "hasOne",
    "hasOneThrough",
    "morphOne",
    "belongsTo",
    "morphTo",
    "hasMany",
    "hasManyThrough",
    "morphMany",
    "belongsToMany",
    "morphToMany",
    "morphedByMany",
];

/// `RELATION_WITH_INTERMEDIATE_METHODS`.
const INTERMEDIATE_METHODS: &[&str] = &["hasManyThrough", "hasOneThrough"];

pub struct AddRelationGenerics;

impl Rule for AddRelationGenerics {
    fn id(&self) -> &'static str {
        "lint/laravel/add-relation-generics"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Method])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Method(method) = node else { return };
        let MethodBody::Concrete(body) = &method.body else {
            return;
        };
        // `shouldSkipNode`: a trait, or a class (not anonymous) that is a subclass of `Model`
        let class_like = ancestors.iter().rev().find(|a| {
            matches!(
                a,
                Node::Class(_)
                    | Node::AnonymousClass(_)
                    | Node::Interface(_)
                    | Node::Trait(_)
                    | Node::Enum(_)
            )
        });
        let (local, is_trait) = match class_like {
            Some(Node::Class(c)) => (c.name.value, false),
            Some(Node::Trait(t)) => (t.name.value, true),
            _ => return,
        };
        let own = qualified(ancestors, local);
        if !is_trait && (own.eq_ignore_ascii_case(MODEL) || !is_a(ctx, &own, &[MODEL])) {
            return;
        }
        let Some(Hint::Identifier(hint)) = method.return_type_hint.as_ref().map(|h| &h.hint) else {
            return;
        };
        let native = resolve_class_name(ancestors, hint);
        if !is_a(ctx, &native, &[RELATION]) {
            return;
        }
        let annotated = Annotated::of(ctx, node, ancestors).unwrap();
        let Ok(tag) = return_tag(annotated.doc.as_ref()) else {
            return;
        };
        let facts = facts(ctx);
        let exists = |name: &str| facts.codebase.get_class_like(name.as_bytes()).is_some();
        // `IdentifierPhpDocTypeMapper`: `self`, `static` and `parent` are the class's and its parent's types, which
        // compare by their class's name, as `$this` does
        let parent = || {
            let class = facts.codebase.get_class_like(own.as_bytes())?;
            let parent = facts
                .codebase
                .get_class_like(class.direct_parent_class.as_ref()?.as_bytes())?;
            Some(parent.original_name.to_string())
        };
        let doc_type = |written: &str, generic: bool| match written {
            "$this" => DocType::This(own.clone()),
            _ if written.eq_ignore_ascii_case("self") || written.eq_ignore_ascii_case("static") => {
                DocType::Class(own.clone())
            }
            _ if written.eq_ignore_ascii_case("parent") => {
                parent().map_or(DocType::Other, DocType::Class)
            }
            _ if generic => DocType::Class(resolve_doc_string_name(ancestors, written)),
            _ => DocType::Class(resolve_doc_class_name(ancestors, written, exists)),
        };
        // `areNativeTypeAndPhpDocReturnTypeEqual`
        if tag.as_ref().is_some_and(
            |t| !matches!(doc_type(t.base, t.args.is_some()), DocType::Class(c) if c == native),
        ) {
            return;
        }
        let Some(call) = relation_call(body) else {
            return;
        };
        let method_name = member_name(&call.method)
            .map(String::from_utf8_lossy)
            .unwrap_or_default();
        let args: Vec<&Expression<'_>> = (call.argument_list.arguments.iter())
            .map(Argument::value)
            .collect();
        let class_of = |e: &Expression<'_>| {
            facts
                .full_type(node_span(e))
                .and_then(|t| class_string_class(ctx, t))
        };
        let Some(related) = class_of(args[0]) else {
            return;
        };
        // `shouldUseNewGenerics()` and `shouldUsePivotGeneric()` are true
        let intermediate = INTERMEDIATE_METHODS
            .contains(&&*method_name)
            .then(|| args.get(1).and_then(|a| class_of(a)))
            .flatten();
        // `areGenericTypesEqual`
        if let Some(types) = tag.as_ref().and_then(|t| t.args.as_ref()) {
            let pivots = is_a(ctx, &native, &[BELONGS_TO_MANY, MORPH_TO_MANY]);
            let mut types: Vec<DocType> = types.iter().map(|t| doc_type(t, true)).collect();
            // `TypeNodeResolver::resolveGenericTypeNode`: a generic class's missing template types take their defaults
            let given = types.len();
            let templates = (facts.documented().get_class_like(native.as_bytes()))
                .into_iter()
                .flat_map(|c| c.template_types.values().skip(given));
            for default in templates.filter_map(|t| t.default.as_ref()) {
                types.push(default_type(&facts, default));
            }
            let equal = |t: &DocType, class: &str| matches!(t, DocType::Class(c) | DocType::This(c) if c == class);
            let subtype = |t: &DocType, class: &str| matches!(t, DocType::Class(c) | DocType::This(c) if is_a(ctx, c, &[class]));
            let done = if equal(&types[0], &related)
                && types.len() > 1
                && matches!(types[1], DocType::This(_))
                && !pivots
            {
                true
            } else {
                match &intermediate {
                    None if pivots => {
                        types.len() >= 3
                            && equal(&types[0], &related)
                            && matches!(types[1], DocType::This(_))
                            && subtype(&types[2], PIVOT)
                    }
                    None => false,
                    Some(intermediate) => equal(&types[1], intermediate),
                }
            };
            if done {
                return;
            }
        }
        let pivot = is_a(ctx, &native, &[BELONGS_TO_MANY]);
        self.report(
            ctx,
            method,
            &annotated,
            tag.as_ref(),
            &native,
            &related,
            intermediate,
            pivot,
            out,
        );
    }
}

impl AddRelationGenerics {
    /// The fix: the `@return` tag with `getGenericTypes()`'s generics, in place of the tag's type or added.
    #[allow(clippy::too_many_arguments)]
    fn report(
        &self,
        ctx: &Context<'_>,
        method: &mago_syntax::cst::Method<'_>,
        annotated: &Annotated<'_>,
        tag: Option<&ReturnTag<'_>>,
        native: &str,
        related: &str,
        intermediate: Option<String>,
        pivot: bool,
        out: &mut Vec<Report>,
    ) {
        let mut generics = vec![format!("\\{related}")];
        generics.extend(intermediate.map(|i| format!("\\{i}")));
        generics.push("$this".to_string());
        if pivot {
            generics.push(format!("\\{PIVOT}"));
        }
        let ty = format!("\\{native}<{}>", generics.join(", "));
        let new_doc = match (annotated.doc.as_ref(), tag) {
            (Some(doc), Some(tag)) => {
                let text = doc.text_all();
                let child = doc.text(tag.index).trim_end();
                let from = child.as_ptr() as usize - text.as_ptr() as usize;
                let value = format!("{ty} {}", tag.description);
                format!(
                    "{}@return {}{}",
                    &text[..from],
                    value.trim(),
                    &text[from + child.len()..]
                )
            }
            (Some(doc), None) => {
                doc.print_appending(&vec![false; doc.len()], &[&format!("@return {ty}")])
            }
            (None, _) => format!("/**\n * @return {ty}\n */"),
        };
        // The tag printed as it is: nothing changes (a `BelongsToMany` from 11.15.0 to 12.3.0 gets generics the rule
        // does not count as done).
        if annotated
            .doc
            .as_ref()
            .is_some_and(|d| d.text_all() == new_doc)
        {
            return;
        }
        let Some(edits) = annotated.edits(ctx.source, Some(&new_doc), &[]) else {
            return;
        };
        out.push(Report::fixed(
            node_span(&method.name),
            "Add the relation's generics to its `@return` type.",
            Safety::Unsafe,
            edits,
        ));
    }
}

/// A template's default as a [`DocType`]: one object is its class (a class the codebase lacks stays a reference to
/// its name, which PHPStan compares as that class), anything else is not compared.
fn default_type(
    facts: &crate::rules::types::facts::Facts,
    t: &mago_codex::ttype::union::TUnion,
) -> DocType {
    match &t.types[..] {
        [TAtomic::Object(TObject::Named(n))] => DocType::Class(
            (facts.codebase.get_class_like(n.name.as_bytes()))
                .map_or(n.name.to_string(), |c| c.original_name.to_string()),
        ),
        [TAtomic::Reference(TReference::Symbol { name, .. })] => DocType::Class(name.to_string()),
        _ => DocType::Other,
    }
}

/// A type in the doc, as far as the rule compares it.
enum DocType {
    /// A class's type (`ObjectType`, `SelfObjectType`, `StaticType`, `ParentStaticType`), by the name it compares by.
    Class(String),
    /// `$this` (`ThisType`) in the class it names.
    This(String),
    /// `parent` in a class without one (`MixedType`).
    Other,
}

/// The first `@return` tag of a doc comment as far as Pun reads it: its index among the doc's children, its type's
/// name and generic arguments as written, and its description.
struct ReturnTag<'s> {
    index: usize,
    base: &'s str,
    args: Option<Vec<&'s str>>,
    description: &'s str,
}

/// `PhpDocInfo::getReturnTagValue()`: the first `@return` tag of `doc`. `Err(())` where Pun does not read it as
/// phpdoc-parser does.
fn return_tag<'s>(doc: Option<&Doc<'s>>) -> Result<Option<ReturnTag<'s>>, ()> {
    let Some(doc) = doc else { return Ok(None) };
    // a tag without a value is invalid (`InvalidTagValueNode`), and does not count
    let Some(index) = (0..doc.len()).find(|&i| {
        let words = doc.words(i);
        words.len() > 1 && tag_name(words[0]) == "@return"
    }) else {
        return Ok(None);
    };
    let text = doc.text(index);
    let value = text["@return".len()..].trim_start_matches([' ', '\t']);
    if text.contains('\n') || value.len() == text.len() - "@return".len() {
        return Err(());
    }
    let name_end = |s: &str| {
        s.find(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || c == '\\' || !c.is_ascii()))
            .unwrap_or(s.len())
    };
    let base_len = name_end(value);
    let (base, rest) = value.split_at(base_len);
    if base.is_empty() {
        return Err(());
    }
    let (args, rest) = match rest.strip_prefix('<') {
        Some(inner) => {
            let mut depth = 0usize;
            let close = inner
                .char_indices()
                .find(|&(_, c)| {
                    match c {
                        '<' | '{' | '(' | '[' => depth += 1,
                        '>' | '}' | ')' | ']' if depth > 0 => depth -= 1,
                        '>' => return true,
                        _ => {}
                    }
                    false
                })
                .ok_or(())?
                .0;
            let args: Vec<&str> = inner[..close].split(',').map(str::trim).collect();
            let simple = |a: &str| a == "$this" || (!a.is_empty() && name_end(a) == a.len());
            if !args.iter().all(|a| simple(a)) {
                return Err(());
            }
            (Some(args), &inner[close + 1..])
        }
        None => (None, rest),
    };
    if !rest.is_empty() && !rest.starts_with([' ', '\t']) {
        return Err(());
    }
    Ok(Some(ReturnTag {
        index,
        base,
        args,
        description: rest.trim_matches([' ', '\t']),
    }))
}

/// `getRelationMethodCall`: in the first `return` of the method (outside nested functions and classes), the first
/// method call, followed down the method calls it is called on; a relation method with an argument.
fn relation_call<'a>(
    body: &'a mago_syntax::cst::Block<'a>,
) -> Option<&'a mago_syntax::cst::MethodCall<'a>> {
    fn first_return<'a>(node: Node<'a, 'a>) -> Option<&'a mago_syntax::cst::Return<'a>> {
        if let Node::Return(r) = node {
            return Some(r);
        }
        if matches!(
            node,
            Node::Closure(_)
                | Node::ArrowFunction(_)
                | Node::Function(_)
                | Node::Class(_)
                | Node::AnonymousClass(_)
                | Node::Interface(_)
                | Node::Trait(_)
                | Node::Enum(_)
        ) {
            return None;
        }
        let mut found = None;
        node.visit_children(|c| {
            if found.is_none() {
                found = first_return(c);
            }
        });
        found
    }
    /// The first method call in pre-order; `Err` at a first-class callable method call, which php-parser reads as
    /// a method call that Pun does not follow.
    fn first_call<'a>(
        node: Node<'a, 'a>,
    ) -> Result<Option<&'a mago_syntax::cst::MethodCall<'a>>, ()> {
        match node {
            Node::MethodCall(c) => return Ok(Some(c)),
            Node::MethodPartialApplication(_) => return Err(()),
            _ => {}
        }
        let mut found = Ok(None);
        node.visit_children(|c| {
            if matches!(found, Ok(None)) {
                found = first_call(c);
            }
        });
        found
    }
    let mut ret = None;
    for statement in body.statements.iter() {
        ret = first_return(Node::Statement(statement));
        if ret.is_some() {
            break;
        }
    }
    let mut call = first_call(Node::Expression(ret?.value?)).ok()??;
    while let Expression::Call(Call::Method(inner)) = call.object {
        call = inner;
    }
    let ClassLikeMemberSelector::Identifier(name) = &call.method else {
        return None;
    };
    let name = String::from_utf8_lossy(name.value);
    (RELATION_METHODS.contains(&&*name) && !call.argument_list.arguments.is_empty()).then_some(call)
}
