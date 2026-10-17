//! Larastan's model knowledge as a Mago analyser plugin (analyse.md §8): a model's database columns are typed
//! properties, read from the migrations, as Larastan's `ModelPropertyExtension` and `ModelPropertyHelper` give them.
//! Registered as an expression hook: after Mago types `$model->column` (through `Model::__get`, so `mixed`), the hook
//! replaces the type with the column's.
//!
//! Without PHP running, the model instance Larastan asks (`getTable()`, `getKeyName()`, `getCasts()`, `getDates()`,
//! `Date::now()`) is answered from the model's own source and its parent models': `$table`, `$primaryKey`,
//! `$keyType`, `$incrementing`, `$timestamps`, `$casts`, `casts()`, the `SoftDeletes` trait, and a `Date::use()`
//! call anywhere in the analysed files.

mod schema;
mod strings;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use mago_allocator::LocalArena;
use mago_analyzer::plugin::{
    ExpressionHook, HookContext, HookResult, PluginRegistry, Provider, ProviderMeta,
};
use mago_codex::metadata::CodebaseMetadata;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::union::TUnion;
use mago_codex::ttype::*;
use mago_database::file::FileType;
use mago_names::ResolvedNames;
use mago_names::resolver::NameResolver;
use mago_span::HasSpan;
use mago_syntax::cst::*;
use mago_syntax::parser::parse_file;
use mago_syntax::walker::MutWalker;
use mago_word::Word;

use crate::codebase::read;
use crate::{Project, php_files};
use schema::{Column, SchemaAggregator};
use strings::{camel, class_basename, plural_studly, snake, studly};

const MODEL: &str = "Illuminate\\Database\\Eloquent\\Model";
const ATTRIBUTE: &str = "Illuminate\\Database\\Eloquent\\Casts\\Attribute";
const CARBON_IMMUTABLE: &str = "Carbon\\CarbonImmutable";

/// A Laravel app: its root (for `database/migrations`) and Larastan's options.
#[derive(Clone, Copy, Debug)]
pub struct Laravel<'a> {
    pub root: &'a Path,
    /// Larastan's `parseModelCastsMethod` (default false): read the array `casts()` returns.
    pub parse_casts_method: bool,
}

/// Registers the model-attribute hook for the Laravel app.
pub fn register(
    registry: &mut PluginRegistry,
    app: Laravel,
    project: &Project,
    codebase: &CodebaseMetadata,
) {
    registry.register_expression_hook(ModelAttributes {
        models: models(app, project, codebase),
    });
}

/// What a model class declares itself (`None`: inherited or Laravel's default).
#[derive(Default)]
struct ModelSource {
    table: Option<String>,
    primary_key: Option<String>,
    key_type: Option<String>,
    incrementing: Option<bool>,
    timestamps: Option<bool>,
    /// `$casts`.
    casts: Vec<(String, String)>,
    /// The array `casts()` returns.
    casts_method: Vec<(String, String)>,
}

/// The model classes, `Date::use()`'s class, from the analysed files.
#[derive(Default)]
struct Scan {
    models: HashMap<String, (String, ModelSource)>,
    date_class: Option<String>,
}

struct Collector<'s, 'n, 'arena> {
    scan: &'s mut Scan,
    names: &'n ResolvedNames<'arena>,
    codebase: &'s CodebaseMetadata,
    parse_casts_method: bool,
}

fn resolved(names: &ResolvedNames, e: &Expression) -> Option<String> {
    match e {
        Expression::Identifier(id) => {
            Some(String::from_utf8_lossy(names.get(&id.span())).into_owned())
        }
        _ => None,
    }
}

/// A constant string: `'...'` or `Foo::class`.
fn constant_string(names: &ResolvedNames, e: &Expression) -> Option<String> {
    match e {
        Expression::Literal(Literal::String(s)) => s
            .value
            .map(|v| String::from_utf8_lossy(v).replace("\\\\", "\\")),
        Expression::Access(Access::ClassConstant(c)) if matches!(&c.constant, ClassLikeConstantSelector::Identifier(i) if i.value.eq_ignore_ascii_case(b"class")) => {
            resolved(names, c.class)
        }
        _ => None,
    }
}

fn string_pairs(names: &ResolvedNames, e: &Expression) -> Vec<(String, String)> {
    let elements = match e {
        Expression::Array(a) => &a.elements,
        Expression::LegacyArray(a) => &a.elements,
        _ => return Vec::new(),
    };
    elements
        .iter()
        .filter_map(|el| match el {
            ArrayElement::KeyValue(kv) => Some((
                constant_string(names, kv.key)?,
                constant_string(names, kv.value)?,
            )),
            _ => None,
        })
        .collect()
}

/// The first `return` in a method body (`NodeFinder::findFirstInstanceOf(Return_)`).
#[derive(Default)]
struct FirstReturn<'ast, 'arena>(Option<&'ast Return<'arena>>);

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for FirstReturn<'ast, 'arena> {
    fn walk_in_return(&mut self, r: &'ast Return<'arena>, _: &mut ()) {
        self.0.get_or_insert(r);
    }
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Collector<'_, '_, 'arena> {
    fn walk_in_class(&mut self, class: &'ast Class<'arena>, _: &mut ()) {
        let name = String::from_utf8_lossy(self.names.get(&class.name.span())).into_owned();
        if !self
            .codebase
            .is_instance_of(name.as_bytes(), MODEL.as_bytes())
        {
            return;
        }
        let mut model = ModelSource::default();
        for member in class.members.iter() {
            match member {
                ClassLikeMember::Property(Property::Plain(p)) => {
                    for item in p.items.iter() {
                        let PropertyItem::Concrete(item) = item else {
                            continue;
                        };
                        let string = constant_string(self.names, item.value);
                        let bool = match item.value {
                            Expression::Literal(Literal::True(_)) => Some(true),
                            Expression::Literal(Literal::False(_)) => Some(false),
                            _ => None,
                        };
                        match item.variable.name {
                            b"$table" => model.table = string,
                            b"$primaryKey" => model.primary_key = string,
                            b"$keyType" => model.key_type = string,
                            b"$incrementing" => model.incrementing = bool,
                            b"$timestamps" => model.timestamps = bool,
                            b"$casts" => model.casts = string_pairs(self.names, item.value),
                            _ => {}
                        }
                    }
                }
                ClassLikeMember::Method(m)
                    if self.parse_casts_method && m.name.value == b"casts" =>
                {
                    let mut first = FirstReturn::default();
                    first.walk_method(m, &mut ());
                    if let Some(value) = first.0.and_then(|r| r.value) {
                        model.casts_method = string_pairs(self.names, value);
                    }
                }
                _ => {}
            }
        }
        self.scan
            .models
            .insert(name.to_ascii_lowercase(), (name, model));
    }

    fn walk_in_static_method_call(&mut self, call: &'ast StaticMethodCall<'arena>, _: &mut ()) {
        if resolved(self.names, call.class)
            .is_some_and(|c| c.eq_ignore_ascii_case("Illuminate\\Support\\Facades\\Date"))
            && matches!(&call.method, ClassLikeMemberSelector::Identifier(m) if m.value.eq_ignore_ascii_case(b"use"))
            && let Some(Argument::Positional(a)) = call.argument_list.arguments.first()
            && let Some(class) = constant_string(self.names, a.value)
        {
            self.scan.date_class = Some(class);
        }
    }
}

/// A model with everything Larastan asks its instance for.
struct Model<'a> {
    name: &'a str,
    table: String,
    key: String,
    key_type: String,
    casts: HashMap<String, String>,
    dates: Vec<&'static str>,
}

fn model<'a>(scan: &'a Scan, codebase: &CodebaseMetadata, class: &str) -> Option<Model<'a>> {
    let (name, _) = scan.models.get(&class.to_ascii_lowercase())?;
    // the class and its parents, nearest first
    let mut chain = Vec::new();
    let mut current = Some(name.clone());
    while let Some(c) = current {
        if let Some((_, source)) = scan.models.get(&c.to_ascii_lowercase()) {
            chain.push(source);
        }
        current = codebase
            .get_class_like(c.as_bytes())
            .and_then(|m| m.direct_parent_class)
            .map(|p| p.as_str_lossy().into_owned());
    }
    fn first<T>(chain: &[&ModelSource], f: impl Fn(&ModelSource) -> Option<T>) -> Option<T> {
        chain.iter().find_map(|s| f(s))
    }
    let table = first(&chain, |s| s.table.clone())
        .unwrap_or_else(|| snake(&plural_studly(class_basename(name))));
    let key = first(&chain, |s| s.primary_key.clone()).unwrap_or_else(|| "id".into());
    // Model::getCasts(): the key cast to its type when incrementing, then `$casts`; then `casts()` (Larastan)
    let key_type = first(&chain, |s| s.key_type.clone()).unwrap_or_else(|| "int".into());
    let mut casts = HashMap::new();
    if first(&chain, |s| s.incrementing).unwrap_or(true) {
        casts.insert(key.clone(), key_type.clone());
    }
    casts.extend(
        first(&chain, |s| (!s.casts.is_empty()).then(|| s.casts.clone())).unwrap_or_default(),
    );
    // the nearest `casts()` declaration
    casts.extend(
        first(&chain, |s| {
            (!s.casts_method.is_empty()).then(|| s.casts_method.clone())
        })
        .unwrap_or_default(),
    );
    let mut dates = Vec::new();
    if first(&chain, |s| s.timestamps).unwrap_or(true) {
        dates.extend(["created_at", "updated_at"]);
    }
    if codebase.class_uses_trait(
        name.as_bytes(),
        b"Illuminate\\Database\\Eloquent\\SoftDeletes",
    ) {
        dates.push("deleted_at");
    }
    Some(Model {
        name,
        table,
        key,
        key_type,
        casts,
        dates,
    })
}

/// The models' column types, by lowercase class name and column.
fn models(
    app: Laravel,
    project: &Project,
    codebase: &CodebaseMetadata,
) -> HashMap<String, HashMap<String, TUnion>> {
    let mut scan = Scan::default();
    for file in project
        .analysed
        .iter()
        .filter_map(|p| read(p, FileType::Host))
    {
        let arena = LocalArena::new();
        let program = parse_file(&arena, &file);
        let names = NameResolver::new(&arena).resolve(program);
        let parse_casts_method = app.parse_casts_method;
        Collector {
            scan: &mut scan,
            names: &names,
            codebase,
            parse_casts_method,
        }
        .walk_program(program, &mut ());
    }
    let model_key = |class: &str| model(&scan, codebase, class).map(|m| (m.table, m.key));
    let mut schema = SchemaAggregator::new(codebase, &model_key);
    // MigrationHelper: database/migrations, recursively, in file name order
    let mut migrations: Vec<PathBuf> = Vec::new();
    php_files(
        &app.root.join("database/migrations"),
        &["php".to_string()],
        &mut migrations,
    );
    migrations.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
    for file in migrations.iter().filter_map(|p| read(p, FileType::Host)) {
        let arena = LocalArena::new();
        let program = parse_file(&arena, &file);
        let names = NameResolver::new(&arena).resolve(program);
        schema.add_statements(program, &names);
    }
    // ModelCastHelper::getDateType(): `Date::now()::class`; Illuminate's Carbon is reported as Carbon\Carbon
    let date_class = scan
        .date_class
        .clone()
        .filter(|c| c != "Illuminate\\Support\\Carbon");
    let date = named(date_class.as_deref().unwrap_or("Carbon\\Carbon"));
    let mut out = HashMap::new();
    for m in scan
        .models
        .keys()
        .filter_map(|key| model(&scan, codebase, key))
    {
        if codebase
            .get_class_like(m.name.as_bytes())
            .is_none_or(|c| c.flags.is_abstract())
        {
            continue;
        }
        let mut properties = HashMap::new();
        let table = schema.tables.get(&m.table);
        // getDatabaseProperty(): the primary key without a column is typed by `$keyType`
        if table.is_none_or(|t| !t.columns.contains_key(&m.key)) {
            properties.insert(m.key.clone(), column_type(&m.key_type));
        }
        for (name, column) in table.into_iter().flat_map(|t| &t.columns) {
            if has_accessor(codebase, m.name, name) {
                continue;
            }
            properties.insert(name.clone(), readable(&m, name, column, &date, codebase));
        }
        out.insert(m.name.to_ascii_lowercase(), properties);
    }
    out
}

fn named(class: &str) -> TUnion {
    get_named_object(Word::new(class.as_bytes()), None)
}

/// `TypeStringResolver` on a column's readable type.
fn column_type(ty: &str) -> TUnion {
    match ty {
        "int" => get_int(),
        "non-negative-int" => get_int_range(Some(0), None),
        "string" => get_string(),
        "bool" => get_bool(),
        "float" => get_float(),
        _ => get_mixed(),
    }
}

/// `getDatabaseProperty()`'s readable type.
fn readable(
    m: &Model,
    name: &str,
    column: &Column,
    date: &TUnion,
    codebase: &CodebaseMetadata,
) -> TUnion {
    let ty = if m.dates.contains(&name) {
        date.clone()
    } else if let Some(cast) = m.casts.get(name) {
        cast_type(cast, column_type(&column.readable_type), date, codebase)
    } else if matches!(column.readable_type.as_str(), "enum" | "set") {
        match &column.options {
            Some(options) if !options.is_empty() => TUnion::from_vec(
                options
                    .iter()
                    .flat_map(|o| {
                        get_literal_string(Word::new(o.as_bytes()))
                            .types
                            .into_owned()
                    })
                    .collect(),
            ),
            _ => get_string(),
        }
    } else {
        column_type(&column.readable_type)
    };
    if column.nullable {
        ty.as_nullable()
    } else {
        ty
    }
}

/// `ModelCastHelper::getReadableType()`.
fn cast_type(cast: &str, original: TUnion, date: &TUnion, codebase: &CodebaseMetadata) -> TUnion {
    let cast = cast.split(':').find(|p| *p != "encrypted").unwrap_or(cast);
    match cast {
        "int" | "integer" | "timestamp" => return get_int(),
        "real" | "float" | "double" => return get_float(),
        "decimal" => return get_numeric_string(),
        "string" => return get_string(),
        "bool" | "boolean" => return get_bool(),
        "object" => return named("stdClass"),
        "array" | "json" => return get_keyed_array(get_arraykey(), get_mixed()),
        "collection" => return named("Illuminate\\Support\\Collection"),
        "date" | "datetime" => return date.clone(),
        "immutable_date" | "immutable_datetime" => return named(CARBON_IMMUTABLE),
        _ => {}
    }
    let Some(class) = codebase.get_class_like(cast.as_bytes()) else {
        return original;
    };
    if class.kind.is_enum() {
        return named(class.original_name.as_str_lossy().as_ref());
    }
    // ponytail: AsArrayObject, AsCollection, AsStringable and `Castable::castUsing()` are not special-cased
    if codebase.is_instance_of(
        cast.as_bytes(),
        b"Illuminate\\Contracts\\Database\\Eloquent\\CastsAttributes",
    ) {
        return codebase
            .get_declaring_method(cast.as_bytes(), b"get")
            .and_then(|m| m.return_type_metadata.as_ref())
            .map_or_else(get_mixed, |t| t.type_union.clone());
    }
    if codebase.is_instance_of(
        cast.as_bytes(),
        b"Illuminate\\Contracts\\Database\\Eloquent\\CastsInboundAttributes",
    ) {
        return original;
    }
    get_mixed()
}

/// `ModelPropertyHelper::hasAccessor(strictGenerics: false)`.
fn has_accessor(codebase: &CodebaseMetadata, class: &str, property: &str) -> bool {
    let Some(method) = codebase.get_declaring_method(class.as_bytes(), camel(property).as_bytes())
    else {
        let getter = format!("get{}Attribute", studly(property));
        return codebase
            .get_declaring_method(class.as_bytes(), getter.as_bytes())
            .is_some();
    };
    let visibility = method.method_metadata.as_ref().map(|m| m.visibility);
    if visibility.is_none_or(|v| v.is_public() || v.is_private()) {
        return false;
    }
    method
        .return_type_metadata
        .as_ref()
        .is_some_and(|t| t.type_union.types.iter().all(|a| is_named(a, ATTRIBUTE)))
}

fn is_named(atomic: &TAtomic, class: &str) -> bool {
    matches!(atomic, TAtomic::Object(TObject::Named(n)) if n.name.as_bytes().eq_ignore_ascii_case(class.as_bytes()))
}

/// The hook: `$model->column` and `$model?->column` read the column's type, unless the class declares the property
/// or a `@property` tag (PHPStan's own reflection comes before Larastan's extension).
struct ModelAttributes {
    models: HashMap<String, HashMap<String, TUnion>>,
}

impl Provider for ModelAttributes {
    fn meta() -> &'static ProviderMeta {
        static META: ProviderMeta = ProviderMeta::new(
            "pun-laravel-model-attributes",
            "Laravel model attributes",
            "Model columns from migrations",
        );
        &META
    }
}

impl ExpressionHook for ModelAttributes {
    fn after_expression(
        &self,
        expr: &Expression<'_>,
        context: &mut HookContext<'_, '_>,
    ) -> HookResult<()> {
        let (object, property, nullsafe) = match expr {
            Expression::Access(Access::Property(p)) => (p.object, &p.property, false),
            Expression::Access(Access::NullSafeProperty(p)) => (p.object, &p.property, true),
            _ => return Ok(()),
        };
        let ClassLikeMemberSelector::Identifier(property) = property else {
            return Ok(());
        };
        let Some(object_type) = context.get_expression_type(object) else {
            return Ok(());
        };
        let name = String::from_utf8_lossy(property.value);
        let dollar = format!("${name}");
        let mut types = Vec::new();
        for atomic in object_type.types.iter() {
            let ty = match atomic {
                TAtomic::Null if nullsafe => get_null(),
                TAtomic::Object(TObject::Named(n)) => {
                    let class = n.name.as_bytes();
                    let declared = context
                        .codebase()
                        .get_declaring_property(class, dollar.as_bytes())
                        .is_some()
                        || context.codebase().get_class_like(class).is_some_and(|c| {
                            c.magic_property_ids
                                .contains_key(&Word::new(dollar.as_bytes()))
                        });
                    let column = self
                        .models
                        .get(&n.name.as_str_lossy().to_ascii_lowercase())
                        .and_then(|m| m.get(name.as_ref()));
                    match column {
                        Some(ty) if !declared => ty.clone(),
                        _ => return Ok(()),
                    }
                }
                _ => return Ok(()),
            };
            types.extend(ty.types.into_owned());
        }
        if !types.is_empty() {
            context.set_expression_type(expr, TUnion::from_vec(types));
        }
        Ok(())
    }
}
