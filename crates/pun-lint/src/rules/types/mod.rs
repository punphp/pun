//! The `types` group: rules ported from Rector 2.6.1's TypeDeclaration, TypeDeclarationDocblocks and CodeQuality type
//! rules (rules/...), each in its own file, and what they share.
//!
//! A rule that adds a return type writes `: type` right after the `)` of the parameters (or of a closure's `use`), where
//! php-parser's format-preserving printer inserts a new `returnType` node.

pub mod facts;
mod names;

mod add_array_return;
mod add_bool_return;
mod add_bool_return_from_const;
mod add_closure_never_return;
mod add_closure_return;
mod add_closure_void_return;
mod add_fluent_return;
mod add_function_void_return;
mod add_magic_method_types;
mod add_method_void_return;
mod add_native_scalar_return;
mod add_never_return;
mod add_return_type_from_cast;
mod add_return_type_from_constant;
mod add_return_type_from_new;
mod add_return_type_from_param;
mod add_return_type_from_property;
mod add_return_type_from_ternary;
mod add_scalar_return;
mod add_union_return;
mod declare;
mod native;
mod never;
mod silent_void;

pub use add_array_return::AddArrayReturn;
pub use add_bool_return::AddBoolReturn;
pub use add_bool_return_from_const::AddBoolReturnFromConst;
pub use add_closure_never_return::AddClosureNeverReturn;
pub use add_closure_return::{AddArrowFunctionReturn, AddClosureReturn};
pub use add_closure_void_return::AddClosureVoidReturn;
pub use add_fluent_return::AddFluentReturn;
pub use add_function_void_return::AddFunctionVoidReturn;
pub use add_magic_method_types::AddMagicMethodTypes;
pub use add_method_void_return::AddMethodVoidReturn;
pub use add_native_scalar_return::{AddNumericReturn, AddStringReturn};
pub use add_never_return::AddNeverReturn;
pub use add_return_type_from_cast::AddReturnTypeFromCast;
pub use add_return_type_from_constant::AddReturnTypeFromConstant;
pub use add_return_type_from_new::AddReturnTypeFromNew;
pub use add_return_type_from_param::AddReturnTypeFromParam;
pub use add_return_type_from_property::AddReturnTypeFromProperty;
pub use add_return_type_from_ternary::AddReturnTypeFromTernary;
pub use add_scalar_return::{AddNumericReturnFromScalar, AddStringReturnFromScalar};
pub use add_union_return::{AddNullableReturn, AddUnionReturn};

use mago_syntax::cst::{
    Block, Expression, Extends, Identifier, MethodBody, Modifier, Node, Return, Sequence, Statement,
};
use pun_problem::{Edit, Fix, Safety, Span};

use crate::syntax::node_span;
use crate::{Report, Rule};
use facts::Facts;
use names::resolve_class_name;

/// The group's ported rules, in a fixed order. `AddReturnTypeFromTernary`, `AddNullableReturn` and `AddUnionReturn` pass Rector's fixtures but
/// not the corpus yet: Rector's `TypeFactory::uniquateTypes` unwraps constant arrays into one `array<key, item>` per
/// item and hashes arrays by key and item type, so `[]` against `[$a, $b]`, or two classes, make a union it skips (or
/// writes) where Pun's grouping by kind does not; they stay out of `all()` until that is ported.
pub(crate) fn all() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(AddFunctionVoidReturn),
        Box::new(AddClosureVoidReturn),
        Box::new(AddMethodVoidReturn),
        Box::new(AddNeverReturn),
        Box::new(AddClosureNeverReturn),
        Box::new(AddReturnTypeFromCast),
        Box::new(AddMagicMethodTypes),
        Box::new(AddBoolReturnFromConst),
        Box::new(AddReturnTypeFromNew),
        Box::new(AddStringReturnFromScalar),
        Box::new(AddNumericReturnFromScalar),
        Box::new(AddBoolReturn),
        Box::new(AddArrayReturn),
        Box::new(AddStringReturn),
        Box::new(AddNumericReturn),
        Box::new(AddFluentReturn),
        Box::new(AddReturnTypeFromParam),
        Box::new(AddReturnTypeFromProperty),
        Box::new(AddReturnTypeFromConstant),
        Box::new(AddClosureReturn),
        Box::new(AddArrowFunctionReturn),
    ]
}

/// php-parser's `ClassMethod::$magicNames`.
pub(crate) const MAGIC: [&str; 17] = [
    "__construct",
    "__destruct",
    "__call",
    "__callstatic",
    "__get",
    "__set",
    "__isset",
    "__unset",
    "__sleep",
    "__wakeup",
    "__tostring",
    "__set_state",
    "__clone",
    "__invoke",
    "__debuginfo",
    "__serialize",
    "__unserialize",
];

/// `PhpVersionFeature::SCALAR_TYPES`.
pub(crate) const SCALAR_TYPES: u32 = 70000;

/// `PhpVersionFeature::VOID_TYPE`.
pub(crate) const VOID_TYPE: u32 = 70100;

/// A report at `span` whose fix inserts `: type` at `at`.
pub(crate) fn add_return_type(span: Span, at: usize, r#type: &str) -> Report {
    Report {
        span,
        message: format!("Add the `{type}` return type.", type = r#type),
        help: None,
        fix: Some(Fix {
            safety: Safety::Unsafe,
            edits: vec![Edit::new(
                Span::new(at, at),
                format!(": {type}", type = r#type),
            )],
        }),
    }
}

/// `BetterNodeFinder::findFirstInFunctionLikeScoped`: whether `found` holds for a node in `node`'s body, without
/// looking into nested functions, closures, arrow functions and classes (an anonymous class's arguments are looked
/// into, as php-parser keeps them on the `new`).
pub(crate) fn any_scoped<'ast, 'arena>(
    node: Node<'ast, 'arena>,
    found: &mut dyn FnMut(Node<'ast, 'arena>) -> bool,
) -> bool {
    let mut hit = false;
    node.visit_children(|child| {
        if hit {
            return;
        }
        if found(child) {
            hit = true;
            return;
        }
        match child {
            Node::Function(_)
            | Node::Closure(_)
            | Node::ArrowFunction(_)
            | Node::Method(_)
            | Node::Class(_)
            | Node::Interface(_)
            | Node::Trait(_)
            | Node::Enum(_) => {}
            Node::AnonymousClass(class) => {
                if let Some(args) = &class.argument_list {
                    hit = any_scoped(Node::PartialArgumentList(args), found);
                }
            }
            _ => hit = any_scoped(child, found),
        }
    });
    hit
}

/// A function or method without a return type that Rector's return-type rules look at (`ClassMethod|Function_`).
pub(crate) struct Untyped<'ast, 'arena> {
    /// Where the report points: the name.
    pub span: Span,
    /// Where the return type goes.
    pub at: usize,
    pub body: &'ast Block<'arena>,
}

/// `node` as an [`Untyped`] function or method that `ClassMethodReturnTypeOverrideGuard` lets a return type be added
/// to.
pub(crate) fn untyped<'ast, 'arena>(
    node: Node<'ast, 'arena>,
    ancestors: &[Node<'ast, 'arena>],
    ctx: &crate::Context<'_>,
) -> Option<Untyped<'ast, 'arena>> {
    match node {
        Node::Function(f) if f.return_type_hint.is_none() => Some(Untyped {
            span: node_span(&f.name),
            at: node_span(&f.parameter_list).end,
            body: &f.body,
        }),
        Node::Method(m) if m.return_type_hint.is_none() => {
            let MethodBody::Concrete(body) = &m.body else {
                return None;
            };
            let scope = class_scope(ancestors);
            if never::should_skip_class_method(&facts::facts(ctx), scope.as_ref(), m) {
                return None;
            }
            Some(Untyped {
                span: node_span(&m.name),
                at: node_span(&m.parameter_list).end,
                body,
            })
        }
        _ => None,
    }
}

/// `BetterNodeFinder::findReturnsScoped`: the `return`s of a body outside nested functions and classes, or none when
/// it yields.
pub(crate) fn returns_scoped<'ast, 'arena>(body: &'ast Block<'arena>) -> Vec<&'ast Return<'arena>> {
    let mut returns = Vec::new();
    let yields = any_scoped(Node::Block(body), &mut |n| match n {
        Node::Yield(_) => true,
        Node::Return(r) => {
            returns.push(r);
            false
        }
        _ => false,
    });
    if yields {
        returns.clear();
    }
    returns
}

/// `ReturnAnalyzer::hasOnlyReturnWithExpr`: the body returns, every `return` has a value, and its end cannot be
/// reached.
pub(crate) fn has_only_return_with_expr(
    place: &silent_void::Place<'_, '_, '_>,
    body: &Block<'_>,
    returns: &[&Return<'_>],
) -> bool {
    !returns.is_empty()
        && returns.iter().all(|r| r.value.is_some())
        && !silent_void::has_silent_void(place, body.statements.iter())
}

/// `SilentVoidResolver::hasExclusiveVoid` for a body: no `return` with a value and no `yield`, outside nested
/// functions and classes.
pub(crate) fn has_exclusive_void(body: &Block<'_>) -> bool {
    !any_scoped(Node::Block(body), &mut |n| match n {
        Node::Return(r) => r.value.is_some(),
        Node::Yield(_) => true,
        _ => false,
    })
}

/// The statements of a block as php-parser counts them: a comment after the last statement (or in an empty block)
/// is one more, a `Nop`.
pub(crate) fn statement_count(source: &str, block: &Block<'_>) -> usize {
    let statements: Vec<&Statement<'_>> = block.statements.iter().collect();
    let from = statements
        .last()
        .map_or(block.left_brace.end.offset as usize, |s| node_span(*s).end);
    let rest = &source[from..block.right_brace.start.offset as usize];
    statements.len() + usize::from(!rest.trim().is_empty())
}

/// What Rector's `ReflectionResolver::resolveClassReflection` and `ClassModifierChecker` tell about the class-like a
/// node is in, read from its declaration.
pub(crate) struct ClassScope {
    /// The fully qualified name; `None` for an anonymous class.
    pub name: Option<String>,
    pub is_final: bool,
    pub is_abstract: bool,
    pub is_interface: bool,
    /// The classes and interfaces it extends and implements, fully qualified.
    pub parents: Vec<String>,
    /// The class it extends, fully qualified.
    pub parent_class: Option<String>,
}

/// The class-like the innermost `ancestors` member belongs to.
pub(crate) fn class_scope(ancestors: &[Node<'_, '_>]) -> Option<ClassScope> {
    let at = ancestors.iter().rposition(|a| {
        matches!(
            a,
            Node::Class(_)
                | Node::AnonymousClass(_)
                | Node::Interface(_)
                | Node::Trait(_)
                | Node::Enum(_)
        )
    })?;
    let before = &ancestors[..at];
    let named = |name: &[u8]| {
        let id = String::from_utf8_lossy(name);
        Some(match namespace_of(before) {
            Some(ns) => format!("{ns}\\{id}"),
            None => id.into_owned(),
        })
    };
    let resolve = |ids: Vec<&Identifier<'_>>| {
        ids.into_iter()
            .map(|id| resolve_class_name(before, id))
            .collect::<Vec<_>>()
    };
    let has = |modifiers: &Sequence<'_, Modifier<'_>>, f: fn(&Modifier<'_>) -> bool| {
        modifiers.iter().any(f)
    };
    let extended = |extends: Option<&Extends<'_>>| {
        extends
            .and_then(|e| e.types.first())
            .map(|id| resolve_class_name(before, id))
    };
    Some(match ancestors[at] {
        Node::Class(c) => ClassScope {
            name: named(c.name.value),
            parent_class: extended(c.extends.as_ref()),
            is_final: has(&c.modifiers, |m| matches!(m, Modifier::Final(_))),
            is_abstract: has(&c.modifiers, |m| matches!(m, Modifier::Abstract(_))),
            is_interface: false,
            parents: resolve(
                c.extends
                    .iter()
                    .flat_map(|e| e.types.iter())
                    .chain(c.implements.iter().flat_map(|i| i.types.iter()))
                    .collect(),
            ),
        },
        Node::AnonymousClass(c) => ClassScope {
            name: None,
            parent_class: extended(c.extends.as_ref()),
            is_final: true,
            is_abstract: false,
            is_interface: false,
            parents: resolve(
                c.extends
                    .iter()
                    .flat_map(|e| e.types.iter())
                    .chain(c.implements.iter().flat_map(|i| i.types.iter()))
                    .collect(),
            ),
        },
        Node::Interface(i) => ClassScope {
            name: named(i.name.value),
            is_final: false,
            is_abstract: true,
            is_interface: true,
            parent_class: None,
            parents: resolve(i.extends.iter().flat_map(|e| e.types.iter()).collect()),
        },
        Node::Trait(t) => ClassScope {
            name: named(t.name.value),
            is_final: false,
            is_abstract: false,
            is_interface: false,
            parent_class: None,
            parents: Vec::new(),
        },
        Node::Enum(e) => ClassScope {
            name: named(e.name.value),
            is_final: true,
            is_abstract: false,
            is_interface: false,
            parent_class: None,
            parents: resolve(e.implements.iter().flat_map(|i| i.types.iter()).collect()),
        },
        _ => unreachable!("matched above"),
    })
}

/// The namespace the node `ancestors` lead to is declared in.
pub(crate) fn namespace_of<'a>(ancestors: &[Node<'a, 'a>]) -> Option<String> {
    ancestors.iter().rev().find_map(|a| match a {
        Node::Namespace(ns) => ns
            .name
            .as_ref()
            .map(|n| String::from_utf8_lossy(n.value()).into_owned()),
        _ => None,
    })
}

/// The ancestors of a class (`ClassReflection::getAncestors()` without the class itself): its interfaces, traits and
/// parent classes, from the codebase, or from its declaration when the codebase has no class of that name (an
/// anonymous class).
pub(crate) fn ancestor_names(facts: &Facts, scope: &ClassScope) -> Vec<String> {
    let codebase = &facts.codebase;
    if let Some(class) = scope
        .name
        .as_deref()
        .and_then(|n| codebase.get_class_like(n.as_bytes()))
    {
        let words = class
            .all_parent_interfaces
            .iter()
            .chain(class.used_traits.iter())
            .chain(class.all_parent_classes.iter());
        return words
            .map(|w| String::from_utf8_lossy(w.as_bytes()).into_owned())
            .collect();
    }
    let mut out = Vec::new();
    for parent in &scope.parents {
        out.push(parent.clone());
        if let Some(class) = codebase.get_class_like(parent.as_bytes()) {
            out.extend(
                class
                    .all_parent_interfaces
                    .iter()
                    .chain(class.used_traits.iter())
                    .chain(class.all_parent_classes.iter())
                    .map(|w| String::from_utf8_lossy(w.as_bytes()).into_owned()),
            );
        }
    }
    out
}

/// `ClassMethodReturnVendorLockResolver::isVendorLocked` once the magic-method and private checks are done: the first
/// ancestor with the method decides, locked when it declares a native return type other than `mixed`.
pub(crate) fn is_return_vendor_locked(facts: &Facts, scope: &ClassScope, method: &[u8]) -> bool {
    for ancestor in ancestor_names(facts, scope) {
        let Some(m) = facts.codebase.get_method(ancestor.as_bytes(), method) else {
            continue;
        };
        return m
            .return_type_declaration_metadata
            .as_ref()
            .is_some_and(|t| !t.type_union.is_mixed());
    }
    false
}

/// Whether an expression statement throws (`throw ...;`).
pub(crate) fn is_throw_statement(statement: &Statement<'_>) -> bool {
    matches!(statement, Statement::Expression(e) if matches!(e.expression, Expression::Throw(_)))
}
