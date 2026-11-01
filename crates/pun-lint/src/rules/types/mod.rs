//! The `types` group: rules ported from Rector 2.6.1's TypeDeclaration, TypeDeclarationDocblocks and CodeQuality type
//! rules (rules/...), each in its own file, and what they share.
//!
//! A rule that adds a return type writes `: type` right after the `)` of the parameters (or of a closure's `use`), where
//! php-parser's format-preserving printer inserts a new `returnType` node.

pub(crate) mod compare;
pub mod facts;
pub(crate) mod names;

mod add_array_return;
mod add_return_phpdoc_direct_array;
mod add_getter_return_phpdoc;
mod add_param_phpdoc_from_callable_call;
mod add_param_phpdoc_from_dim_fetch;
mod add_param_phpdoc_from_local_calls;
mod add_param_phpdoc_from_reference_assign;
mod add_return_phpdoc_common_object;
mod add_return_phpdoc_dim_assigned_object;
mod add_return_phpdoc_json_array;
mod add_return_phpdoc_scalar_array;
mod add_var_phpdoc_from_constructor_param;
mod add_var_phpdoc_from_default;
mod add_var_phpdoc_from_dim_fetch;
mod add_var_phpdoc_from_getter;
mod prefer_generic_collection_phpdoc;
mod add_bool_return;
mod add_bool_return_from_const;
mod add_closure_never_return;
mod add_closure_return;
mod implicit_array_observer;
mod add_return_type_from_try_catch;
mod add_closure_void_return;
mod add_fluent_return;
mod add_function_void_return;
mod add_magic_method_types;
mod add_method_void_return;
mod add_native_scalar_return;
mod add_param_type_array_any_all;
mod add_param_type_array_dim_fetch;
mod add_param_type_array_function;
mod add_param_type_array_map;
mod add_param_type_array_reduce;
mod add_param_type_arrow_dim_fetch;
mod add_param_type_data_provider;
mod add_param_type_dim_fetch_key;
mod add_param_type_iterable_method_call;
mod add_param_type_method_call;
mod add_param_type_parent_call;
mod add_param_type_property;
mod add_param_type_spl_fixed_array;
mod add_param_type_variable_call;
mod add_never_return;
mod add_property_type_assigns;
mod add_property_type_behat_context;
mod add_property_type_constructor;
mod add_property_type_create_mock;
mod add_property_type_jms;
mod add_property_type_setter_getter;
mod add_property_type_setup;
mod add_property_type_setup_docblock;
mod add_return_type_from_cast;
mod add_return_type_from_constant;
mod add_return_type_from_get_repository;
mod add_return_type_from_native_call;
mod add_return_type_from_new;
mod add_return_type_from_new_array;
mod add_return_type_from_param;
mod add_return_type_from_parent;
mod add_return_type_from_property;
mod add_return_type_from_symfony_serializer;
mod add_return_type_from_ternary;
mod add_return_type_from_typed_call;
mod add_return_type_from_yield;
mod add_scalar_return;
mod add_union_return;
pub(crate) mod declare;
mod doctrine_repository;
pub(crate) mod docblock;
mod native;
mod union_method;
mod never;
pub(crate) mod no_nullsafe_on_non_null;
mod prefer_identical_same_type;
mod prefer_instanceof;
mod prefer_specific_return_type;
pub(crate) mod property_type;
pub(crate) mod rector_type;
mod param;
pub(crate) mod silent_void;
mod type_factory;

pub use add_array_return::AddArrayReturn;
pub use add_return_phpdoc_direct_array::AddReturnPhpdocDirectArray;
pub use add_getter_return_phpdoc::AddGetterReturnPhpdoc;
pub use add_param_phpdoc_from_callable_call::AddParamPhpdocFromCallableCall;
pub use add_param_phpdoc_from_dim_fetch::AddParamPhpdocFromDimFetch;
pub use add_param_phpdoc_from_local_calls::AddParamPhpdocFromLocalCalls;
pub use add_param_phpdoc_from_reference_assign::AddParamPhpdocFromReferenceAssign;
pub use add_return_phpdoc_common_object::AddReturnPhpdocCommonObject;
pub use add_return_phpdoc_dim_assigned_object::AddReturnPhpdocDimAssignedObject;
pub use add_return_phpdoc_json_array::AddReturnPhpdocJsonArray;
pub use add_return_phpdoc_scalar_array::AddReturnPhpdocScalarArray;
pub use add_var_phpdoc_from_constructor_param::AddVarPhpdocFromConstructorParam;
pub use add_var_phpdoc_from_default::AddVarPhpdocFromDefault;
pub use add_var_phpdoc_from_dim_fetch::AddVarPhpdocFromDimFetch;
pub use add_var_phpdoc_from_getter::AddVarPhpdocFromGetter;
pub use prefer_generic_collection_phpdoc::PreferGenericCollectionPhpdoc;
pub use add_bool_return::AddBoolReturn;
pub use add_bool_return_from_const::AddBoolReturnFromConst;
pub use add_closure_never_return::AddClosureNeverReturn;
pub use add_closure_return::{AddArrowFunctionReturn, AddClosureReturn};
pub(crate) use add_closure_return::closure_return;
pub use add_return_type_from_try_catch::AddReturnTypeFromTryCatch;
pub use add_closure_void_return::AddClosureVoidReturn;
pub use add_fluent_return::AddFluentReturn;
pub use add_function_void_return::AddFunctionVoidReturn;
pub use add_magic_method_types::AddMagicMethodTypes;
pub use add_method_void_return::AddMethodVoidReturn;
pub use add_native_scalar_return::{AddNumericReturn, AddStringReturn};
pub use add_never_return::AddNeverReturn;
pub use add_property_type_assigns::AddPropertyTypeAssigns;
pub use add_property_type_behat_context::AddPropertyTypeBehatContext;
pub use add_property_type_constructor::AddPropertyTypeConstructor;
pub use add_property_type_create_mock::AddPropertyTypeCreateMock;
pub use add_property_type_jms::{AddPropertyTypeJmsObject, AddPropertyTypeJmsScalar};
pub use add_property_type_setter_getter::AddPropertyTypeSetterGetter;
pub use add_property_type_setup::AddPropertyTypeSetup;
pub use add_property_type_setup_docblock::AddPropertyTypeSetupDocblock;
pub use add_param_type_array_any_all::{AddParamTypeArrayAnyAll, PreferNarrowParamType};
pub use add_param_type_array_dim_fetch::AddParamTypeArrayDimFetch;
pub use add_param_type_array_function::AddParamTypeArrayFunction;
pub use add_param_type_array_map::AddParamTypeArrayMap;
pub use add_param_type_array_reduce::AddParamTypeArrayReduce;
pub use add_param_type_arrow_dim_fetch::AddParamTypeArrowDimFetch;
pub use add_param_type_data_provider::AddParamTypeDataProvider;
pub use add_param_type_dim_fetch_key::AddParamTypeDimFetchKey;
pub use add_param_type_iterable_method_call::AddParamTypeIterableMethodCall;
pub use add_param_type_method_call::AddParamTypeMethodCall;
pub use add_param_type_parent_call::AddParamTypeParentCall;
pub use add_param_type_property::AddParamTypeProperty;
pub use add_param_type_spl_fixed_array::AddParamTypeSplFixedArray;
pub use add_param_type_variable_call::AddParamTypeVariableCall;
pub use add_return_type_from_cast::AddReturnTypeFromCast;
pub use add_return_type_from_constant::AddReturnTypeFromConstant;
pub use add_return_type_from_get_repository::AddReturnTypeFromGetRepository;
pub use add_return_type_from_native_call::AddReturnTypeFromNativeCall;
pub use add_return_type_from_new::AddReturnTypeFromNew;
pub use add_return_type_from_new_array::{AddPrivateMethodArrayReturnType, AddReturnTypeFromNewArray};
pub use add_return_type_from_param::AddReturnTypeFromParam;
pub use add_return_type_from_parent::AddReturnTypeFromParent;
pub use add_return_type_from_property::AddReturnTypeFromProperty;
pub use add_return_type_from_symfony_serializer::AddReturnTypeFromSymfonySerializer;
pub use add_return_type_from_ternary::AddReturnTypeFromTernary;
pub use add_return_type_from_typed_call::AddReturnTypeFromTypedCall;
pub use add_return_type_from_yield::AddReturnTypeFromYield;
pub use add_scalar_return::{AddNumericReturnFromScalar, AddStringReturnFromScalar};
pub use add_union_return::{AddNullableReturn, AddUnionReturn};
pub use doctrine_repository::{
    AddDoctrineRepositoryReturnType, AddPropertyTypeContainerGet, AddPropertyTypeGetRepository,
};
pub(crate) use never::has_parent_class_method;
pub use no_nullsafe_on_non_null::NoNullsafeOnNonNull;
pub use prefer_identical_same_type::PreferIdenticalSameType;
pub use prefer_instanceof::PreferInstanceof;
pub use prefer_specific_return_type::PreferSpecificReturnType;

use mago_syntax::cst::{
    Block, Expression, Extends, Identifier, MethodBody, Modifier, Node, Return, Sequence, Statement,
};
use pun_problem::{Edit, Safety, Span};

use crate::syntax::node_span;
use crate::{Report, Rule};
use facts::Facts;
use names::resolve_class_name;

/// The group's ported rules, in a fixed order.
pub(crate) fn all() -> Vec<Box<dyn Rule>> {
    // Rector's order (src/Config/Level/TypeDeclarationLevel.php): on one function the first rule that adds a return
    // type wins. `AddReturnTypeDeclarationBasedOnParentClassMethodRector` visits the class, before its methods.
    vec![
        Box::new(AddReturnTypeFromParent),
        Box::new(AddClosureVoidReturn),
        Box::new(AddFunctionVoidReturn),
        Box::new(AddArrowFunctionReturn),
        Box::new(AddBoolReturnFromConst),
        Box::new(AddPrivateMethodArrayReturnType),
        Box::new(AddReturnTypeFromNewArray),
        Box::new(AddReturnTypeFromConstant),
        Box::new(AddStringReturnFromScalar),
        Box::new(AddNumericReturnFromScalar),
        Box::new(AddBoolReturn),
        Box::new(AddStringReturn),
        Box::new(AddNumericReturn),
        Box::new(AddReturnTypeFromTernary),
        Box::new(AddArrayReturn),
        Box::new(AddReturnTypeFromNew),
        Box::new(AddReturnTypeFromCast),
        Box::new(AddReturnTypeFromSymfonySerializer),
        Box::new(AddMethodVoidReturn),
        Box::new(AddReturnTypeFromProperty),
        Box::new(AddNullableReturn),
        Box::new(AddReturnTypeFromYield),
        Box::new(AddReturnTypeFromNativeCall),
        Box::new(AddReturnTypeFromTryCatch),
        Box::new(AddReturnTypeFromTypedCall),
        Box::new(AddMagicMethodTypes),
        Box::new(PreferSpecificReturnType),
        Box::new(AddUnionReturn),
        Box::new(AddClosureNeverReturn),
        Box::new(AddClosureReturn),
        Box::new(AddReturnTypeFromParam),
        Box::new(AddFluentReturn),
        Box::new(AddNeverReturn),
        Box::new(PreferInstanceof),
        Box::new(PreferIdenticalSameType),
        Box::new(NoNullsafeOnNonNull),
        Box::new(AddPropertyTypeConstructor),
        Box::new(AddPropertyTypeAssigns::default()),
        Box::new(AddPropertyTypeSetterGetter),
        Box::new(AddPropertyTypeJmsScalar),
        Box::new(AddPropertyTypeJmsObject),
        Box::new(AddPropertyTypeBehatContext),
        Box::new(AddPropertyTypeContainerGet),
        Box::new(AddPropertyTypeGetRepository),
        Box::new(AddPropertyTypeCreateMock),
        Box::new(AddPropertyTypeSetup),
        Box::new(AddPropertyTypeSetupDocblock),
        Box::new(AddDoctrineRepositoryReturnType),
        Box::new(AddReturnTypeFromGetRepository),
        // the docblock rules, in Rector's order (src/Config/Level/TypeDeclarationDocblocksLevel.php)
        Box::new(AddVarPhpdocFromDefault),
        Box::new(AddParamPhpdocFromDimFetch),
        Box::new(AddParamPhpdocFromLocalCalls),
        Box::new(AddParamPhpdocFromReferenceAssign),
        Box::new(AddParamPhpdocFromCallableCall),
        Box::new(AddReturnPhpdocCommonObject),
        Box::new(AddReturnPhpdocScalarArray),
        Box::new(AddReturnPhpdocDirectArray),
        Box::new(AddReturnPhpdocDimAssignedObject),
        Box::new(AddReturnPhpdocJsonArray),
        Box::new(AddVarPhpdocFromConstructorParam),
        Box::new(AddVarPhpdocFromGetter),
        Box::new(AddVarPhpdocFromDimFetch),
        Box::new(AddGetterReturnPhpdoc),
        Box::new(PreferGenericCollectionPhpdoc),
        // The parameter rules, in Rector's order too.
        Box::new(AddParamTypeSplFixedArray),
        Box::new(AddParamTypeDataProvider),
        Box::new(AddParamTypeMethodCall::default()),
        Box::new(AddParamTypeParentCall),
        Box::new(AddParamTypeArrayMap),
        Box::new(AddParamTypeArrayReduce),
        Box::new(AddParamTypeArrowDimFetch),
        Box::new(AddParamTypeProperty),
        Box::new(AddParamTypeVariableCall),
        Box::new(AddParamTypeArrayDimFetch),
        Box::new(AddParamTypeDimFetchKey),
        Box::new(AddParamTypeArrayFunction),
        Box::new(AddParamTypeIterableMethodCall),
        Box::new(PreferNarrowParamType),
        Box::new(AddParamTypeArrayAnyAll),
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
    Report::fixed(
        span,
        format!("Add the `{type}` return type.", type = r#type),
        Safety::Unsafe,
        vec![Edit::new(
            Span::new(at, at),
            format!(": {type}", type = r#type),
        )],
    )
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
