//! `lint/dead-code/no-unused-return-type`, ported from Rector 2.8.4's `NarrowWideUnionReturnTypeRector`
//! (rules/DeadCode/Rector/FunctionLike), from PHP 8.0 (union types), with what it calls:
//! `StaticTypeMapper::mapPHPStanTypeToPhpParserNode($type, TypeKind::RETURN)` of the narrowed union
//! (`UnionTypeMapper::mapToPhpParserNode()`), PHPStan's `TypeCombinator::remove()` (with `UnionType::tryRemove()`
//! and `BooleanType::tryRemove()`, the `tryRemove()`s of the types a native union declares), and
//! `ComposerJsonPackageVersionResolver::hasPackageMultiMajorVersions()` with the part of composer/semver's
//! `VersionParser::parseConstraints()` that gives a constraint's lowest major version.
//!
//! A function, closure, arrow function, or a method that cannot be overridden (private, final, or in a final class),
//! declared to return a union (or `?T`), drops the members no `return` can return (the native type of each returned
//! value, `null` for a bare `return` or a reachable end). A `@return` loses them too. A ternary whose condition reads a
//! constant of a vendor package that `composer.json` allows in more than one major version counts both branches.

use std::path::{Path, PathBuf};

use mago_syntax::cst::{
    Block, Expression, FunctionLikeReturnTypeHint, Hint, Modifier, Node, NodeKind,
};
use pun_phpstan::types::{Tri, Type, sort_types, to_phpdoc};
use pun_problem::{Edit, Fix, Safety};

use super::phpdoc_writer::type_mapper::map_to_phpdoc;
use super::phpdoc_writer::{DocTarget, TypeNode};
use super::types::compare::classes::with_classes;
use super::types::compare::mapper::{TypeScope, expr_type};
use super::types::compare::{Object, RType, are_types_equal, create_union, equals, is_super};
use super::types::facts::{Facts, facts, path_of};
use super::types::names::resolve_class_name;
use super::types::silent_void::{Place, has_silent_void};
use super::types::{any_scoped, class_scope, returns_scoped};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

/// `PhpVersionFeature::UNION_TYPES`.
const UNION_TYPES: u32 = 80000;
/// `PhpVersionFeature::INTERSECTION_TYPES`.
const INTERSECTION_TYPES: u32 = 80100;
/// `PhpVersionFeature::UNION_INTERSECTION_TYPES`.
const DNF_TYPES: u32 = 80200;

/// The rule, reading `composer.json` from `composer_json` (Rector's `ComposerJsonPackageVersionResolver` service),
/// or from the working directory when it is `None`, as Rector does.
#[derive(Default)]
pub struct NoUnusedReturnType {
    pub composer_json: Option<PathBuf>,
}

/// A function-like node as the rule sees it.
struct FunctionLike<'ast, 'arena> {
    hint: &'ast FunctionLikeReturnTypeHint<'arena>,
    body: Body<'ast, 'arena>,
    /// php-parser's start of the node, for its doc comment.
    start: usize,
    span: pun_problem::Span,
}

enum Body<'ast, 'arena> {
    Block(&'ast Block<'arena>),
    /// An arrow function's expression (`getStmts()` gives one `return` of it).
    Arrow(&'ast Expression<'arena>),
}

impl Rule for NoUnusedReturnType {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-unused-return-type"
    }

    fn min_php_version(&self) -> u32 {
        UNION_TYPES
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[
            NodeKind::Method,
            NodeKind::Function,
            NodeKind::Closure,
            NodeKind::ArrowFunction,
        ])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Some(f) = function_like(node, ancestors) else {
            return;
        };
        if should_skip_hint(&f.hint.hint) {
            return;
        }
        let target = DocTarget::of(ctx, f.start);
        let info = target.doc.map(|_| target.info(ctx.source));
        let facts = facts(ctx);
        let mut chain = ancestors.to_vec();
        chain.push(node);
        let scope = TypeScope::new(&chain, facts.documented());
        let doc_return = info
            .as_ref()
            .and_then(|i| i.return_tag())
            .and_then(|tag| tag.parsed.as_ref())
            .map(|parsed| with_classes(facts.documented(), || scope.map_doc(parsed)));
        // shouldSkipByDocblock: a `@return` union that `createMixedPassedOrUnionType()` would change
        if let Some(doc) = &doc_return
            && let Some(members) = doc.union_members()
            && !with_classes(facts.documented(), || {
                equals(&create_union(members, false), doc)
            })
        {
            return;
        }
        let returns: Vec<Option<&Expression<'_>>> = match f.body {
            Body::Block(block) => returns_scoped(block).into_iter().map(|r| r.value).collect(),
            Body::Arrow(expr) => {
                if yields(expr) {
                    vec![]
                } else {
                    vec![Some(expr)]
                }
            }
        };
        if returns.is_empty() {
            return;
        }
        let class = class_scope(ancestors);
        let place = Place {
            facts: &facts,
            ancestors,
            scope: class.as_ref(),
        };
        let implicit_null = match f.body {
            Body::Block(block) => has_silent_void(&place, block.statements.iter()),
            Body::Arrow(_) => false,
        } || returns.iter().any(Option::is_none);
        let return_type = with_classes(facts.documented(), || scope.map_hint(&f.hint.hint));
        let mut actual: Vec<Type> = Vec::new();
        for value in returns.into_iter().flatten() {
            actual.extend(self.native_return_types(&facts, ancestors, value));
        }
        if implicit_null {
            actual.push(Type::Null);
        }
        let unused: Vec<RType> = with_classes(facts.documented(), || {
            let members = return_type
                .union_members()
                .unwrap_or_else(|| vec![return_type.clone()]);
            members
                .into_iter()
                .filter(|member| {
                    actual
                        .iter()
                        .all(|a| is_super(member, &RType::T(a.clone())).no())
                })
                .collect()
        });
        if unused.is_empty() {
            return;
        }
        let narrowed = with_classes(facts.documented(), || remove_all(&return_type, &unused));
        let Some(declaration) = declaration(&narrowed, ctx.php_version) else {
            return;
        };
        // mostly placeholder
        if declaration == "null" {
            return;
        }
        let mut edits = vec![Edit::new(node_span(&f.hint.hint), declaration.clone())];
        // changePhpDocReturnType()
        if let Some(mut info) = info
            && let Some(doc) = doc_return
        {
            let new = with_classes(facts.documented(), || remove_all(&doc, &unused));
            let changed = with_classes(facts.documented(), || !are_types_equal(&doc, &new));
            let legal = new
                .union_members()
                .is_none_or(|members| !members.iter().any(RType::is_mixed));
            if changed && legal {
                info.set_return_type(phpdoc_node(&new));
                edits.push(target.edit(ctx.source, &info.print()));
            }
        }
        out.push(Report {
            span: f.span,
            message: format!(
                "Narrow the return type to `{declaration}`, what the returns can return."
            ),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits,
            }),
        });
    }
}

impl NoUnusedReturnType {
    /// `resolveNativeReturnTypes()`: the native type of `expr`; both branches' of a ternary whose condition fetches
    /// a constant of a class from a package `composer.json` allows in several major versions.
    fn native_return_types(
        &self,
        facts: &Facts,
        ancestors: &[Node<'_, '_>],
        expr: &Expression<'_>,
    ) -> Vec<Type> {
        if let Expression::Conditional(c) = unparenthesize(expr)
            && self.has_vendor_class_const_fetch(facts, ancestors, c.condition)
        {
            let mut types =
                self.native_return_types(facts, ancestors, c.then.unwrap_or(c.condition));
            types.extend(self.native_return_types(facts, ancestors, c.r#else));
            return types;
        }
        vec![expr_type(facts, ancestors, expr, true)]
    }

    /// `hasVendorClassConstFetch()`: a `Name::CONSTANT` in `expr` whose class is declared in a file of a package
    /// with several major versions.
    fn has_vendor_class_const_fetch(
        &self,
        facts: &Facts,
        ancestors: &[Node<'_, '_>],
        expr: &Expression<'_>,
    ) -> bool {
        let mut found = false;
        let mut check = |node: Node<'_, '_>| {
            if let Node::ClassConstantAccess(access) = node
                && let Expression::Identifier(id) = access.class
                && let Some(class) = facts
                    .codebase
                    .get_class_like(resolve_class_name(ancestors, id).as_bytes())
                && let Some(path) = path_of(class.span.file_id)
                && self.has_package_multi_major_versions(&path)
            {
                found = true;
            }
        };
        check(Node::Expression(expr));
        walk(Node::Expression(expr), &mut check);
        found
    }

    /// `ComposerJsonPackageVersionResolver::hasPackageMultiMajorVersions()`: the file is in `vendor/<vendor>/<name>/`
    /// and `composer.json` requires that package with constraints of more than one lowest major version.
    fn has_package_multi_major_versions(&self, path: &str) -> bool {
        let path = path.replace('\\', "/");
        let Some(at) = path.find("/vendor/") else {
            return false;
        };
        let parts: Vec<&str> = path[at + "/vendor/".len()..].split('/').collect();
        if parts.len() < 2 {
            return false;
        }
        let package = format!("{}/{}", parts[0], parts[1]);
        let file = self
            .composer_json
            .as_deref()
            .unwrap_or(Path::new("composer.json"));
        let Some(constraint) = required_constraint(file, &package) else {
            return false;
        };
        has_multi_major_versions(&constraint)
    }
}

/// The version constraint `composer.json` at `file` requires `package` with (`require`, then `require-dev`).
fn required_constraint(file: &Path, package: &str) -> Option<String> {
    let json: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(file).ok()?).ok()?;
    let mut found = None;
    for key in ["require", "require-dev"] {
        if let Some(serde_json::Value::String(c)) = json.get(key).and_then(|r| r.get(package)) {
            found = Some(c.clone());
        }
    }
    found
}

/// `resolveHasPackageMultiMajorVersions()`: the `|` or `||` alternatives of `constraint` with a number have more than
/// one lowest major version.
fn has_multi_major_versions(constraint: &str) -> bool {
    let parts: Vec<&str> = constraint
        .split('|')
        .map(str::trim)
        .filter(|p| !p.is_empty())
        .collect();
    if parts.len() < 2 {
        return false;
    }
    let mut majors: Vec<u64> = Vec::new();
    for part in parts {
        if !part.bytes().any(|b| b.is_ascii_digit()) {
            continue;
        }
        if let Some(major) = lower_major(part)
            && !majors.contains(&major)
        {
            majors.push(major);
        }
    }
    majors.len() > 1
}

/// The major version of `getLowerBound()` of one alternative of a constraint (`VersionParser::parseConstraints()`):
/// a hyphen range's left side; else the highest lower bound of its space- or comma-separated constraints, `0` for
/// `<`, `<=` and `!=`. `None` where composer/semver throws or the bound has no major version (`dev-main`).
fn lower_major(part: &str) -> Option<u64> {
    let part = part.split('@').next().unwrap_or(part).trim();
    if let Some((from, _)) = part.split_once(" - ") {
        return version_major(from.trim());
    }
    let mut major = None;
    for atom in part.split([' ', ',']).filter(|a| !a.is_empty()) {
        let bound = if atom.starts_with('<') || atom.starts_with("!=") {
            0
        } else {
            let version = atom.trim_start_matches(['>', '=', '^', '~']);
            match version {
                "*" | "x" | "X" => 0,
                _ => version_major(version)?,
            }
        };
        major = Some(major.map_or(bound, |m: u64| m.max(bound)));
    }
    major
}

/// The major number of a version (`v2.1`, `3.*`, `4`).
fn version_major(version: &str) -> Option<u64> {
    let version = version.strip_prefix(['v', 'V']).unwrap_or(version);
    let digits: String = version.chars().take_while(char::is_ascii_digit).collect();
    if digits.is_empty()
        || !version[digits.len()..]
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || ".-*xX+".contains(c))
    {
        return None;
    }
    digits.parse().ok()
}

/// Every node under `node`, depth first (`NodeFinder::findInstanceOf`).
fn walk<'ast, 'arena>(node: Node<'ast, 'arena>, f: &mut dyn FnMut(Node<'ast, 'arena>)) {
    node.visit_children(|child| {
        f(child);
        walk(child, f);
    });
}

/// Whether an arrow function's expression yields, outside nested functions and classes.
fn yields(expr: &Expression<'_>) -> bool {
    matches!(unparenthesize(expr), Expression::Yield(_))
        || any_scoped(Node::Expression(expr), &mut |n| matches!(n, Node::Yield(_)))
}

/// `node` as a function-like with a union return type the rule may narrow (`shouldSkipNode()` past the hint): a
/// method that is private or final, or whose class (not an interface, trait or enum) is final.
fn function_like<'ast, 'arena>(
    node: Node<'ast, 'arena>,
    ancestors: &[Node<'ast, 'arena>],
) -> Option<FunctionLike<'ast, 'arena>> {
    let start = node_span(&node).start;
    match node {
        Node::Function(f) => Some(FunctionLike {
            hint: f.return_type_hint.as_ref()?,
            body: Body::Block(&f.body),
            start,
            span: node_span(&f.name),
        }),
        Node::Closure(c) => Some(FunctionLike {
            hint: c.return_type_hint.as_ref()?,
            body: Body::Block(&c.body),
            start,
            span: node_span(&c.function),
        }),
        Node::ArrowFunction(a) => Some(FunctionLike {
            hint: a.return_type_hint.as_ref()?,
            body: Body::Arrow(a.expression),
            start,
            span: node_span(&a.r#fn),
        }),
        Node::Method(m) => {
            let hint = m.return_type_hint.as_ref()?;
            if !matches!(hint.hint, Hint::Union(_) | Hint::Nullable(_)) {
                return None;
            }
            let has = |f: fn(&Modifier<'_>) -> bool| m.modifiers.iter().any(f);
            let open = !has(|m| matches!(m, Modifier::Private(_) | Modifier::Final(_)));
            let mago_syntax::cst::MethodBody::Concrete(body) = &m.body else {
                return None;
            };
            if open {
                // the class reflection: a final class (an anonymous class is final), not an interface, trait or enum
                let class = ancestors.iter().rev().find(|a| {
                    matches!(
                        a,
                        Node::Class(_)
                            | Node::AnonymousClass(_)
                            | Node::Interface(_)
                            | Node::Trait(_)
                            | Node::Enum(_)
                    )
                })?;
                let final_class = match class {
                    Node::Class(c) => c.modifiers.iter().any(|m| matches!(m, Modifier::Final(_))),
                    Node::AnonymousClass(_) => true,
                    _ => false,
                };
                if !final_class {
                    return None;
                }
            }
            Some(FunctionLike {
                hint,
                body: Body::Block(body),
                start,
                span: node_span(&m.name),
            })
        }
        _ => None,
    }
}

/// `shouldSkipNode()` of the hint: not a union or `?T`, or a union with `true` or `false`.
fn should_skip_hint(hint: &Hint<'_>) -> bool {
    match hint {
        Hint::Nullable(n) => is_true_or_false(n.hint),
        Hint::Union(_) => {
            let mut members = Vec::new();
            flatten(hint, &mut members);
            members.into_iter().any(is_true_or_false)
        }
        _ => true,
    }
}

fn is_true_or_false(hint: &Hint<'_>) -> bool {
    matches!(hint, Hint::True(_) | Hint::False(_))
}

fn flatten<'a, 'arena>(hint: &'a Hint<'arena>, out: &mut Vec<&'a Hint<'arena>>) {
    match hint {
        Hint::Union(u) => {
            flatten(u.left, out);
            flatten(u.right, out);
        }
        other => out.push(other),
    }
}

/// `TypeCombinator::remove($from, TypeCombinator::union(...$unused))`: each unused type removed in turn.
fn remove_all(from: &RType, unused: &[RType]) -> RType {
    unused.iter().fold(from.clone(), |from, u| remove(&from, u))
}

/// `TypeCombinator::remove()` of one type: nothing left when it takes in the whole type, the type as it is when it
/// takes in none of it, else `tryRemove()`: a union's members each, `bool` without one of its values.
fn remove(from: &RType, removed: &RType) -> RType {
    match is_super(removed, from) {
        Tri::Yes => return RType::T(Type::Never(pun_phpstan::types::NeverKind::Implicit)),
        Tri::No => return from.clone(),
        Tri::Maybe => {}
    }
    if let Some(members) = from.union_members() {
        let mut kept: Vec<RType> = members
            .iter()
            .map(|m| remove(m, removed))
            .filter(|m| !matches!(m, RType::T(Type::Never(_))))
            .collect();
        // the members left, in their order (a declared union has no member another takes in); some is left, else
        // `removed` would take in the whole union
        return match kept.len() {
            1 => kept.remove(0),
            _ => RType::Union(kept),
        };
    }
    // (`BooleanType::tryRemove()` takes a constant boolean, which a declared union with `true` or `false` has none of)
    from.clone()
}

/// `mapPHPStanTypeToPhpParserNode($type, TypeKind::RETURN)` of the narrowed declared type, printed: a union's members
/// each mapped (`UnionTypeMapper`), repeats dropped, `?T` for one with `null`; `None` where Rector maps none.
fn declaration(t: &RType, php_version: u32) -> Option<String> {
    let Some(members) = t.union_members() else {
        return member_name(t, false, php_version);
    };
    let mut names: Vec<String> = Vec::new();
    for member in &members {
        let name = member_name(member, true, php_version)?;
        if !names.contains(&name) {
            names.push(name);
        }
    }
    match &names[..] {
        [a, b]
            if a != b && (a == "null" || b == "null") && !a.contains('&') && !b.contains('&') =>
        {
            let other = if a == "null" { b } else { a };
            Some(format!("?{other}"))
        }
        // (`resolveUnionTypeNode()`'s guard against `object` with a class never fires: the members are some of a
        // declared union's, and PHP rejects a union with both as redundant)
        many => Some(
            many.iter()
                .map(|n| {
                    if n.contains('&') {
                        format!("({n})")
                    } else {
                        n.clone()
                    }
                })
                .collect::<Vec<_>>()
                .join("|"),
        ),
    }
}

/// The node `PHPStanStaticTypeMapper::mapToPhpParserNode()` gives one type of a declared one, printed; `in_union`
/// for `TypeKind::UNION`. A declared union holds no `true`, `false` or `mixed` (the rule skips the first two, and PHP
/// allows `mixed` alone only), so its other types are keywords.
fn member_name(t: &RType, in_union: bool, php_version: u32) -> Option<String> {
    Some(match t {
        RType::Object(o) => match o {
            // (a declared name is never `NonExisting`, which only a PHPDoc name maps to, and `parent` of a class
            // without one does not compile)
            Object::FullyQualified(n) | Object::NonExisting(n) => {
                format!("\\{}", n.trim_start_matches('\\'))
            }
            Object::Shortened { full, .. } | Object::Aliased { full, .. } => format!("\\{full}"),
            Object::SelfStatic(_) | Object::SelfObject(_) => "self".to_string(),
            Object::ParentStatic(_) | Object::ParentWithoutClass => "parent".to_string(),
        },
        // `IntersectionTypeMapper`: from PHP 8.1, in a union (a DNF type) from PHP 8.2
        RType::Intersection(types) => {
            if php_version < INTERSECTION_TYPES || (in_union && php_version < DNF_TYPES) {
                return None;
            }
            types
                .iter()
                .map(|t| member_name(t, false, php_version))
                .collect::<Option<Vec<_>>>()?
                .join("&")
        }
        // `StaticTypeMapper`: `self` in a union
        RType::T(Type::Static(..)) if in_union => "self".to_string(),
        other => {
            let printed = to_phpdoc(&other.to_sub());
            KEYWORDS.contains(&printed.as_str()).then_some(printed)?
        }
    })
}

/// The types a declared union holds that php-parser writes as `Identifier`s.
const KEYWORDS: [&str; 10] = [
    "int", "float", "string", "bool", "null", "array", "iterable", "callable", "object", "static",
];

/// `mapPHPStanTypeToPHPStanPhpDocTypeNode()` of a type Rector's mappers built (`ObjectTypeMapper`): a shortened or
/// aliased class prints as written, a fully qualified one with its `\`.
fn phpdoc_node(t: &RType) -> TypeNode {
    match t {
        RType::Object(Object::Shortened { short, .. }) => TypeNode::Identifier(short.clone()),
        RType::Object(Object::Aliased { alias, .. }) => TypeNode::Identifier(alias.clone()),
        RType::Union(types) => {
            // `UnionType::toPhpDocNode()`: the members in `sortTypes()`'s order, a shortened or aliased class by the
            // name it holds
            let keys: Vec<Type> = (types.iter())
                .map(|t| match t {
                    RType::Object(Object::Shortened { short: name, .. } | Object::Aliased { alias: name, .. }) => {
                        Type::Object(name.clone(), Vec::new(), false)
                    }
                    other => other.to_sub(),
                })
                .collect();
            let mut used = vec![false; keys.len()];
            let mut sorted = Vec::new();
            for key in sort_types(&keys) {
                if let Some(i) = (0..keys.len()).find(|&i| !used[i] && keys[i] == key) {
                    used[i] = true;
                    sorted.push(phpdoc_node(&types[i]));
                }
            }
            sorted.extend((0..keys.len()).filter(|&i| !used[i]).map(|i| phpdoc_node(&types[i])));
            TypeNode::Union {
                types: sorted,
                wrapped: false,
            }
        }
        other => map_to_phpdoc(&other.to_sub()),
    }
}
