//! Larastan's call forwarding for the rules' inference. ModelForwardsCallsExtension: a method a model does not declare
//! is its `Builder<static>`'s (`Model::__call()` and `__callStatic()` forward to a new query), with the builder's
//! `TModel` the model and its `static` the builder. EloquentBuilderForwardsCallsExtension: a query builder method an
//! Eloquent builder reaches through its `@mixin` returns the Eloquent builder, unless it is passed through.
//! Mago resolves the mixin itself; these providers only replace the return types.

use mago_analyzer::plugin::context::{InvocationInfo, ProviderContext};
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
use mago_codex::ttype::union::TUnion;

const MODEL: &[u8] = b"Illuminate\\Database\\Eloquent\\Model";
const BUILDER: &[u8] = b"Illuminate\\Database\\Eloquent\\Builder";
const QUERY_BUILDER: &[u8] = b"Illuminate\\Database\\Query\\Builder";

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

pub(crate) struct ModelForwards;

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
            if PASSTHRU.contains(
                &String::from_utf8_lossy(method)
                    .to_ascii_lowercase()
                    .as_str(),
            ) {
                return method_type(codebase, QUERY_BUILDER, receiver, method);
            }
            return Some(TUnion::from_atomic(plain(TAtomic::Object(TObject::Named(
                receiver.clone(),
            )))));
        }
        if !codebase.is_instance_of(class, MODEL) || codebase.method_exists(class, method) {
            return None;
        }
        let model = codebase.get_class_like(class)?.original_name;
        let builder = codebase.get_class_like(BUILDER)?;
        let model = TUnion::from_atomic(TAtomic::Object(TObject::Named(TNamedObject::new(model))));
        let builder_type =
            TNamedObject::new_with_type_parameters(builder.original_name, Some(vec![model]));
        method_type(codebase, BUILDER, &builder_type, method)
    }
}

/// The return type of `class`'s native `method` on `object` (`Builder<Model>`): the declaring class's (or trait's)
/// templates as the object specializes them, and its `static` the object.
fn method_type(
    codebase: &CodebaseMetadata,
    class: &[u8],
    object: &TNamedObject,
    method: &[u8],
) -> Option<TUnion> {
    let declaring = codebase.get_declaring_method_class(class, method)?;
    let mut ty = codebase
        .get_method(class, method)?
        .return_type_metadata
        .as_ref()?
        .type_union
        .clone();
    // a conditional return type (`find($id)`) depends on the arguments, which a return type provider cannot see:
    // expanding it would yield both branches
    if ty.types.iter().any(TAtomic::is_conditional) {
        return None;
    }
    let metadata = codebase.get_class_like(class)?;
    expand_union(
        codebase,
        &mut ty,
        &TypeExpansionOptions {
            self_class: Some(metadata.name),
            static_class_type: StaticClassType::Object(TObject::Named(object.clone())),
            ..Default::default()
        },
    );
    let mut templates = TemplateResult::default();
    let owner = codebase.get_class_like(declaring.as_bytes())?;
    for (name, _) in owner.template_types.iter() {
        let parameters = object.type_parameters.as_deref();
        if let Some(bound) =
            get_specialized_template_type(codebase, *name, owner.name, metadata, parameters)
        {
            templates.add_lower_bound(*name, GenericParent::ClassLike(owner.name), bound);
        }
    }
    let ty = inferred_type_replacer::replace(&ty, &templates, codebase);
    Some(TUnion::from_vec(
        ty.types.iter().cloned().map(plain).collect(),
    ))
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
