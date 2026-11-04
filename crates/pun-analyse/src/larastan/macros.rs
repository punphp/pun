//! The macros MacroMethodsClassReflectionExtension finds in a Macroable class's static `$macros`, read from the
//! code that registers them instead (Pun never runs PHP): `X::macro('name', $definition)` calls in PHPStan's
//! `bootstrapFiles`, the files they include, the app's service providers (`app/Providers`) and the framework's
//! FoundationServiceProvider.
//!
//! A macro's return type is what Larastan reflects of its definition: a closure's native return type
//! (ClosureTypeFactory::fromClosureObject(), mixed without one), a function's (`'trim'`), or a method's
//! (`'Class::method'`, `[$this, 'method']`, `[X::class, 'method']`, getNativeMethod()).
//!
//! ponytail: macros that other vendor service providers register and `mixin()` are not seen.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use mago_allocator::LocalArena;
use mago_codex::metadata::CodebaseMetadata;
use mago_codex::ttype::get_mixed;
use mago_codex::ttype::union::TUnion;
use mago_database::file::FileType;
use mago_names::ResolvedNames;
use mago_span::HasSpan;
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;

use crate::codebase::{parse, read};
use crate::larastan::app::App;

const MACROABLE: &[u8] = b"Illuminate\\Support\\Traits\\Macroable";
const BUILDER: &[u8] = b"Illuminate\\Database\\Eloquent\\Builder";
const QUERY_BUILDER: &[u8] = b"Illuminate\\Database\\Query\\Builder";

/// The macros' return types, by the lowercase name of the class whose `$macros` holds them, then by name; and the
/// framework facades' accessors (`getFacadeAccessor()`), by lowercase facade class.
#[derive(Clone, Debug, Default)]
pub(crate) struct Macros {
    macros: HashMap<String, HashMap<String, Macro>>,
    accessors: HashMap<String, String>,
}

/// A macro as Larastan reflects it.
#[derive(Clone, Debug)]
pub(crate) struct Macro {
    pub(crate) ty: TUnion,
    /// The declaring class and name of a macro that is no static method: a callable string's `Macro` (on the class
    /// that holds it) or a non-static native method. A closure's `Macro` is static.
    pub(crate) instance: Option<(String, String)>,
}

/// A registration's definition, resolved once every file is read.
enum Definition {
    /// A closure's native return type.
    Closure(Option<TUnion>),
    Function(String),
    /// A class and method name.
    Method(String, String),
}

impl Macros {
    pub(crate) fn load(files: &[PathBuf], root: &Path, codebase: &CodebaseMetadata, app: &App) -> Macros {
        let mut files: Vec<PathBuf> = files.to_vec();
        let php = ["php".to_string()];
        files.extend(crate::php_files(&root.join("app/Providers"), &php));
        let framework = root.join("vendor/laravel/framework/src/Illuminate");
        files.extend(crate::php_files(&framework.join("Support/Facades"), &php));
        // the framework's own macros (`Request::validate()`)
        files.push(framework.join("Foundation/Providers/FoundationServiceProvider.php"));
        let mut seen = HashSet::new();
        let mut read_files = Reader::default();
        while let Some(path) = files.pop() {
            if !seen.insert(path.clone()) {
                continue;
            }
            files.extend(read_files.read(&path));
        }
        let mut macros = Macros { accessors: std::mem::take(&mut read_files.accessors), ..Macros::default() };
        for (class, name, definition) in read_files.registrations {
            // `Facade::macro()` forwards to the facade's root
            let Some(home) = home(codebase, &class).or_else(|| {
                let root = macros.facade_root(app, &class)?;
                // CacheManager::__call() forwards `macro()` to its default store, a Repository
                let root = if root.eq_ignore_ascii_case("Illuminate\\Cache\\CacheManager") {
                    "Illuminate\\Cache\\Repository".to_string()
                } else {
                    root
                };
                home(codebase, &root)
            }) else {
                continue;
            };
            let holder = codebase.get_class_like(home.as_bytes()).map_or_else(
                || class.clone(),
                |k| String::from_utf8_lossy(k.original_name.as_bytes()).into_owned(),
            );
            let return_type = |t: Option<&mago_codex::metadata::ttype::TypeMetadata>| {
                t.map_or_else(get_mixed, |t| t.type_union.clone())
            };
            let found = match definition {
                Definition::Closure(ty) => Macro { ty: ty.unwrap_or_else(get_mixed), instance: None },
                Definition::Function(function) => {
                    let Some(f) = codebase.get_function(function.trim_start_matches('\\').as_bytes()) else {
                        continue;
                    };
                    Macro { ty: return_type(f.return_type_metadata.as_ref()), instance: Some((holder, name.clone())) }
                }
                Definition::Method(class, method) => {
                    let local = read_files.methods.get(&(class.to_ascii_lowercase(), method.to_ascii_lowercase()));
                    match (codebase.get_method(class.as_bytes(), method.as_bytes()), local) {
                        (Some(m), _) => {
                            let declaring = m
                                .method_metadata
                                .as_ref()
                                .filter(|mm| !mm.is_static)
                                .and_then(|_| codebase.get_declaring_method_class(class.as_bytes(), method.as_bytes()))
                                .and_then(|d| codebase.get_class_like(d.as_bytes()));
                            Macro {
                                ty: return_type(m.return_type_metadata.as_ref()),
                                instance: declaring.map(|d| {
                                    let name = String::from_utf8_lossy(m.original_name.as_bytes()).into_owned();
                                    (String::from_utf8_lossy(d.original_name.as_bytes()).into_owned(), name)
                                }),
                            }
                        }
                        (None, Some(local)) => Macro { ty: local.ty.clone().unwrap_or_else(get_mixed), instance: local.instance.clone() },
                        (None, None) => continue,
                    }
                }
            };
            // a later registration replaces an earlier one, as `static::$macros[$name] =` does; Mago asks for a
            // method by its lowercase name
            macros.macros.entry(home).or_default().insert(name.to_ascii_lowercase(), found);
        }
        macros
    }

    /// The `$macros` homes MacroMethodsClassReflectionExtension::hasMethod() reads for a class, and those the calls
    /// ModelForwardsCallsExtension, RelationForwardsCallsExtension and EloquentBuilderForwardsCallsExtension forward
    /// reach.
    pub(crate) fn homes(&self, codebase: &CodebaseMetadata, app: &App, class: &[u8]) -> Vec<String> {
        const MODEL: &[u8] = b"Illuminate\\Database\\Eloquent\\Model";
        const RELATION: &[u8] = b"Illuminate\\Database\\Eloquent\\Relations\\Relation";
        const FACADE: &[u8] = b"Illuminate\\Support\\Facades\\Facade";
        let builder = String::from_utf8_lossy(BUILDER).into_owned();
        let query_builder = String::from_utf8_lossy(QUERY_BUILDER).into_owned();
        let Some(k) = codebase.get_class_like(class) else { return Vec::new() };
        let name = String::from_utf8_lossy(k.original_name.as_bytes()).into_owned();
        let is = |parent: &[u8]| class.eq_ignore_ascii_case(parent) || codebase.is_instance_of(class, parent);
        let mut homes: Vec<String> = Vec::new();
        if k.kind.is_interface() && name.starts_with("Illuminate\\Contracts") {
            if let Some(concrete) = app.resolve(&name)
                && codebase.class_uses_trait(concrete.as_bytes(), MACROABLE)
            {
                homes.extend(home(codebase, concrete));
            }
        } else if let Some(h) = home(codebase, &name) {
            homes.push(h);
            // RelationForwardsCallsExtension: a relation's own macros, then its related model's builder's
            if is(BUILDER) || is(RELATION) {
                homes.extend(home(codebase, &builder));
                homes.extend(home(codebase, &query_builder));
            }
        } else if is(FACADE) {
            let classes: Vec<String> = match name.to_ascii_lowercase().as_str() {
                "illuminate\\support\\facades\\auth" => {
                    vec!["Illuminate\\Auth\\SessionGuard".into(), "Illuminate\\Auth\\RequestGuard".into()]
                }
                "illuminate\\support\\facades\\cache" => {
                    vec!["Illuminate\\Cache\\CacheManager".into(), "Illuminate\\Cache\\Repository".into()]
                }
                _ => self.facade_root(app, &name).into_iter().collect(),
            };
            homes.extend(classes.iter().filter_map(|c| home(codebase, c)));
        } else if is(MODEL) || is(RELATION) {
            homes.extend(home(codebase, &builder));
            homes.extend(home(codebase, &query_builder));
        }
        homes
    }

    /// The macro `name` in the `$macros` of one of `homes` (as [`home`] names them).
    pub(crate) fn get(&self, homes: &[String], name: &str) -> Option<&Macro> {
        let name = name.to_ascii_lowercase();
        homes.iter().find_map(|h| self.macros.get(h)?.get(&name))
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.macros.is_empty()
    }

    /// The class of a facade's root (`getFacadeRoot()`): what the container makes for its accessor, or the class
    /// the accessor names.
    pub(crate) fn facade_root(&self, app: &App, facade: &str) -> Option<String> {
        let accessor = self.accessors.get(&facade.trim_start_matches('\\').to_ascii_lowercase())?;
        Some(app.resolve(accessor).unwrap_or(accessor).to_string())
    }
}

/// Whether a class uses Macroable or is an Eloquent or query builder (which declare their own `$macros`).
fn macroable(codebase: &CodebaseMetadata, class: &[u8]) -> bool {
    codebase.class_uses_trait(class, MACROABLE)
        || [BUILDER, QUERY_BUILDER]
            .iter()
            .any(|b| class.eq_ignore_ascii_case(b) || codebase.is_instance_of(class, b))
}

/// The lowercase name of the class whose static `$macros` a Macroable class reads: the nearest class of its
/// ancestry that is macroable while its parent is not. `None` for a class that is not macroable.
pub(crate) fn home(codebase: &CodebaseMetadata, class: &str) -> Option<String> {
    let mut current = codebase.get_class_like(class.trim_start_matches('\\').as_bytes())?;
    if !macroable(codebase, current.name.as_bytes()) {
        return None;
    }
    while let Some(parent) = current.direct_parent_class.and_then(|p| codebase.get_class_like(p.as_bytes()))
        && macroable(codebase, parent.name.as_bytes())
    {
        current = parent;
    }
    Some(String::from_utf8_lossy(current.name.as_bytes()).to_ascii_lowercase())
}

struct Local {
    ty: Option<TUnion>,
    instance: Option<(String, String)>,
}

#[derive(Default)]
struct Reader {
    /// (class, macro name, definition), in registration order.
    registrations: Vec<(String, String, Definition)>,
    /// The read files' methods (their native return types, and for an instance method its class and name), by
    /// lowercase class and method name.
    methods: HashMap<(String, String), Local>,
    /// The facades' accessors, by lowercase class.
    accessors: HashMap<String, String>,
}

impl Reader {
    /// Reads one file; returns the files it includes.
    fn read(&mut self, path: &Path) -> Vec<PathBuf> {
        let Some(file) = read(path, FileType::Host).ok().flatten() else {
            return Vec::new();
        };
        let arena = LocalArena::new();
        let (program, names) = parse(&arena, &file);
        let mut walker = Walker {
            reader: self,
            names: &names,
            dir: path.parent().unwrap_or(Path::new("")).to_path_buf(),
            namespace: String::new(),
            class: None,
            includes: Vec::new(),
        };
        walker.walk_program(program, &mut ());
        walker.includes
    }
}

struct Walker<'r, 'n, 'arena> {
    reader: &'r mut Reader,
    names: &'n ResolvedNames<'arena>,
    dir: PathBuf,
    namespace: String,
    /// The enclosing class's name.
    class: Option<String>,
    includes: Vec<PathBuf>,
}

impl Walker<'_, '_, '_> {
    fn name(&self, id: &Identifier) -> String {
        String::from_utf8_lossy(self.names.get(&id.span())).into_owned()
    }

    /// An included file's path: a string, or `__DIR__` followed by one.
    fn include(&mut self, e: &Expression) {
        let path = match e {
            Expression::Literal(Literal::String(s)) => s.value.map(|v| self.dir.join(String::from_utf8_lossy(v).as_ref())),
            Expression::Binary(b) if b.operator.is_concatenation() => match (b.lhs, b.rhs) {
                (Expression::MagicConstant(MagicConstant::Directory(_)), Expression::Literal(Literal::String(s))) => s
                    .value
                    .map(|v| PathBuf::from(format!("{}{}", self.dir.display(), String::from_utf8_lossy(v)))),
                _ => None,
            },
            _ => None,
        };
        self.includes.extend(path.filter(|p| p.is_file()));
    }

    /// A class named by an expression: `X::class`, `self::class`, `static::class`, or a string.
    fn class_of(&self, e: &Expression) -> Option<String> {
        match e {
            Expression::Literal(Literal::String(s)) => Some(String::from_utf8_lossy(s.value?).into_owned()),
            Expression::Access(Access::ClassConstant(c)) => match (&c.class, &c.constant) {
                (class, ClassLikeConstantSelector::Identifier(i)) if i.value.eq_ignore_ascii_case(b"class") => {
                    match class {
                        Expression::Self_(_) | Expression::Static(_) => self.class.clone(),
                        Expression::Identifier(id) => Some(self.name(id)),
                        _ => None,
                    }
                }
                _ => None,
            },
            _ => None,
        }
    }

    /// A string definition: its constant value, `X::class . '::method'` included.
    fn string(&self, e: &Expression) -> Option<String> {
        match e {
            Expression::Literal(Literal::String(s)) => Some(String::from_utf8_lossy(s.value?).into_owned()),
            Expression::Binary(b) if b.operator.is_concatenation() => {
                let left = self.string(b.lhs).or_else(|| self.class_of(b.lhs))?;
                Some(left + &self.string(b.rhs)?)
            }
            _ => None,
        }
    }

    fn definition(&self, e: &Expression) -> Option<Definition> {
        match e {
            Expression::Closure(c) => {
                Some(Definition::Closure(c.return_type_hint.as_ref().and_then(|h| self.hint(&h.hint))))
            }
            Expression::ArrowFunction(f) => {
                Some(Definition::Closure(f.return_type_hint.as_ref().and_then(|h| self.hint(&h.hint))))
            }
            Expression::Array(a) => self.callable_array(&a.elements),
            Expression::LegacyArray(a) => self.callable_array(&a.elements),
            e => {
                let s = self.string(e)?;
                Some(match s.split_once("::") {
                    Some((class, method)) => Definition::Method(class.to_string(), method.to_string()),
                    None => Definition::Function(s),
                })
            }
        }
    }

    /// `[$this, 'method']` or `[X::class, 'method']`.
    fn callable_array(&self, elements: &TokenSeparatedSequence<ArrayElement>) -> Option<Definition> {
        let [ArrayElement::Value(object), ArrayElement::Value(method)] = elements.as_slice() else {
            return None;
        };
        let class = match object.value {
            Expression::Variable(Variable::Direct(v)) if v.name == b"$this" => self.class.clone()?,
            e => self.class_of(e)?,
        };
        let Expression::Literal(Literal::String(m)) = method.value else {
            return None;
        };
        Some(Definition::Method(class, String::from_utf8_lossy(m.value?).into_owned()))
    }

    /// A native type hint's type (TypehintHelper::decideTypeFromReflection()); `None` for `self`, `static` and
    /// `parent`.
    fn hint(&self, hint: &Hint) -> Option<TUnion> {
        crate::larastan::config::type_from_text(self.hint_text(hint)?.as_bytes())
    }

    fn hint_text(&self, hint: &Hint) -> Option<String> {
        Some(match hint {
            Hint::Identifier(id) => format!("\\{}", self.name(id)),
            Hint::Parenthesized(p) => format!("({})", self.hint_text(p.hint)?),
            Hint::Nullable(n) => format!("?{}", self.hint_text(n.hint)?),
            Hint::Union(u) => format!("{}|{}", self.hint_text(u.left)?, self.hint_text(u.right)?),
            Hint::Intersection(i) => format!("{}&{}", self.hint_text(i.left)?, self.hint_text(i.right)?),
            Hint::Static(_) | Hint::Self_(_) | Hint::Parent(_) => return None,
            Hint::Null(k) | Hint::True(k) | Hint::False(k) | Hint::Array(k) | Hint::Callable(k) => {
                String::from_utf8_lossy(k.value).to_ascii_lowercase()
            }
            Hint::Void(i)
            | Hint::Never(i)
            | Hint::Float(i)
            | Hint::Bool(i)
            | Hint::Integer(i)
            | Hint::String(i)
            | Hint::Object(i)
            | Hint::Mixed(i)
            | Hint::Iterable(i) => String::from_utf8_lossy(i.value).to_ascii_lowercase(),
        })
    }
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Walker<'_, '_, 'arena> {
    fn walk_in_namespace(&mut self, n: &'ast Namespace<'arena>, _: &mut ()) {
        self.namespace = n.name.as_ref().map_or_else(String::new, |n| String::from_utf8_lossy(n.value()).into_owned());
    }

    fn walk_in_class(&mut self, c: &'ast Class<'arena>, _: &mut ()) {
        let name = String::from_utf8_lossy(c.name.value).into_owned();
        self.class = Some(if self.namespace.is_empty() { name } else { format!("{}\\{name}", self.namespace) });
    }

    fn walk_out_class(&mut self, _: &'ast Class<'arena>, _: &mut ()) {
        self.class = None;
    }

    fn walk_in_method(&mut self, m: &'ast Method<'arena>, _: &mut ()) {
        if let Some(class) = &self.class {
            let ty = m.return_type_hint.as_ref().and_then(|h| self.hint(&h.hint));
            let key = (class.to_ascii_lowercase(), String::from_utf8_lossy(m.name.value).to_ascii_lowercase());
            if m.name.value.eq_ignore_ascii_case(b"getFacadeAccessor")
                && let MethodBody::Concrete(body) = &m.body
                && let [Statement::Return(Return { value: Some(value), .. })] = body.statements.as_slice()
                && let Some(accessor) = self.class_of(value)
            {
                self.reader.accessors.insert(class.to_ascii_lowercase(), accessor);
            }
            let is_static = m.modifiers.iter().any(|m| matches!(m, Modifier::Static(_)));
            let instance = (!is_static).then(|| (class.clone(), String::from_utf8_lossy(m.name.value).into_owned()));
            self.reader.methods.insert(key, Local { ty, instance });
        }
    }

    fn walk_in_include_construct(&mut self, c: &'ast IncludeConstruct<'arena>, _: &mut ()) {
        self.include(c.value);
    }

    fn walk_in_include_once_construct(&mut self, c: &'ast IncludeOnceConstruct<'arena>, _: &mut ()) {
        self.include(c.value);
    }

    fn walk_in_require_construct(&mut self, c: &'ast RequireConstruct<'arena>, _: &mut ()) {
        self.include(c.value);
    }

    fn walk_in_require_once_construct(&mut self, c: &'ast RequireOnceConstruct<'arena>, _: &mut ()) {
        self.include(c.value);
    }

    fn walk_in_static_method_call(&mut self, call: &'ast StaticMethodCall<'arena>, _: &mut ()) {
        let ClassLikeMemberSelector::Identifier(method) = &call.method else { return };
        let Expression::Identifier(class) = call.class else { return };
        if !method.value.eq_ignore_ascii_case(b"macro") {
            return;
        }
        let arguments = &call.argument_list.arguments;
        let (Some(name), Some(definition)) = (arguments.get(0), arguments.get(1)) else { return };
        let Expression::Literal(Literal::String(name)) = name.value() else { return };
        let Some(name) = name.value else { return };
        let Some(definition) = self.definition(definition.value()) else { return };
        let class = self.name(class);
        self.reader.registrations.push((class, String::from_utf8_lossy(name).into_owned(), definition));
    }
}
