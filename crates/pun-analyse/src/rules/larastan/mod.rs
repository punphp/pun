//! Larastan 3.10.0's rules (the `laravel` group), run when the project's PHPStan config includes Larastan's
//! extension.neon. Each rule is registered by extension.neon whatever the level; the opt-in ones only when their
//! parameter is on. See [`Larastan`].

mod appends;
mod calls;
mod classes;
mod collections;
mod collectors;
mod config;
mod console;
mod dispatch;
mod extensions;
mod forwards;
mod octane;
mod relations;

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::object::named::TNamedObject;
use mago_codex::ttype::union::TUnion;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;
use pun_phpstan::{Array, Value};

use super::Checker;

pub(crate) use appends::property;
pub(crate) use calls::{function_call, method_call, static_method_call};
pub(crate) use classes::{class, method};
pub(crate) use collectors::finish;
pub(super) use extensions::{Functions as AppFunctions, Methods as AppMethods};
pub(super) use forwards::ModelForwards;

pub const IDENTIFIERS: &[&str] = &[
    "larastan.uselessConstructs.with",
    "larastan.uselessConstructs.value",
    "larastan.noEnvCallsOutsideOfConfig",
    "larastan.noModelMake",
    "larastan.deferrableServiceProvider.missingProvides",
    "larastan.noAuthFacadeInRequestScope",
    "larastan.noAuthHelperInRequestScope",
    "larastan.noPublicModelScopeMethod",
    "larastan.noPublicModelAccessorMethod",
    "larastan.octaneCompatibility",
    "rules.modelAppends",
    "larastan.jobs.noConstructor",
    "larastan.events.noConstructor",
    "larastan.console.undefinedArgument",
    "larastan.console.undefinedOption",
    "larastan.configCollection",
    "larastan.unnecessaryEnumerableToArrayCall",
    "larastan.relationExistence",
    "larastan.noUnnecessaryCollectionCall",
    "larastan.unusedViews",
    "larastan.missingTranslations",
];

const MODEL: &[u8] = b"illuminate\\database\\eloquent\\model";
const REQUEST: &[u8] = b"illuminate\\http\\request";

/// Larastan's switches (extension.neon's parameters), with the Laravel app's config directory.
#[derive(Clone, Debug)]
pub struct Larastan {
    /// The Laravel app (whose migrations are `database/migrations`).
    pub root: PathBuf,
    /// `configDirectories` as absolute globs, or the app's `config_path()`.
    pub config_directories: Vec<String>,
    /// `noEnvCallsOutsideOfConfig` (default true).
    pub no_env_calls_outside_of_config: bool,
    /// `noModelMake` (default true).
    pub no_model_make: bool,
    /// `checkOctaneCompatibility` (default false).
    pub check_octane_compatibility: bool,
    /// `checkModelMethodVisibility` (default false).
    pub check_model_method_visibility: bool,
    /// `checkAuthCallsWhenInRequestScope` (default false).
    pub check_auth_calls_when_in_request_scope: bool,
    /// `checkModelAppends` (default true).
    pub check_model_appends: bool,
    /// The analysed models' database properties, by lowercase class name (filled by [`Larastan::load`]).
    pub(crate) database_properties: HashMap<String, HashSet<String>>,
    /// `noUnnecessaryCollectionCall` (default true), with `noUnnecessaryCollectionCallOnly` and
    /// `noUnnecessaryCollectionCallExcept`: the lowercase method names it checks.
    pub no_unnecessary_collection_call: bool,
    pub no_unnecessary_collection_call_methods: Vec<String>,
    /// `noUnnecessaryEnumerableToArrayCalls` (default false).
    pub no_unnecessary_enumerable_to_array_calls: bool,
    /// `checkUnusedViews` (default false) and `viewDirectories` (absolute).
    pub check_unused_views: bool,
    pub view_directories: Vec<PathBuf>,
    /// `checkMissingTranslations` (default false) and `translationDirectories` (absolute).
    pub check_missing_translations: bool,
    pub translation_directories: Vec<PathBuf>,
    /// `checkConfigTypes` (default false).
    pub check_config_types: bool,
    /// The app's config files (filled by [`Larastan::load`] when `checkConfigTypes` is on).
    pub(crate) configs: crate::larastan::config::Configs,
    /// The models' database properties and their types, by lowercase class name (filled by [`Larastan::load`]).
    pub(crate) models: std::sync::Arc<crate::laravel::Models>,
    /// What the extensions ask the booted app (filled by [`Larastan::load`]).
    pub(crate) app: std::sync::Arc<crate::larastan::app::App>,
    /// The app's console commands (filled by [`Larastan::load`]).
    pub(crate) commands: Vec<crate::larastan::console::Command>,
}

impl Larastan {
    /// Larastan's defaults for the app at `root` (whose `config_path()` is `root/config`).
    pub fn new(root: &Path) -> Larastan {
        Larastan {
            root: root.to_path_buf(),
            config_directories: vec![root.join("config").to_string_lossy().into_owned()],
            no_env_calls_outside_of_config: true,
            no_model_make: true,
            check_octane_compatibility: false,
            check_model_method_visibility: false,
            check_auth_calls_when_in_request_scope: false,
            check_model_appends: true,
            database_properties: HashMap::new(),
            commands: Vec::new(),
            check_config_types: false,
            check_unused_views: false,
            view_directories: Vec::new(),
            check_missing_translations: false,
            translation_directories: Vec::new(),
            no_unnecessary_enumerable_to_array_calls: false,
            no_unnecessary_collection_call: true,
            no_unnecessary_collection_call_methods: collections::should_handle(&[], &[]),
            configs: Default::default(),
            app: Default::default(),
            models: Default::default(),
        }
    }

    /// What the rules read of the analysed models and the migrations.
    pub(crate) fn load(
        &mut self,
        project: &crate::Project,
        codebase: &mago_codex::metadata::CodebaseMetadata,
    ) {
        let app = crate::Laravel {
            root: &self.root,
            parse_casts_method: false,
        };
        self.database_properties = crate::laravel::database_properties(app, project, codebase);
        self.models = std::sync::Arc::new(crate::laravel::models(app, project, codebase));
        self.commands = crate::larastan::console::commands(project, codebase);
        self.app = std::sync::Arc::new(crate::larastan::app::App::load(&self.root));
        if self.check_config_types {
            self.configs = crate::larastan::config::Configs::load(
                &self.config_directories,
                &self.root,
                codebase,
            );
        }
    }

    /// The switches of a resolved PHPStan config, `None` when it does not include Larastan (whose extension.neon
    /// declares `noModelMake`). Relative `configDirectories` are relative to `root`, the working directory.
    pub fn from_params(root: &Path, params: &Array) -> Option<Larastan> {
        params.get_str("noModelMake")?;
        let flag = |k: &str, default: bool| match params.get_str(k) {
            Some(Value::Bool(b)) => *b,
            _ => default,
        };
        let mut larastan = Larastan::new(root);
        let directories: Vec<String> = params
            .get_str("configDirectories")
            .and_then(Value::as_array)
            .map(|a| {
                a.values()
                    .filter_map(Value::as_str)
                    .map(|d| root.join(d).to_string_lossy().into_owned())
                    .collect()
            })
            .unwrap_or_default();
        if !directories.is_empty() {
            larastan.config_directories = directories;
        }
        larastan.no_env_calls_outside_of_config = flag("noEnvCallsOutsideOfConfig", true);
        larastan.no_model_make = flag("noModelMake", true);
        larastan.check_octane_compatibility = flag("checkOctaneCompatibility", false);
        larastan.check_model_method_visibility = flag("checkModelMethodVisibility", false);
        larastan.check_auth_calls_when_in_request_scope =
            flag("checkAuthCallsWhenInRequestScope", false);
        larastan.check_model_appends = flag("checkModelAppends", true);
        larastan.check_config_types = flag("checkConfigTypes", false);
        larastan.no_unnecessary_enumerable_to_array_calls =
            flag("noUnnecessaryEnumerableToArrayCalls", false);
        larastan.no_unnecessary_collection_call = flag("noUnnecessaryCollectionCall", true);
        larastan.check_unused_views = flag("checkUnusedViews", false);
        larastan.check_missing_translations = flag("checkMissingTranslations", false);
        let names = |k: &str| -> Vec<String> {
            params
                .get_str(k)
                .and_then(Value::as_array)
                .map(|a| {
                    a.values()
                        .filter_map(Value::as_str)
                        .map(str::to_string)
                        .collect()
                })
                .unwrap_or_default()
        };
        larastan.view_directories = names("viewDirectories")
            .iter()
            .map(|d| root.join(d))
            .collect();
        larastan.translation_directories = names("translationDirectories")
            .iter()
            .map(|d| root.join(d))
            .collect();
        larastan.no_unnecessary_collection_call_methods = collections::should_handle(
            &names("noUnnecessaryCollectionCallOnly"),
            &names("noUnnecessaryCollectionCallExcept"),
        );
        Some(larastan)
    }
}

/// Per-walk state: for each function-like scope entered, whether `$request` is defined there as a
/// `Illuminate\Http\Request` (`$scope->hasVariableType('request')->yes()` and its type).
#[derive(Default)]
pub(crate) struct State {
    request: Vec<bool>,
}

/// `ModelPropertyExtension::hasProperty()`: a database column of the model that no accessor or `@property` tag hides.
fn has_database_property(c: &Checker, model: &str, column: &str) -> bool {
    c.larastan.is_some_and(|larastan| {
        !crate::laravel::has_accessor(c.codebase, model, column)
            && larastan
                .database_properties
                .get(&model.to_ascii_lowercase())
                .is_some_and(|p| p.contains(column))
            && !c
                .codebase
                .get_class_like(model.as_bytes())
                .is_some_and(|k| appends::property_tag(k, column))
    })
}

/// Larastan's model property extensions on `$property` (with its `$`): ModelAccessorExtension, ModelPropertyExtension
/// (a database column) and ModelRelationsExtension (a relation method, or its `_count`).
pub(crate) fn has_model_property(c: &Checker, k: &ClassLikeMetadata, property: &[u8]) -> bool {
    if c.larastan.is_none() || !is_model(c, k) {
        return false;
    }
    let name = String::from_utf8_lossy(property.strip_prefix(b"$").unwrap_or(property)).into_owned();
    let class = k.original_name.as_str_lossy().into_owned();
    if crate::laravel::has_accessor(c.codebase, &class, &name)
        || !k.flags.is_abstract() && has_database_property(c, &class, &name)
    {
        return true;
    }
    if appends::property_tag(k, &name) {
        return false;
    }
    let methods = match name.strip_suffix("_count") {
        Some(relation) => vec![crate::laravel::camel(relation), relation.to_string()],
        None => vec![name.clone()],
    };
    methods.iter().any(|m| {
        c.codebase
            .get_declaring_method(k.name.as_bytes(), m.as_bytes())
            .and_then(|f| f.return_type_metadata.as_ref())
            .is_some_and(|t| is_object_of(c, &t.type_union, b"illuminate\\database\\eloquent\\relations\\relation"))
    })
}

/// BuilderHelper::searchOnEloquentBuilder()'s scopes of `model`: a non-public native method with the `#[Scope]`
/// attribute, a `@method` tag `scopeName` or a native `scopeName` method.
pub(crate) fn has_scope(c: &Checker, model: &str, method: &[u8]) -> bool {
    let codebase = c.codebase;
    let Some(k) = codebase.get_class_like(model.as_bytes()) else {
        return false;
    };
    let attribute = codebase.get_declaring_method(k.name.as_bytes(), method).is_some_and(|m| {
        m.method_metadata.as_ref().is_some_and(|mm| !mm.visibility.is_public())
            && m.attributes.iter().any(|a| a.name.as_bytes().eq_ignore_ascii_case(b"Illuminate\\Database\\Eloquent\\Attributes\\Scope"))
    });
    let mut scope = b"scope".to_vec();
    scope.extend(method.iter().take(1).map(u8::to_ascii_uppercase));
    scope.extend(method.iter().skip(1));
    attribute || codebase.get_declaring_method(k.name.as_bytes(), &scope).is_some()
}

/// `ObjectType($parent)->isSuperTypeOf(ObjectType($class))->yes()`, for lowercase `parent`.
pub(crate) fn is_a(c: &Checker, class: &[u8], parent: &[u8]) -> bool {
    class.eq_ignore_ascii_case(parent)
        || c.codebase
            .is_instance_of(&class.to_ascii_lowercase(), parent)
}

/// `$classReflection->is(Model::class)`.
pub(crate) fn is_model(c: &Checker, k: &ClassLikeMetadata) -> bool {
    class_is(c, k, MODEL)
}

/// `$classReflection->is($parent)` for the class a node is in.
fn class_is(c: &Checker, k: &ClassLikeMetadata, parent: &[u8]) -> bool {
    is_a(c, k.name.as_bytes(), parent)
}

/// The names of the objects a type is (`getObjectClassNames()`), `None` when one of its parts is no named object.
fn object_names(ty: &TUnion) -> Option<Vec<&[u8]>> {
    named_objects(ty).map(|objects| objects.iter().map(|n| n.name.as_bytes()).collect())
}

/// The type's parts when every one is a named object.
fn named_objects(ty: &TUnion) -> Option<Vec<&TNamedObject>> {
    ty.types
        .iter()
        .map(|t| match t {
            TAtomic::Object(TObject::Named(n)) => Some(n),
            _ => None,
        })
        .collect()
}

/// `ObjectType($class)->isSuperTypeOf($ty)->yes()`: every part of the type is that class or a subclass.
fn is_object_of(c: &Checker, ty: &TUnion, class: &[u8]) -> bool {
    object_names(ty)
        .is_some_and(|names| !names.is_empty() && names.iter().all(|n| is_a(c, n, class)))
}

/// A parameter list's `$request` parameter typed as a Request, without null (the implicit nullable of a `null`
/// default included).
fn request_parameter(c: &Checker, parameters: &FunctionLikeParameterList) -> bool {
    parameters.parameters.iter().any(|p| {
        p.variable.name == b"$request"
            && p.default_value
                .as_ref()
                .is_none_or(|d| !matches!(d.value, Expression::Literal(Literal::Null(_))))
            && match &p.hint {
                Some(Hint::Identifier(id)) => is_a(c, c.names.get(&id.span()), REQUEST),
                _ => false,
            }
    })
}

impl Checker<'_, '_> {
    pub(super) fn larastan_enter_function(&mut self, parameters: &FunctionLikeParameterList) {
        let request = self.larastan.is_some() && request_parameter(self, parameters);
        self.larastan_state.request.push(request);
    }

    /// A closure: its parameters, and `use ($request)` keeps the outer `$request`.
    pub(super) fn larastan_enter_closure(&mut self, f: &Closure) {
        let outer = self.larastan_state.request.last().copied().unwrap_or(false);
        let used = f
            .use_clause
            .as_ref()
            .is_some_and(|u| u.variables.iter().any(|v| v.variable.name == b"$request"));
        let request = self.larastan.is_some()
            && (request_parameter(self, &f.parameter_list) || (used && outer));
        self.larastan_state.request.push(request);
    }

    pub(super) fn larastan_leave_function(&mut self) {
        self.larastan_state.request.pop();
    }

    fn request_in_scope(&self) -> bool {
        self.larastan_state.request.last().copied().unwrap_or(false)
    }
}

/// The expressions whose type the rules here read, for [`super::infer`]: the arguments of `with()` and `value()`,
/// the class expression of `X::make()`, the object of `->singleton()` / `->bind()` with a closure.
pub(crate) fn typed_expressions<'arena>(
    larastan: Option<&Larastan>,
    names: &mago_names::ResolvedNames<'arena>,
    program: &Program<'arena>,
) -> Vec<Span> {
    let Some(larastan) = larastan else {
        return Vec::new();
    };
    let mut wanted = Wanted {
        larastan,
        names,
        spans: Vec::new(),
    };
    wanted.walk_program(program, &mut ());
    wanted.spans
}

struct Wanted<'l, 'n, 'arena> {
    larastan: &'l Larastan,
    names: &'n mago_names::ResolvedNames<'arena>,
    spans: Vec<Span>,
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Wanted<'_, '_, 'arena> {
    fn walk_in_function_call(&mut self, call: &'ast FunctionCall<'arena>, _: &mut ()) {
        let Expression::Identifier(id) = call.function else {
            return;
        };
        let name = calls::written_name(self.names, id);
        let args = &call.argument_list.arguments;
        let wanted = if name.eq_ignore_ascii_case("with") {
            args.get(1)
        } else if name.eq_ignore_ascii_case("value") {
            args.first()
        } else {
            None
        };
        self.spans.extend(wanted.map(|a| a.value().span()));
    }
    fn walk_in_static_method_call(&mut self, call: &'ast StaticMethodCall<'arena>, _: &mut ()) {
        if self.larastan.check_config_types {
            self.spans.extend(config::typed(self.names, call));
        }
        if let Some(arg) = relations::relation_call(&call.method, &call.argument_list) {
            self.spans.push(arg.span());
            if !matches!(call.class, Expression::Identifier(_)) {
                self.spans.push(call.class.span());
            }
        }
        if let ClassLikeMemberSelector::Identifier(m) = &call.method
            && dispatch::conditional(m.value)
            && let Some(first) = call.argument_list.arguments.first()
        {
            self.spans.push(first.value().span());
        }
        if self.larastan.no_model_make
            && matches!(&call.method, ClassLikeMemberSelector::Identifier(m) if m.value == b"make")
            && !matches!(
                call.class,
                Expression::Identifier(_)
                    | Expression::Self_(_)
                    | Expression::Static(_)
                    | Expression::Parent(_)
            )
        {
            self.spans.push(call.class.span());
        }
    }
    fn walk_in_method_call(&mut self, call: &'ast MethodCall<'arena>, _: &mut ()) {
        if self.larastan.no_unnecessary_enumerable_to_array_calls
            && collections::to_array(call).is_some()
        {
            self.spans.push(call.object.span());
        }
        if let Some(arg) = relations::relation_call(&call.method, &call.argument_list) {
            self.spans.extend([arg.span(), call.object.span()]);
        }
        if self.larastan.no_unnecessary_collection_call {
            self.spans.extend(collections::unnecessary_typed(call));
        }
        if collectors::typed_method_call(self.larastan, call).is_some() {
            self.spans.push(call.object.span());
        }
        if let Some((_, arg)) = console::input_call(call) {
            self.spans.extend([call.object.span(), arg.span()]);
        }
        if self.larastan.check_octane_compatibility && octane::is_binding(call) {
            self.spans.push(call.object.span());
        }
    }
}
