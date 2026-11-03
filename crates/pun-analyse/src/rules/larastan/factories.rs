//! Larastan's model factories: ModelFactoryDynamicStaticMethodReturnTypeExtension (`Model::factory()` is the model's
//! factory), FactoryDynamicMethodReturnTypeExtension (whether the factory makes one model or many, which ModelFactoryType
//! carries, narrows `create()` and `make()`) and ModelFactoryMethodsClassReflectionExtension (`trashed()` and the
//! `for*()` and `has*()` relationship methods).
//!
//! Mago's types carry no ModelFactoryType, so a call that returns one records its `isSingleModel()` by the call's
//! span, and a factory method reads its receiver's.

use std::cell::RefCell;
use std::collections::HashMap;

use mago_analyzer::plugin::context::{InvocationInfo, ProviderContext};
use mago_codex::metadata::CodebaseMetadata;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::object::named::TNamedObject;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::get_specialized_template_type;
use mago_codex::ttype::union::TUnion;
use mago_span::Span;
use mago_word::word;

const MODEL: &[u8] = b"Illuminate\\Database\\Eloquent\\Model";
const FACTORY: &[u8] = b"Illuminate\\Database\\Eloquent\\Factories\\Factory";
const ELOQUENT_COLLECTION: &[u8] = b"Illuminate\\Database\\Eloquent\\Collection";
const SOFT_DELETES: &[u8] = b"Illuminate\\Database\\Eloquent\\SoftDeletes";

/// TrinaryLogic of `isSingleModel()`.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Single {
    Yes,
    No,
    Maybe,
}

thread_local! {
    /// `isSingleModel()` of each call whose type is a ModelFactoryType, by the call's span.
    static FACTORIES: RefCell<HashMap<Span, Single>> = RefCell::new(HashMap::new());
}

fn is(codebase: &CodebaseMetadata, class: &[u8], parent: &[u8]) -> bool {
    class.eq_ignore_ascii_case(parent) || codebase.is_instance_of(class, parent)
}

fn object(name: mago_word::Word) -> TUnion {
    TUnion::from_atomic(TAtomic::Object(TObject::Named(TNamedObject::new(name))))
}

/// The ModelFactoryType of class `factory`, recorded for the call.
fn factory_type(codebase: &CodebaseMetadata, invocation: &InvocationInfo<'_, '_, '_>, factory: &[u8], single: Single) -> Option<TUnion> {
    let k = codebase.get_class_like(factory)?;
    FACTORIES.with(|f| f.borrow_mut().insert(invocation.span(), single));
    Some(object(k.original_name))
}

/// `Model::factory()`.
pub(super) fn model_factory(
    context: &ProviderContext<'_, '_, '_>,
    model: &[u8],
    invocation: &InvocationInfo<'_, '_, '_>,
) -> Option<TUnion> {
    let codebase = context.codebase();
    if !codebase.is_instance_of(model, MODEL) {
        return None;
    }
    let single = match invocation.get_argument(0, &[b"count"]) {
        None => Single::Yes,
        // `int|float|numeric-string`'s isSuperTypeOf(), negated
        Some(argument) => {
            let ty = context.get_expression_type(argument)?;
            let numeric = |t: &TAtomic| matches!(t, TAtomic::Scalar(s) if s.is_numeric());
            if ty.types.iter().all(numeric) {
                Single::No
            } else if ty.types.iter().any(numeric) || ty.types.iter().any(|t| t.is_mixed()) {
                Single::Maybe
            } else {
                Single::Yes
            }
        }
    };
    let factory = factory_class(codebase, model)?;
    factory_type(codebase, invocation, factory.as_bytes(), single)
}

/// `getFactoryReflection()`: the concrete factory `newFactory()` returns, else `Factory::resolveFactoryName()`'s.
fn factory_class(codebase: &CodebaseMetadata, model: &[u8]) -> Option<mago_word::Word> {
    let k = codebase.get_class_like(model)?;
    if codebase.method_exists(model, b"newFactory") {
        let returned = super::forwards::method_type(codebase, model, &TNamedObject::new(k.original_name), b"newFactory", None);
        let classes: Vec<_> = returned
            .iter()
            .flat_map(|t| t.types.iter())
            .filter_map(|t| match t {
                TAtomic::Object(TObject::Named(n)) => Some(n.name),
                TAtomic::GenericParameter(g) => g.constraint.types.iter().find_map(|c| match c {
                    TAtomic::Object(TObject::Named(n)) => Some(n.name),
                    _ => None,
                }),
                _ => None,
            })
            .collect();
        if let [factory] = classes.as_slice()
            && is(codebase, factory.as_bytes(), FACTORY)
            && codebase.get_class_like(factory.as_bytes()).is_some_and(|f| !f.flags.is_abstract())
        {
            return Some(*factory);
        }
    }
    // `Factory::$namespace` and the app namespace `App\`
    let name = String::from_utf8_lossy(k.original_name.as_bytes()).into_owned();
    let relative = name.strip_prefix("App\\Models\\").or_else(|| name.strip_prefix("App\\")).unwrap_or(&name);
    let factory = format!("Database\\Factories\\{relative}Factory");
    codebase.class_exists(factory.as_bytes()).then(|| codebase.get_class_like(factory.as_bytes()).map(|f| f.original_name))?
}

/// The receiver's ModelFactoryType: its `isSingleModel()`.
fn receiver_single(receiver: Option<Span>) -> Option<Single> {
    FACTORIES.with(|f| f.borrow().get(&receiver?).copied())
}

/// The factory's `TModel`, or its parent's.
fn model_of(codebase: &CodebaseMetadata, factory: &TNamedObject) -> Option<TUnion> {
    let k = codebase.get_class_like(factory.name.as_bytes())?;
    let defining = codebase.get_class_like(FACTORY)?.name;
    get_specialized_template_type(codebase, word("TModel"), defining, k, factory.type_parameters.as_deref())
}

/// ModelFactoryMethodsClassReflectionExtension::hasMethod().
fn has_magic_method(codebase: &CodebaseMetadata, factory: &TNamedObject, method: &str) -> bool {
    let Some(model) = model_of(codebase, factory) else { return false };
    let model = model.types.iter().find_map(|t| match t {
        TAtomic::Object(TObject::Named(n)) => Some(n.name),
        _ => None,
    });
    let model_word = model.unwrap_or_else(|| word(MODEL));
    let model_class = model_word.as_bytes();
    let uses_soft_deletes = codebase.class_uses_trait(model_class, SOFT_DELETES)
        || codebase.get_class_like(model_class).is_some_and(|k| {
            k.all_parent_classes.iter().any(|p| codebase.class_uses_trait(p.as_bytes(), SOFT_DELETES))
        });
    if method == "trashed" && uses_soft_deletes {
        return true;
    }
    let Some(relationship) = method.strip_prefix("for").or_else(|| method.strip_prefix("has")) else {
        return false;
    };
    let relationship = crate::laravel::camel(relationship);
    model.is_some_and(|m| codebase.method_exists(m.as_bytes(), relationship.as_bytes()))
}

/// FactoryDynamicMethodReturnTypeExtension on a factory method; `receiver` is the call's receiver.
pub(super) fn factory_method(
    context: &ProviderContext<'_, '_, '_>,
    method: &[u8],
    invocation: &InvocationInfo<'_, '_, '_>,
    receiver: Option<Span>,
) -> Option<TUnion> {
    let codebase = context.codebase();
    let target = invocation.inner().target.get_method_context()?;
    let mago_codex::ttype::expander::StaticClassType::Object(TObject::Named(factory)) = &target.class_type else {
        return None;
    };
    let class = factory.name.as_bytes();
    if !is(codebase, class, FACTORY) {
        return None;
    }
    let name = String::from_utf8_lossy(method).into_owned();
    let original = if codebase.method_exists(class, method) {
        super::forwards::method_type(codebase, class, factory, method, Some((context, invocation)))?
    } else if has_magic_method(codebase, factory, &name) {
        object(codebase.get_class_like(class)?.original_name)
    } else {
        return None;
    };
    let Some(single) = receiver_single(receiver) else {
        // not a ModelFactoryType: the method's own type, which Mago lacks for a magic method
        return (!codebase.method_exists(class, method)).then_some(original);
    };
    match name.to_ascii_lowercase().as_str() {
        "count" | "times" => {
            let argument = invocation.get_argument(0, &[b"count"])?;
            let ty = context.get_expression_type(argument)?;
            let single = if ty.types.iter().all(|t| matches!(t, TAtomic::Scalar(TScalar::Integer(_)))) {
                Single::No
            } else if ty.types.iter().all(TAtomic::is_null) {
                Single::Yes
            } else {
                Single::Maybe
            };
            return factory_type(codebase, invocation, class, single);
        }
        "create" | "createquietly" | "make" if single != Single::Maybe => {
            let removed: &[u8] = if single == Single::Yes { ELOQUENT_COLLECTION } else { MODEL };
            let kept: Vec<_> = original
                .types
                .iter()
                .filter(|t| !matches!(t, TAtomic::Object(TObject::Named(n)) if is(codebase, n.name.as_bytes(), removed)))
                .cloned()
                .collect();
            return Some(TUnion::from_vec(kept));
        }
        _ => {}
    }
    // `$originalReturnType->isSuperTypeOf($calledOnType)->yes()`: the factory, still a ModelFactoryType
    let returns_factory = !original.types.is_empty()
        && original.types.iter().all(|t| matches!(t, TAtomic::Object(TObject::Named(n)) if is(codebase, class, n.name.as_bytes())));
    if returns_factory {
        return factory_type(codebase, invocation, class, single);
    }
    Some(original)
}

