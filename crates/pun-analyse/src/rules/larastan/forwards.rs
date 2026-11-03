//! Larastan's call forwarding for the rules' inference. ModelForwardsCallsExtension: a method a model does not declare
//! is its `Builder<static>`'s (`Model::__call()` and `__callStatic()` forward to a new query), with the builder's
//! `TModel` the model and its `static` the builder. EloquentBuilderForwardsCallsExtension: a query builder method an
//! Eloquent builder reaches through its `@mixin` returns the Eloquent builder, unless it is passed through.
//! Mago resolves the mixin itself; these providers only replace the return types.

use mago_analyzer::plugin::context::{InvocationInfo, ProviderContext};
use mago_analyzer::plugin::provider::function::{FunctionReturnTypeProvider, FunctionTarget};
use mago_analyzer::plugin::provider::method::{MethodReturnTypeProvider, MethodTarget};
use mago_analyzer::plugin::provider::{Provider, ProviderMeta};
use mago_codex::metadata::CodebaseMetadata;
use mago_codex::misc::GenericParent;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::object::named::TNamedObject;
use mago_codex::ttype::expander::{StaticClassType, TypeExpansionOptions, expand_union};
use mago_codex::ttype::get_specialized_template_type;
use mago_codex::ttype::template::TemplateResult;
use mago_codex::ttype::template::inferred_type_replacer;
use mago_codex::ttype::comparator::{ComparisonResult, union_comparator};
use mago_codex::ttype::union::TUnion;

const MODEL: &[u8] = b"Illuminate\\Database\\Eloquent\\Model";
const BUILDER: &[u8] = b"Illuminate\\Database\\Eloquent\\Builder";
const QUERY_BUILDER: &[u8] = b"Illuminate\\Database\\Query\\Builder";
const FACADE: &[u8] = b"Illuminate\\Support\\Facades\\Facade";
const PROXY: &[u8] = b"Illuminate\\Support\\HigherOrderCollectionProxy";
const RELATION: &[u8] = b"Illuminate\\Database\\Eloquent\\Relations\\Relation";
const TAP_PROXY: &[u8] = b"Illuminate\\Support\\HigherOrderTapProxy";
const SCOPE: &[u8] = b"Illuminate\\Database\\Eloquent\\Attributes\\Scope";
const USE_ELOQUENT_BUILDER: &[u8] = b"Illuminate\\Database\\Eloquent\\Attributes\\UseEloquentBuilder";
const SOFT_DELETES: &[u8] = b"Illuminate\\Database\\Eloquent\\SoftDeletes";

/// `BuilderHelper::$passthru` (with `getCountForPagination`, Laravel 12.15 and later): the query builder's results.
const PASSTHRU: &[&str] = &[
    "aggregate",
    "average",
    "avg",
    "count",
    "dd",
    "dump",
    "doesntexist",
    "doesntexistor",
    "exists",
    "existsor",
    "explain",
    "getbindings",
    "getconnection",
    "getgrammar",
    "getrawbindings",
    "implode",
    "insert",
    "insertgetid",
    "insertorignore",
    "insertorignoreusing",
    "insertusing",
    "max",
    "min",
    "numericaggregate",
    "raw",
    "rawvalue",
    "sum",
    "tosql",
    "torawsql",
    "dumprawsql",
    "ddrawsql",
    "getcountforpagination",
];

static META: ProviderMeta =
    ProviderMeta::new("pun-larastan", "pun-larastan", "Larastan's call forwarding");

/// Helpers\TapExtension: `tap($value)` is a `HigherOrderTapProxy` of the value, `tap($value, $callback)` the value.
pub(crate) struct Tap;

impl Provider for Tap {
    fn meta() -> &'static ProviderMeta {
        &META
    }
}

impl FunctionReturnTypeProvider for Tap {
    fn targets() -> FunctionTarget {
        FunctionTarget::Exact(b"tap")
    }

    fn get_return_type(
        &self,
        context: &ProviderContext<'_, '_, '_>,
        invocation: &InvocationInfo<'_, '_, '_>,
    ) -> Option<TUnion> {
        let value = || invocation.get_argument(0, &[b"value"]).and_then(|a| context.get_expression_type(a));
        match invocation.argument_count() {
            1 => {
                // a `$this` value is its static object
                let value = TUnion::from_vec(value()?.types.iter().cloned().map(|t| match t {
                    TAtomic::Object(TObject::Named(n)) if n.is_this => {
                        TAtomic::Object(TObject::Named(TNamedObject { is_this: false, ..n }.with_is_static(true)))
                    }
                    t => t,
                }).collect());
                let proxy = context.codebase().get_class_like(TAP_PROXY)?.original_name;
                Some(TUnion::from_atomic(TAtomic::Object(TObject::Named(TNamedObject::new_with_type_parameters(
                    proxy,
                    Some(vec![value]),
                )))))
            }
            2 => value().cloned(),
            _ => Some(mago_codex::ttype::get_never()),
        }
    }
}

/// The analysed file's contents locate a static call's class (`self`, `static` or a name).
pub(crate) struct ModelForwards(pub(crate) Vec<u8>);

impl Provider for ModelForwards {
    fn meta() -> &'static ProviderMeta {
        &META
    }
}

impl MethodReturnTypeProvider for ModelForwards {
    fn targets() -> &'static [MethodTarget] {
        // the registry matches a class pattern by name alone, and the called class is any model
        static TARGETS: [MethodTarget; 1] = [MethodTarget::any_class(b"*")];
        &TARGETS
    }

    fn get_return_type(
        &self,
        context: &ProviderContext<'_, '_, '_>,
        class: &[u8],
        method: &[u8],
        invocation: &InvocationInfo<'_, '_, '_>,
    ) -> Option<TUnion> {
        let codebase = context.codebase();
        if class.eq_ignore_ascii_case(QUERY_BUILDER) {
            let target = invocation.inner().target.get_method_context()?;
            let receiver = match &target.class_type {
                StaticClassType::Object(TObject::Named(receiver))
                    if target.declaring_object_type.is_some()
                        && codebase.is_instance_of(receiver.name.as_bytes(), BUILDER) =>
                {
                    receiver
                }
                _ => return None,
            };
            // a passed-through method's own return type, whose `$this` PHPStan binds to the Eloquent builder
            if is_passthru(method) {
                return method_type(codebase, QUERY_BUILDER, receiver, method, Some((context, invocation)));
            }
            return Some(builder_result(codebase, receiver));
        }
        // the proxy's `@mixin TValue` names the value's class as the method's
        let on_proxy = invocation.inner().target.get_method_context().is_some_and(|t| {
            matches!(&t.class_type, StaticClassType::Object(TObject::Named(n)) if n.name.as_bytes().eq_ignore_ascii_case(PROXY))
        });
        if class.eq_ignore_ascii_case(PROXY) || on_proxy {
            return higher_order_proxy(context, method, invocation);
        }
        // HigherOrderTapProxyExtension: a method of `HigherOrderTapProxy<T>` returns the tapped object
        if let Some(target) = invocation.inner().target.get_method_context()
            && let StaticClassType::Object(TObject::Named(n)) = &target.class_type
            && n.name.as_bytes().eq_ignore_ascii_case(TAP_PROXY)
        {
            return match n.type_parameters.as_deref() {
                Some([tapped]) if !object_classes(tapped).is_empty() => Some(tapped.clone()),
                _ => Some(mago_codex::ttype::get_mixed()),
            };
        }
        if !codebase.method_exists(class, method)
            && let Some(ty) = forward_builder_or_relation(context, method, invocation)
                .or_else(|| facade_mixin(context, class, method, invocation, 0))
        {
            return Some(ty);
        }
        if codebase.method_exists(class, method)
            && !hidden_scope(context, class, method)
            && let Some(ty) = self
                .new_query(context, class, method, invocation)
                .or_else(|| own_collection(context, method, invocation))
        {
            return Some(ty);
        }
        if !codebase.is_instance_of(class, MODEL)
            || codebase.method_exists(class, method) && !hidden_scope(context, class, method)
        {
            return None;
        }
        model_forward(context, class, method, invocation)
    }
}

impl ModelForwards {
    /// ModelDynamicStaticMethodReturnTypeExtension on a static call of a model's native method that returns an
    /// Eloquent builder (`query()`, `with()`), and NewModelQueryDynamicMethodReturnTypeExtension (`newQuery()` and
    /// the like): the model's own builder of the model the call names.
    fn new_query(
        &self,
        context: &ProviderContext<'_, '_, '_>,
        class: &[u8],
        method: &[u8],
        invocation: &InvocationInfo<'_, '_, '_>,
    ) -> Option<TUnion> {
        let codebase = context.codebase();
        let target = invocation.inner().target.get_method_context()?;
        let name = String::from_utf8_lossy(method).to_ascii_lowercase();
        let new_query = matches!(
            name.as_str(),
            "newquery"
                | "newmodelquery"
                | "newquerywithoutrelationships"
                | "newquerywithoutscopes"
                | "newquerywithoutscope"
                | "newqueryforrestoration"
        );
        let objects: Vec<TNamedObject> = match &target.class_type {
            StaticClassType::Object(TObject::Named(n)) => std::iter::once(n.clone())
                .chain(n.intersection_types.iter().flat_map(|b| b.iter()).filter_map(|t| match t {
                    TAtomic::Object(TObject::Named(i)) => Some(i.clone()),
                    _ => None,
                }))
                .map(|mut n| {
                    n.intersection_types = None;
                    n
                })
                .collect(),
            StaticClassType::Exact(n) | StaticClassType::Name(n) => vec![TNamedObject::new(*n)],
            _ => return None,
        };
        let models = || objects.iter().filter(|o| codebase.is_instance_of(o.name.as_bytes(), MODEL));
        let static_class = self.static_class(invocation, method);
        if static_class.is_none() {
            if !new_query || !codebase.is_instance_of(class, MODEL) {
                return None;
            }
            let types: Vec<TAtomic> = models()
                .map(|m| {
                    let name = codebase.get_class_like(m.name.as_bytes()).map_or(m.name, |k| k.original_name);
                    TAtomic::Object(TObject::Named(builder_of(codebase, &TNamedObject::new(name))))
                })
                .collect();
            return (!types.is_empty()).then(|| TUnion::from_vec(types));
        }
        if !codebase.is_instance_of(class, MODEL)
            || !codebase.method_exists(MODEL, method)
            || name.starts_with("find")
            || name == "__construct"
        {
            return None;
        }
        let returned = &codebase.get_declaring_method(MODEL, method)?.return_type_metadata.as_ref()?.type_union;
        if !references(returned, BUILDER) {
            return None;
        }
        // `$scope->resolveTypeByName()` of the call's class: `static` keeps the late static binding, `self`, a name
        // or an expression's object type is the class itself
        let is_static = static_class.is_some_and(|c| c.eq_ignore_ascii_case(b"static"));
        let types: Vec<TAtomic> = models()
            .map(|m| {
                let name = codebase.get_class_like(m.name.as_bytes()).map_or(m.name, |k| k.original_name);
                let model = if is_static {
                    TNamedObject { is_this: false, ..m.clone() }.with_is_static(true)
                } else {
                    TNamedObject::new(name)
                };
                TAtomic::Object(TObject::Named(builder_of(codebase, &model)))
            })
            .collect();
        (!types.is_empty()).then(|| TUnion::from_vec(types))
    }

    /// A static call's class expression (`static`, `self`, a name or an expression), found in the source before
    /// `::` and the method's name; `None` for an instance call.
    fn static_class(&self, invocation: &InvocationInfo<'_, '_, '_>, method: &[u8]) -> Option<&[u8]> {
        use mago_span::HasSpan;
        let call = invocation.span();
        let arguments = invocation.inner().arguments_source.span();
        let before = self.0.get(call.start.offset as usize..arguments.start.offset as usize)?.trim_ascii_end();
        let at = before.len().checked_sub(method.len()).filter(|&at| before[at..].eq_ignore_ascii_case(method))?;
        Some(before[..at].trim_ascii_end().strip_suffix(b"::")?.trim_ascii())
    }
}

/// Whether the passthru list names the method (`BuilderHelper::$passthru`).
fn is_passthru(method: &[u8]) -> bool {
    PASSTHRU.contains(&String::from_utf8_lossy(method).to_ascii_lowercase().as_str())
}

/// Whether a type names the class (`getReferencedClasses()`), in its type arguments too.
fn references(ty: &TUnion, class: &[u8]) -> bool {
    ty.types.iter().any(|t| match t {
        TAtomic::Object(TObject::Named(n)) => {
            n.name.as_bytes().eq_ignore_ascii_case(class) || n.type_parameters.iter().flatten().any(|p| references(p, class))
        }
        _ => false,
    })
}

/// A model's `#[Scope]` method that is not public, which a call outside the model's classes does not reach
/// (ClassReflection::getMethod() asks the extensions when the scope cannot call the native method).
fn hidden_scope(context: &ProviderContext<'_, '_, '_>, class: &[u8], method: &[u8]) -> bool {
    let codebase = context.codebase();
    let Some(m) = codebase.get_declaring_method(class, method) else { return false };
    let hidden = m.method_metadata.as_ref().is_some_and(|mm| !mm.visibility.is_public())
        && m.attributes.iter().any(|a| a.name.as_bytes().eq_ignore_ascii_case(SCOPE));
    hidden
        && !context
            .current_class_name()
            .is_some_and(|current| codebase.is_instance_of(current.as_bytes(), class))
}

/// BuilderHelper::determineBuilderName(): the class `#[UseEloquentBuilder]` names when the model does not declare
/// `newEloquentBuilder()`, else the one it returns (the Eloquent builder when its type names that).
pub(crate) fn builder_name(codebase: &CodebaseMetadata, model: &[u8]) -> mago_word::Word {
    use mago_codex::ttype::atomic::scalar::TScalar;
    use mago_codex::ttype::atomic::scalar::class_like_string::TClassLikeString;
    let original = |name: mago_word::Word| codebase.get_class_like(name.as_bytes()).map_or(name, |k| k.original_name);
    let builder = original(mago_word::word("Illuminate\\Database\\Eloquent\\Builder"));
    let Some(declaring) = codebase.get_declaring_method_class(model, b"newEloquentBuilder") else {
        return builder;
    };
    if declaring.as_bytes().eq_ignore_ascii_case(MODEL)
        && let Some(k) = codebase.get_class_like(model)
        && let Some(attribute) = k.attributes.iter().find(|a| a.name.as_bytes().eq_ignore_ascii_case(USE_ELOQUENT_BUILDER))
        && let Some(TAtomic::Scalar(TScalar::ClassLikeString(TClassLikeString::Literal { value }))) =
            attribute.arguments.first().and_then(|a| a.value_type.as_ref()).and_then(|t| t.types.first())
    {
        return original(*value);
    }
    let Some(returned) = codebase
        .get_declaring_method(model, b"newEloquentBuilder")
        .and_then(|f| f.return_type_metadata.as_ref())
        .map(|t| &t.type_union)
    else {
        return builder;
    };
    if references(returned, BUILDER) {
        return builder;
    }
    match returned.types.as_ref() {
        [TAtomic::Object(TObject::Named(n))] => original(n.name),
        _ => builder,
    }
}

/// `new GenericObjectType($builderName, [$model])` of the model's own builder.
fn builder_of(codebase: &CodebaseMetadata, model: &TNamedObject) -> TNamedObject {
    let name = builder_name(codebase, model.name.as_bytes());
    let model = TUnion::from_atomic(TAtomic::Object(TObject::Named(model.clone())));
    TNamedObject::new_with_type_parameters(name, Some(vec![model]))
}

/// The builder a forwarded call returns: the builder (`$this` and `static` dropped), its type arguments only when
/// its class is generic.
fn builder_result(codebase: &CodebaseMetadata, builder: &TNamedObject) -> TUnion {
    let mut builder = builder.clone();
    if codebase.get_class_like(builder.name.as_bytes()).is_some_and(|k| k.template_types.is_empty()) {
        builder.type_parameters = None;
    }
    TUnion::from_atomic(plain(TAtomic::Object(TObject::Named(builder))))
}

/// The classes of an object type and its intersection's (`getObjectClassReflections()`).
fn object_classes(ty: &TUnion) -> Vec<mago_word::Word> {
    let mut out = Vec::new();
    for t in ty.types.iter() {
        if let TAtomic::Object(TObject::Named(n)) = t {
            out.push(n.name);
            out.extend(n.intersection_types.iter().flat_map(|b| b.iter()).flat_map(|i| object_classes(&TUnion::from_atomic(i.clone()))));
        }
    }
    out
}

/// ModelForwardsCallsExtension: a model's call of a method of its own builder (`Builder<static>`), whose `static`
/// and Eloquent builder are that builder of the model, or of one the builder forwards.
fn model_forward(
    context: &ProviderContext<'_, '_, '_>,
    class: &[u8],
    method: &[u8],
    invocation: &InvocationInfo<'_, '_, '_>,
) -> Option<TUnion> {
    let codebase = context.codebase();
    let model = TNamedObject::new(codebase.get_class_like(class)?.original_name);
    let builder = builder_of(codebase, &model);
    let name = builder.name;
    if !codebase.method_exists(name.as_bytes(), method) {
        return builder_forward(context, &builder, method, invocation);
    }
    let ty = method_type(codebase, name.as_bytes(), &builder, method, Some((context, invocation)))?;
    // `TypeTraverser::map()`: each Eloquent builder is the model's builder
    let ty = TUnion::from_vec(
        ty.types
            .iter()
            .map(|t| match t {
                TAtomic::Object(TObject::Named(n)) if n.name.as_bytes().eq_ignore_ascii_case(BUILDER) => {
                    TAtomic::Object(TObject::Named(builder.clone()))
                }
                t => t.clone(),
            })
            .collect(),
    );
    Some(crate::laravel::replace_collections(codebase, &ty))
}

/// EloquentBuilderForwardsCallsExtension (BuilderHelper::searchOnEloquentBuilder()): a method an Eloquent builder
/// reaches through its model's scopes, the query builder or a dynamic where returns the builder, unless the query
/// builder's is passed through; the SoftDeletes methods of a model that uses the trait.
// ponytail: macros are not seen
fn builder_forward(
    context: &ProviderContext<'_, '_, '_>,
    builder: &TNamedObject,
    method: &[u8],
    invocation: &InvocationInfo<'_, '_, '_>,
) -> Option<TUnion> {
    let codebase = context.codebase();
    let k = codebase.get_class_like(builder.name.as_bytes())?;
    // the builder's own `TModel`, else its ancestors'
    let model = [k.name, codebase.get_class_like(BUILDER)?.name].into_iter().find_map(|defining| {
        get_specialized_template_type(
            codebase,
            mago_word::word("TModel"),
            defining,
            k,
            builder.type_parameters.as_deref(),
        )
    })?;
    let model = TUnion::from_vec(
        model
            .types
            .iter()
            .flat_map(|t| match t {
                TAtomic::GenericParameter(g) => g.constraint.types.to_vec(),
                t => vec![t.clone()],
            })
            .collect(),
    );
    let classes = object_classes(&model);
    let name = String::from_utf8_lossy(method).to_ascii_lowercase();
    let found = classes.iter().any(|c| super::is_scope(codebase, c.as_bytes(), method))
        || codebase.method_exists(QUERY_BUILDER, method)
        || name.starts_with("where");
    if found {
        if is_passthru(method) && codebase.method_exists(QUERY_BUILDER, method) {
            let query = TNamedObject::new(codebase.get_class_like(QUERY_BUILDER)?.original_name);
            return method_type(codebase, QUERY_BUILDER, &query, method, Some((context, invocation)));
        }
        return Some(builder_result(codebase, builder));
    }
    let soft_deletes = classes.iter().any(|c| {
        codebase.get_class_like(c.as_bytes()).is_some_and(|m| {
            std::iter::once(m.name).chain(m.all_parent_classes.iter().copied()).any(|p| {
                codebase.get_class_like(p.as_bytes()).is_some_and(|p| p.used_traits.iter().any(|t| t.as_bytes().eq_ignore_ascii_case(SOFT_DELETES)))
            })
        })
    });
    if !soft_deletes {
        return None;
    }
    match name.as_str() {
        "restore" => Some(mago_codex::ttype::get_int()),
        "restoreorcreate" | "createorrestore" => Some(model),
        "withtrashed" | "onlytrashed" | "withouttrashed" => Some(TUnion::from_atomic(TAtomic::Object(TObject::Named(
            TNamedObject::new_with_type_parameters(k.original_name, Some(vec![model])),
        )))),
        _ => None,
    }
}

/// A method an Eloquent builder or a relation lacks (its stub's PHPDoc has no `@mixin`).
/// EloquentBuilderForwardsCallsExtension on the builder; RelationForwardsCallsExtension: a method of the related
/// model's builder, whose builder result is the relation itself.
fn forward_builder_or_relation(
    context: &ProviderContext<'_, '_, '_>,
    method: &[u8],
    invocation: &InvocationInfo<'_, '_, '_>,
) -> Option<TUnion> {
    let codebase = context.codebase();
    let target = invocation.inner().target.get_method_context()?;
    let StaticClassType::Object(TObject::Named(receiver)) = &target.class_type else {
        return None;
    };
    let is = |parent: &[u8]| {
        let r = receiver.name.as_bytes();
        r.eq_ignore_ascii_case(parent) || codebase.is_instance_of(r, parent)
    };
    if is(BUILDER) {
        return builder_forward(context, receiver, method, invocation);
    }
    if !is(RELATION) {
        return None;
    }
    let metadata = codebase.get_class_like(receiver.name.as_bytes())?;
    let relation = codebase.get_class_like(RELATION)?;
    let related = get_specialized_template_type(
        codebase,
        mago_word::word("TRelatedModel"),
        relation.name,
        metadata,
        receiver.type_parameters.as_deref(),
    )
    .and_then(|r| match r.types.first() {
        Some(TAtomic::Object(TObject::Named(n))) => Some(TNamedObject::new(n.name)),
        _ => None,
    })
    .unwrap_or_else(|| TNamedObject::new(mago_word::word("Illuminate\\Database\\Eloquent\\Model")));
    if !related.name.as_bytes().eq_ignore_ascii_case(MODEL) && !codebase.is_instance_of(related.name.as_bytes(), MODEL) {
        return None;
    }
    let builder = builder_of(codebase, &related);
    let name = builder.name;
    let (ty, late) = if codebase.method_exists(name.as_bytes(), method) {
        // `(new ObjectType(Builder::class))->isSuperTypeOf($returnType)->yes()` asks before the call resolves the
        // method's templates and conditional types, so neither returns the relation
        let late = codebase
            .get_declaring_method(name.as_bytes(), method)
            .and_then(|f| f.return_type_metadata.as_ref())
            .is_some_and(|r| r.type_union.types.iter().any(|t| t.is_conditional() || matches!(t, TAtomic::GenericParameter(_))));
        (method_type(codebase, name.as_bytes(), &builder, method, Some((context, invocation)))?, late)
    } else {
        (builder_forward(context, &builder, method, invocation)?, false)
    };
    let is_builder = !late
        && !ty.types.is_empty()
        && ty.types.iter().all(|t| {
            matches!(t, TAtomic::Object(TObject::Named(n))
                if n.name.as_bytes().eq_ignore_ascii_case(BUILDER) || codebase.is_instance_of(n.name.as_bytes(), BUILDER))
        });
    if is_builder {
        return Some(TUnion::from_atomic(TAtomic::Object(TObject::Named(receiver.clone()))));
    }
    // RelationCollectionExtension
    Some(crate::laravel::replace_collections(codebase, &ty))
}

/// The related models' own collections in what a builder's or relation's method returns, or a model's static method:
/// EloquentBuilderExtension (and `collection-of<TModel>` for the `find` methods), RelationCollectionExtension and
/// ModelDynamicStaticMethodReturnTypeExtension.
fn own_collection(
    context: &ProviderContext<'_, '_, '_>,
    method: &[u8],
    invocation: &InvocationInfo<'_, '_, '_>,
) -> Option<TUnion> {
    let codebase = context.codebase();
    let target = invocation.inner().target.get_method_context()?;
    // a static call on a class name has the class, not an object
    let receiver = &match &target.class_type {
        StaticClassType::Object(TObject::Named(receiver)) => receiver.clone(),
        StaticClassType::Exact(name) | StaticClassType::Name(name) => TNamedObject::new(*name),
        _ => return None,
    };
    let name = String::from_utf8_lossy(method).to_ascii_lowercase();
    let receiver_class = receiver.name.as_bytes();
    let template = |defining: &[u8], template: &str| {
        let defining = codebase.get_class_like(defining)?.name;
        let metadata = codebase.get_class_like(receiver_class)?;
        get_specialized_template_type(
            codebase,
            mago_word::word(template),
            defining,
            metadata,
            receiver.type_parameters.as_deref(),
        )
    };
    let models = |ty: Option<TUnion>| -> Vec<mago_word::Word> {
        ty.iter()
            .flat_map(|t| t.types.iter())
            .filter_map(|t| match t {
                TAtomic::Object(TObject::Named(n)) => Some(n.name),
                _ => None,
            })
            .collect()
    };
    let returned = || method_type(codebase, receiver_class, receiver, method, Some((context, invocation)));
    let is = |parent: &[u8]| receiver_class.eq_ignore_ascii_case(parent) || codebase.is_instance_of(receiver_class, parent);
    if is(BUILDER) && codebase.method_exists(BUILDER, method) {
        let models = models(template(BUILDER, "TModel"));
        let ty = returned()?;
        if models.is_empty() || !crate::laravel::references_collection(&ty) {
            return None;
        }
        if name.starts_with("find") {
            return Some(crate::laravel::replace_collections(codebase, &ty));
        }
        return Some(TUnion::from_atomic(crate::laravel::collection_class(codebase, models[0].as_bytes())));
    }
    if is(RELATION) {
        if models(template(RELATION, "TRelatedModel")).is_empty() {
            return None;
        }
        let ty = returned()?;
        return crate::laravel::references_collection(&ty).then(|| crate::laravel::replace_collections(codebase, &ty));
    }
    // ponytail: an instance call is not told from a static one (Mago keeps the invocation kind private)
    if is(MODEL)
        && codebase.method_exists(MODEL, method)
        && !name.starts_with("find")
        && name != "__construct"
    {
        let ty = returned()?;
        if !crate::laravel::references_collection(&ty) {
            return None;
        }
        // ponytail: a Builder return (the model's custom builder) is left to the declared type
        if ty.types.iter().any(|t| matches!(t, TAtomic::Object(TObject::Named(n)) if n.name.as_bytes().eq_ignore_ascii_case(BUILDER))) {
            return None;
        }
        return Some(TUnion::from_atomic(crate::laravel::collection_class(codebase, receiver_class)));
    }
    None
}

/// HigherOrderCollectionProxyExtension and HigherOrderCollectionProxyHelper::determineReturnType(): a method of the
/// proxy's `HigherOrderCollectionProxy<'name', TValue, TCollection>` is the value's, its result per the collection
/// method proxied.
fn higher_order_proxy(
    context: &ProviderContext<'_, '_, '_>,
    method: &[u8],
    invocation: &InvocationInfo<'_, '_, '_>,
) -> Option<TUnion> {
    use mago_codex::ttype::atomic::scalar::TScalar;
    let codebase = context.codebase();
    let target = invocation.inner().target.get_method_context()?;
    let StaticClassType::Object(TObject::Named(receiver)) = &target.class_type else {
        return None;
    };
    let [proxied, value, collection] = receiver.type_parameters.as_deref()? else {
        return None;
    };
    let [TAtomic::Scalar(TScalar::String(name))] = proxied.types.as_ref() else {
        return None;
    };
    let name = String::from_utf8_lossy(name.get_known_literal_value()?).into_owned();
    let [TAtomic::Object(TObject::Named(value_object))] = value.types.as_ref() else {
        return None;
    };
    if !codebase.method_exists(value_object.name.as_bytes(), method) {
        return None;
    }
    let returned = || method_type(codebase, value_object.name.as_bytes(), value_object, method, Some((context, invocation)));
    let collection_class = match collection.types.as_ref() {
        [TAtomic::Object(TObject::Named(c))] => c.name,
        _ => mago_word::word("Illuminate\\Support\\Collection"),
    };
    let generic = |class: mago_word::Word, key: TUnion, value: TUnion| -> Option<TUnion> {
        let k = codebase.get_class_like(class.as_bytes())?;
        let parameters = match k.template_types.len() {
            2 => vec![key, value],
            1 if k.template_types.contains_key(&mago_word::word("TModel")) => vec![value],
            _ => return Some(TUnion::from_atomic(TAtomic::Object(TObject::Named(TNamedObject::new(k.original_name))))),
        };
        Some(TUnion::from_atomic(TAtomic::Object(TObject::Named(TNamedObject::new_with_type_parameters(
            k.original_name,
            Some(parameters),
        )))))
    };
    let int = mago_codex::ttype::get_int;
    let support = mago_word::word("Illuminate\\Support\\Collection");
    match name.as_str() {
        "average" | "avg" => Some(mago_codex::ttype::get_float()),
        "contains" | "every" | "some" => Some(mago_codex::ttype::get_bool()),
        "each" | "filter" | "reject" | "skipUntil" | "skipWhile" | "sortBy" | "sortByDesc" | "takeUntil"
        | "takeWhile" | "unique" => generic(collection_class, int(), value.clone()),
        "keyBy" => generic(collection_class, super::super::return_types::benevolent(mago_codex::ttype::get_int_or_string()), value.clone()),
        "first" => Some(TUnion::from_vec(value.types.iter().cloned().chain([TAtomic::Null]).collect())),
        "flatMap" => generic(support, int(), mago_codex::ttype::get_mixed()),
        "groupBy" | "partition" => generic(collection_class, int(), generic(collection_class, int(), value.clone())?),
        "map" => generic(support, super::super::return_types::benevolent(mago_codex::ttype::get_int_or_string()), returned()?),
        "max" | "min" => returned(),
        "sum" => union_comparator::is_contained_by(codebase, &int(), &returned()?, false, false, true, &mut ComparisonResult::new())
            .then(int),
        // ponytail: the ErrorType of `sum` and other methods is left to the declared return type
        _ => None,
    }
}

/// MixinMethodsClassReflectionExtension on a facade's static call: the method of a class its (stub's) `@mixin` tags
/// name, or of one of their mixins.
fn facade_mixin(
    context: &ProviderContext<'_, '_, '_>,
    class: &[u8],
    method: &[u8],
    invocation: &InvocationInfo<'_, '_, '_>,
    depth: usize,
) -> Option<TUnion> {
    let codebase = context.codebase();
    if depth == 0 && !codebase.is_instance_of(class, FACADE) || depth > 3 {
        return None;
    }
    let k = codebase.get_class_like(class)?;
    for mixin in &k.mixins {
        for t in mixin.type_union.types.iter() {
            let TAtomic::Object(TObject::Named(named)) = t else { continue };
            let found = if codebase.method_exists(named.name.as_bytes(), method) {
                method_type(codebase, named.name.as_bytes(), named, method, Some((context, invocation)))
            } else {
                facade_mixin(context, named.name.as_bytes(), method, invocation, depth + 1)
            };
            if found.is_some() {
                return found;
            }
        }
    }
    None
}

/// A call's conditional return types (`($id is array ? collection-of<TModel> : TModel)`) resolved on the type of the
/// argument passed for the subject parameter, as Mago's invocation resolver does: the `then` branch when the argument
/// is contained by the target, `otherwise` when the two can't be identical, both else.
fn resolve_conditionals(
    codebase: &CodebaseMetadata,
    ty: &TUnion,
    function: &mago_codex::metadata::function_like::FunctionLikeMetadata,
    (context, invocation): (&ProviderContext<'_, '_, '_>, &InvocationInfo<'_, '_, '_>),
    options: &TypeExpansionOptions,
) -> TUnion {
    let mut types = Vec::new();
    for t in ty.types.iter() {
        let TAtomic::Conditional(conditional) = t else {
            types.push(t.clone());
            continue;
        };
        let argument = match conditional.subject.types.as_ref() {
            [TAtomic::Variable(name)] => {
                let name = name.as_bytes().strip_prefix(b"$").unwrap_or(name.as_bytes());
                function
                    .parameters
                    .iter()
                    .position(|p| p.get_name().0.as_bytes().strip_prefix(b"$") == Some(name))
                    .and_then(|i| {
                        match invocation.get_argument(i, &[name]) {
                            Some(argument) => context.get_expression_type(argument).cloned(),
                            None => function.parameters[i].get_default_type().map(|d| d.type_union.clone()),
                        }
                    })
            }
            _ => Some((*conditional.subject).clone()),
        };
        let mut target = (*conditional.target).clone();
        expand_union(codebase, &mut target, options);
        let then = (*conditional.then).clone();
        let otherwise = (*conditional.otherwise).clone();
        let (yes, no) = if conditional.negated { (otherwise, then) } else { (then, otherwise) };
        let chosen = match argument {
            Some(subject) if !subject.is_never() => {
                if !union_comparator::can_expression_types_be_identical(codebase, &subject, &target, false, false) {
                    vec![no]
                } else if union_comparator::is_contained_by(
                    codebase,
                    &subject,
                    &target,
                    false,
                    false,
                    true,
                    &mut ComparisonResult::new(),
                ) {
                    vec![yes]
                } else {
                    vec![yes, no]
                }
            }
            _ => vec![yes, no],
        };
        for branch in chosen {
            types.extend(resolve_conditionals(codebase, &branch, function, (context, invocation), options).types.iter().cloned());
        }
    }
    TUnion::from_vec(types)
}

/// The return type of `class`'s native `method` on `object` (`Builder<Model>`): the declaring class's (or trait's)
/// templates as the object specializes them, and its `static` the object.
pub(super) fn method_type(
    codebase: &CodebaseMetadata,
    class: &[u8],
    object: &TNamedObject,
    method: &[u8],
    call: Option<(&ProviderContext<'_, '_, '_>, &InvocationInfo<'_, '_, '_>)>,
) -> Option<TUnion> {
    let declaring = codebase.get_declaring_method_class(class, method)?;
    // the declaring class's (or trait's) method: a trait's is not stored on the using class
    let function = codebase.get_declaring_method(class, method)?;
    let mut ty = function.return_type_metadata.as_ref()?.type_union.clone();
    let metadata = codebase.get_class_like(class)?;
    let options = TypeExpansionOptions {
        self_class: Some(metadata.name),
        static_class_type: StaticClassType::Object(TObject::Named(object.clone())),
        ..Default::default()
    };
    let mut templates = TemplateResult::default();
    let owner = codebase.get_class_like(declaring.as_bytes())?;
    for (name, template) in owner.template_types.iter() {
        let parameters = object.type_parameters.as_deref();
        // an object without arguments has its templates' defaults (`@template TAsync of bool = false`)
        let bound = match (parameters, &template.default) {
            (None, Some(default)) if owner.name == metadata.name => Some(default.clone()),
            _ => get_specialized_template_type(codebase, *name, owner.name, metadata, parameters),
        };
        if let Some(bound) = bound {
            templates.add_lower_bound(*name, GenericParent::ClassLike(owner.name), bound);
        }
    }
    if ty.types.iter().any(TAtomic::is_conditional) {
        if let Some(call) = call {
            infer_method_templates(codebase, function, call, &mut templates);
        }
        ty = inferred_type_replacer::replace(&ty, &templates, codebase);
        ty = resolve_conditionals(codebase, &ty, function, call?, &options);
    }
    expand_union(codebase, &mut ty, &options);
    let ty = inferred_type_replacer::replace(&ty, &templates, codebase);
    Some(TUnion::from_vec(
        ty.types.iter().cloned().map(plain).collect(),
    ))
}

/// `GenericParametersAcceptorResolver::resolve()` for the forwarded method's own templates, as far as the forwarded
/// return types need it: a parameter typed `T` (or `?T`) takes the argument's type, a `callable(...): T` one the
/// passed callable's return type.
fn infer_method_templates(
    codebase: &CodebaseMetadata,
    function: &mago_codex::metadata::function_like::FunctionLikeMetadata,
    (context, invocation): (&ProviderContext<'_, '_, '_>, &InvocationInfo<'_, '_, '_>),
    templates: &mut TemplateResult,
) {
    use mago_codex::ttype::atomic::callable::TCallable;
    let own = |t: &TAtomic| match t {
        TAtomic::GenericParameter(g) if matches!(g.defining_entity, GenericParent::FunctionLike(_)) => Some(g.clone()),
        _ => None,
    };
    let single = |t: &TUnion| match t.types.as_ref() {
        [one] => own(one),
        _ => None,
    };
    for (i, parameter) in function.parameters.iter().enumerate() {
        let Some(declared) = parameter.get_type_metadata().map(|t| &t.type_union) else { continue };
        let name = parameter.get_name().0;
        let name = name.as_bytes().strip_prefix(b"$").unwrap_or(name.as_bytes());
        let Some(argument) = invocation.get_argument(i, &[name]).and_then(|a| context.get_expression_type(a)) else {
            continue;
        };
        for atomic in declared.types.iter() {
            if atomic.is_null() {
                continue;
            }
            if let Some(g) = own(atomic)
                && declared.types.iter().all(|t| t.is_null() || own(t).is_some())
            {
                templates.add_lower_bound(g.parameter_name, g.defining_entity, argument.clone());
            }
            let TAtomic::Callable(TCallable::Signature(signature)) = atomic else { continue };
            let Some(g) = signature.return_type.as_deref().and_then(single) else { continue };
            let returns: Vec<TUnion> = argument
                .types
                .iter()
                .filter_map(|a| match a {
                    TAtomic::Callable(TCallable::Signature(s)) => {
                        Some(s.return_type.as_deref().cloned().unwrap_or_else(mago_codex::ttype::get_mixed))
                    }
                    TAtomic::Callable(TCallable::Alias(id)) => codebase
                        .get_function_like(id)
                        .and_then(|f| f.return_type_metadata.as_ref())
                        .map(|r| r.type_union.clone()),
                    _ => None,
                })
                .collect();
            for returned in returns {
                templates.add_lower_bound(g.parameter_name, g.defining_entity, returned);
            }
        }
    }
}

/// The type as a generic object type: neither `$this` nor `static`, which Mago would bind to the call site again.
fn plain(t: TAtomic) -> TAtomic {
    match t {
        TAtomic::Object(TObject::Named(n)) => TAtomic::Object(TObject::Named(
            TNamedObject {
                is_this: false,
                ..n
            }
            .with_is_static(false),
        )),
        t => t,
    }
}

/// ModelForwardsCallsExtension on a static call of a model's non-public `#[Scope]` method from outside the model
/// (`Activity::completed()`): PHPStan's ClassReflection::getMethod() asks the extensions when the scope cannot call
/// the native method, and the scope returns the model's builder. Mago resolves the protected instance method and
/// gives up, so the hook replaces the call's type.
/// The file's resolved names, by their start offset.
pub(crate) struct ScopeStaticCalls(pub(crate) std::collections::HashMap<u32, Vec<u8>>);

impl Provider for ScopeStaticCalls {
    fn meta() -> &'static ProviderMeta {
        &META
    }
}

impl mago_analyzer::plugin::ExpressionHook for ScopeStaticCalls {
    fn after_expression(
        &self,
        expr: &mago_syntax::cst::Expression<'_>,
        context: &mut mago_analyzer::plugin::HookContext<'_, '_>,
    ) -> mago_analyzer::plugin::HookResult<()> {
        use mago_span::HasSpan;
        use mago_syntax::cst::{Call, ClassLikeMemberSelector, Expression};
        let Expression::Call(Call::StaticMethod(call)) = expr else { return Ok(()) };
        let ClassLikeMemberSelector::Identifier(method) = &call.method else { return Ok(()) };
        let codebase = context.codebase();
        let Expression::Identifier(id) = call.class else { return Ok(()) };
        let Some(class) = self.0.get(&id.span().start.offset) else { return Ok(()) };
        let Some(class) = codebase.get_class_like(class).map(|k| k.name) else { return Ok(()) };
        let Some(m) = codebase.get_declaring_method(class.as_bytes(), method.value) else { return Ok(()) };
        let scope = m.method_metadata.as_ref().is_some_and(|mm| !mm.visibility.is_public())
            && m.attributes.iter().any(|a| a.name.as_bytes().eq_ignore_ascii_case(SCOPE));
        let inside = context.current_class_name().is_some_and(|c| codebase.is_instance_of(c.as_bytes(), class.as_bytes()));
        if !scope || inside || !codebase.is_instance_of(class.as_bytes(), MODEL) {
            return Ok(());
        }
        let Some(model) = codebase.get_class_like(class.as_bytes()).map(|k| TNamedObject::new(k.original_name)) else {
            return Ok(());
        };
        let builder = builder_of(codebase, &model);
        context.set_expression_type(expr, TUnion::from_atomic(TAtomic::Object(TObject::Named(builder))));
        Ok(())
    }
}
