//! ConsoleApplicationResolver::findCommands(): the commands of the app's Artisan application, each with the
//! InputDefinition `mergeApplicationDefinition()` leaves. Larastan boots the app and asks it; Pun takes the concrete
//! `Illuminate\Console\Command` subclasses of the analysed files (Laravel discovers commands in app/Console/Commands)
//! and builds what their constructors would: Laravel's `Command::__construct()` with `Parser::parse()` of the
//! signature, else `$name`, `getArguments()` and `getOptions()`; then `configure()`'s `addArgument()` and
//! `addOption()`, and Isolatable's `--isolated`. A command whose definition is not constant (a computed signature, a
//! `getArguments()` that does not return an array literal) is left out, as if it were not registered.

use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::OnceLock;

use mago_allocator::LocalArena;
use mago_codex::metadata::CodebaseMetadata;
use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::symbol::SymbolKind;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::array::TArray;
use mago_codex::ttype::union::TUnion;
use mago_database::file::{FileId, FileType};
use mago_names::ResolvedNames;
use mago_names::resolver::NameResolver;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::parser::parse_file;
use mago_syntax::walker::MutWalker;
use mago_word::word;
use pun_tokens::php_compat::{ltrim_chars, preg_compile, rtrim_chars};

use crate::Project;
use crate::codebase::read;
use crate::text;

const COMMAND: &[u8] = b"illuminate\\console\\command";
const SYMFONY_COMMAND: &[u8] = b"symfony\\component\\console\\command\\command";
const ISOLATABLE: &[u8] = b"illuminate\\contracts\\console\\isolatable";
const INPUT_ARGUMENT: &[u8] = b"Symfony\\Component\\Console\\Input\\InputArgument";
const INPUT_OPTION: &[u8] = b"Symfony\\Component\\Console\\Input\\InputOption";

/// A registered command, with its own definition and the one `mergeApplicationDefinition()` leaves.
#[derive(Clone, Debug)]
pub(crate) struct Command {
    /// The class, lowercase.
    pub(crate) class: String,
    /// Its keys in `Application::all()`: its name and aliases, or `#[AsCommand]`'s name for a lazy command.
    pub(crate) names: Vec<String>,
    /// `getDefinition()` before `mergeApplicationDefinition()`.
    pub(crate) own: Definition,
    /// `getDefinition()` after it.
    pub(crate) merged: Definition,
}

impl Command {
    /// `getDefinition()->hasArgument()`.
    pub(crate) fn has_argument(&self, name: &str) -> bool {
        self.merged.argument(name).is_some()
    }

    /// `getDefinition()->hasOption() || getDefinition()->hasShortcut()`.
    pub(crate) fn has_option(&self, name: &str) -> bool {
        self.merged.option(name).is_some() || self.merged.shortcuts.contains_key(name)
    }
}

/// A constant value as PHP has it, `Unknown` for an expression Pun does not evaluate.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    /// An array's keys (a string or an int) and values.
    Array(Vec<(Key, Value)>),
    Unknown,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Key {
    Int(i64),
    String(String),
}

/// An InputArgument: its name, its mode (`None` when not constant) and its default as `setDefault()` leaves it.
#[derive(Clone, Debug)]
pub(crate) struct InputArgument {
    pub(crate) name: String,
    pub(crate) mode: Option<u32>,
    pub(crate) default: Value,
}

impl InputArgument {
    pub(crate) const REQUIRED: u32 = 1;
    pub(crate) const IS_ARRAY: u32 = 4;

    /// `new InputArgument($name, $mode, '', $default)`: a missing mode is OPTIONAL, an array's `null` default `[]`.
    fn new(name: String, mode: Option<Option<u32>>, default: Value) -> InputArgument {
        let mode = mode.map_or(Some(2), |m| m);
        let default = match (mode, default) {
            (Some(m), Value::Null) if m & Self::IS_ARRAY != 0 => Value::Array(Vec::new()),
            (_, d) => d,
        };
        InputArgument { name, mode, default }
    }
}

/// An InputOption: its name, its mode (`None` when not constant) and its default as `setDefault()` leaves it.
#[derive(Clone, Debug)]
pub(crate) struct InputOption {
    pub(crate) name: String,
    pub(crate) mode: Option<u32>,
    pub(crate) default: Value,
}

impl InputOption {
    pub(crate) const VALUE_NONE: u32 = 1;
    pub(crate) const VALUE_REQUIRED: u32 = 2;
    pub(crate) const VALUE_OPTIONAL: u32 = 4;
    pub(crate) const VALUE_IS_ARRAY: u32 = 8;
    pub(crate) const VALUE_NEGATABLE: u32 = 16;

    /// InputOption's constructor: a mode that neither requires nor allows a value is VALUE_NONE, an array's `null`
    /// default is `[]`, and an option that takes no value (nor is negatable) defaults to `false`.
    fn new(name: String, mode: Option<Option<u32>>, default: Value) -> InputOption {
        let mode = mode.map_or(Some(Self::VALUE_NONE), |m| {
            m.map(|m| {
                if m & (Self::VALUE_REQUIRED | Self::VALUE_OPTIONAL) != 0 { m } else { m | Self::VALUE_NONE }
            })
        });
        let default = match mode {
            Some(m) if m & Self::VALUE_IS_ARRAY != 0 && default == Value::Null => Value::Array(Vec::new()),
            Some(m) if m & (Self::VALUE_REQUIRED | Self::VALUE_OPTIONAL | Self::VALUE_NEGATABLE) == 0 => {
                Value::Bool(false)
            }
            _ => default,
        };
        InputOption { name, mode, default }
    }
}

/// An InputDefinition's arguments and options in their order, and the option each shortcut names.
#[derive(Clone, Debug, Default)]
pub(crate) struct Definition {
    pub(crate) arguments: Vec<InputArgument>,
    pub(crate) options: Vec<InputOption>,
    pub(crate) shortcuts: HashMap<String, String>,
}

impl Definition {
    pub(crate) fn argument(&self, name: &str) -> Option<&InputArgument> {
        self.arguments.iter().find(|a| a.name == name)
    }

    pub(crate) fn option(&self, name: &str) -> Option<&InputOption> {
        self.options.iter().find(|o| o.name == name)
    }

    fn add_argument(&mut self, argument: InputArgument) {
        match self.arguments.iter_mut().find(|a| a.name == argument.name) {
            Some(a) => *a = argument,
            None => self.arguments.push(argument),
        }
    }

    /// `InputDefinition::addOption()`: the name without `--`, and each shortcut (InputOption's constructor).
    fn add_option(&mut self, name: &str, shortcut: Option<&str>, mode: Option<Option<u32>>, default: Value) -> Option<()> {
        let name = name.strip_prefix("--").unwrap_or(name);
        if name.is_empty() {
            return None;
        }
        if let Some(shortcut) = shortcut.filter(|s| !s.is_empty()) {
            // preg_split('{(\|)-?}', ltrim($shortcut, '-')), without the empty ones
            let shortcuts: Vec<String> = text(ltrim_chars(shortcut.as_bytes(), b"-"))
                .split('|')
                .map(|s| s.strip_prefix('-').unwrap_or(s).to_string())
                .filter(|s| !s.is_empty())
                .collect();
            if shortcuts.is_empty() {
                return None;
            }
            for s in shortcuts {
                self.shortcuts.insert(s, name.to_string());
            }
        }
        let option = InputOption::new(name.to_string(), mode, default);
        match self.options.iter_mut().find(|o| o.name == name) {
            Some(o) => *o = option,
            None => self.options.push(option),
        }
        Some(())
    }

    /// Symfony's `Application::getDefaultInputDefinition()` and Laravel's `--env`.
    fn application() -> Definition {
        let mut d = Definition::default();
        d.add_argument(InputArgument::new("command".into(), Some(Some(InputArgument::REQUIRED)), Value::Null));
        for (name, shortcut, mode) in [
            ("help", Some("h"), InputOption::VALUE_NONE),
            ("silent", None, InputOption::VALUE_NONE),
            ("quiet", Some("q"), InputOption::VALUE_NONE),
            ("verbose", Some("v|vv|vvv"), InputOption::VALUE_NONE),
            ("version", Some("V"), InputOption::VALUE_NONE),
            ("ansi", None, InputOption::VALUE_NEGATABLE),
            ("no-interaction", Some("n"), InputOption::VALUE_NONE),
            ("env", None, InputOption::VALUE_OPTIONAL),
        ] {
            d.add_option(name, shortcut, Some(Some(mode)), Value::Null);
        }
        d
    }

    /// `mergeApplicationDefinition()`: the application's arguments first, then its options after the command's.
    fn merge(&self) -> Definition {
        let application = Definition::application();
        let mut merged = Definition { arguments: application.arguments, ..Definition::default() };
        for a in &self.arguments {
            merged.add_argument(a.clone());
        }
        merged.options = self.options.clone();
        merged.shortcuts = self.shortcuts.clone();
        for o in application.options {
            if merged.option(&o.name).is_none() {
                merged.options.push(o);
            }
        }
        for (s, o) in application.shortcuts {
            merged.shortcuts.entry(s).or_insert(o);
        }
        merged
    }
}

/// The app's commands: every concrete command class declared in an analysed file.
pub(crate) fn commands(project: &Project, codebase: &CodebaseMetadata) -> Vec<Command> {
    let files: HashMap<FileId, &Path> = project
        .analysed
        .iter()
        .chain(&project.scanned)
        .chain(project.autoloaded.iter().map(|a| &a.path))
        .map(|p| (FileId::new(p.to_string_lossy().as_bytes()), p.as_path()))
        .collect();
    let analysed: HashSet<FileId> = project
        .analysed
        .iter()
        .map(|p| FileId::new(p.to_string_lossy().as_bytes()))
        .collect();
    let mut classes: Vec<_> = codebase
        .get_all_descendants(COMMAND)
        .into_iter()
        .filter_map(|c| codebase.get_class_like(c.as_bytes()))
        .filter(|k| {
            k.kind == SymbolKind::Class
                && !k.flags.is_abstract()
                && analysed.contains(&k.span.file_id)
        })
        .collect();
    classes.sort_by_key(|k| k.name.as_bytes().to_ascii_lowercase());
    classes
        .into_iter()
        .filter_map(|k| command(codebase, &files, k))
        .collect()
}

/// What `new $class()` and `mergeApplicationDefinition()` make of one command class.
fn command(
    codebase: &CodebaseMetadata,
    files: &HashMap<FileId, &Path>,
    k: &ClassLikeMetadata,
) -> Option<Command> {
    let class = k.name.as_bytes();
    let attribute = |name: &[u8]| {
        k.attributes
            .iter()
            .find(|a| a.name.as_bytes().eq_ignore_ascii_case(name))
    };
    // configureFromAttributes()
    let mut signature = string_property(codebase, class, b"$signature")?;
    let mut aliases = match codebase
        .get_declaring_property(class, b"$aliases")
        .and_then(|p| p.default_type_metadata.as_ref())
    {
        Some(t) if !t.type_union.is_null() => strings(&t.type_union)?,
        _ => Vec::new(),
    };
    if let Some(a) = attribute(b"illuminate\\console\\attributes\\signature") {
        signature = Some(single_string(argument(a, 0, b"signature")?)?);
        if let Some(t) = argument(a, 1, b"aliases").filter(|t| !t.is_null()) {
            aliases = strings(t)?;
        }
    }
    if let Some(a) = attribute(b"illuminate\\console\\attributes\\aliases") {
        aliases = strings(argument(a, 0, b"aliases")?)?;
    }
    let mut definition = Definition::default();
    let name = match signature {
        Some(signature) => {
            let (name, arguments, options) = parse(&signature)?;
            for argument in arguments {
                definition.add_argument(argument);
            }
            for (option, shortcut, mode, default) in options {
                definition.add_option(&option, shortcut.as_deref(), Some(Some(mode)), default)?;
            }
            name
        }
        None => {
            // specifyParameters()
            inputs(codebase, files, k, b"getarguments", false, &mut definition)?;
            inputs(codebase, files, k, b"getoptions", true, &mut definition)?;
            match string_property(codebase, class, b"$name")? {
                Some(name) => name,
                // Symfony's getDefaultName()
                None => attribute(b"symfony\\component\\console\\attribute\\ascommand")
                    .and_then(|a| argument(a, 0, b"name"))
                    .and_then(single_string)?,
            }
        }
    };
    configure(codebase, files, k, &mut definition)?;
    if codebase.class_implements(class, ISOLATABLE) {
        // `new InputOption('isolated', null, InputOption::VALUE_OPTIONAL, ..., false)`
        definition.add_option("isolated", None, Some(Some(InputOption::VALUE_OPTIONAL)), Value::Bool(false));
    }
    // Application::resolve(): a command with #[AsCommand] is lazy, under the attribute's names
    let names = match attribute(b"symfony\\component\\console\\attribute\\ascommand") {
        Some(a) => single_string(argument(a, 0, b"name")?)?
            .split('|')
            .map(str::to_string)
            .collect(),
        None => std::iter::once(name).chain(aliases).collect(),
    };
    Some(Command {
        class: String::from_utf8_lossy(class).to_ascii_lowercase(),
        names,
        merged: definition.merge(),
        own: definition,
    })
}

/// A property's default as a string: `Some(None)` without one or for `null`, `None` when it is no constant string.
fn string_property(
    codebase: &CodebaseMetadata,
    class: &[u8],
    property: &[u8],
) -> Option<Option<String>> {
    match codebase
        .get_declaring_property(class, property)
        .and_then(|p| p.default_type_metadata.as_ref())
    {
        Some(t) if !t.type_union.is_null() => single_string(&t.type_union).map(Some),
        _ => Some(None),
    }
}

/// An attribute's argument by position or name.
fn argument<'a>(
    a: &'a mago_codex::metadata::attribute::AttributeMetadata,
    position: usize,
    name: &[u8],
) -> Option<&'a TUnion> {
    a.arguments
        .iter()
        .find(|arg| arg.name.is_some_and(|n| n.as_bytes() == name))
        .or_else(|| a.arguments.get(position).filter(|arg| arg.name.is_none()))?
        .value_type
        .as_ref()
}

fn single_string(t: &TUnion) -> Option<String> {
    t.get_single_literal_string_value()
        .map(|v| String::from_utf8_lossy(v).into_owned())
}

/// The strings of a constant array type, or a constant string as `(array)` casts it.
fn strings(t: &TUnion) -> Option<Vec<String>> {
    if let Some(s) = single_string(t) {
        return Some(vec![s]);
    }
    let items: Vec<&TUnion> = match t.types.as_ref() {
        [TAtomic::Array(TArray::List(l))] => l
            .known_elements
            .iter()
            .flat_map(|m| m.values().map(|(_, v)| v))
            .collect(),
        [TAtomic::Array(TArray::Keyed(k))] if k.parameters.is_none() => k
            .known_items
            .iter()
            .flat_map(|m| m.values().map(|(_, v)| v))
            .collect(),
        _ => return None,
    };
    items.into_iter().map(single_string).collect()
}

/// A signature's name, arguments, and options with their shortcuts, modes and defaults.
type Signature = (String, Vec<InputArgument>, Vec<(String, Option<String>, u32, Value)>);

/// Laravel's `Parser::parse()`: the name, the arguments and the options. `None` when the signature has no name
/// (Laravel throws).
fn parse(expression: &str) -> Option<Signature> {
    static NAME: OnceLock<pcre2::bytes::Regex> = OnceLock::new();
    static PARAMETER: OnceLock<pcre2::bytes::Regex> = OnceLock::new();
    static OPTION: OnceLock<pcre2::bytes::Regex> = OnceLock::new();
    let re = |cell: &'static OnceLock<pcre2::bytes::Regex>, pattern: &str| {
        cell.get_or_init(|| preg_compile(pattern).expect("a valid pattern"))
    };
    let bytes = expression.as_bytes();
    let name = re(&NAME, r"/[^\s]+/").find(bytes).ok()??;
    let mut arguments = Vec::new();
    let mut options = Vec::new();
    for m in re(&PARAMETER, r"/\{\s*(.*?)\s*\}/").captures_iter(bytes) {
        let token = text(&m.ok()?[1]);
        match re(&OPTION, r"/^-{2,}(.*)/")
            .captures(token.as_bytes())
            .ok()?
        {
            Some(o) => options.push(parse_option(&text(&o[1]))),
            None => arguments.push(parse_argument(&token)),
        }
    }
    Some((text(name.as_bytes()), arguments, options))
}

/// `trim($s, $chars)`.
fn trim<'a>(s: &'a str, chars: &str) -> &'a str {
    std::str::from_utf8(ltrim_chars(
        rtrim_chars(s.as_bytes(), chars.as_bytes()),
        chars.as_bytes(),
    ))
    .unwrap_or_default()
}

static ARRAY_DEFAULT: OnceLock<pcre2::bytes::Regex> = OnceLock::new();
static DEFAULT: OnceLock<pcre2::bytes::Regex> = OnceLock::new();

/// `preg_split('/,\s?/', $s)`.
fn split_list(s: &str) -> Value {
    static COMMA: OnceLock<pcre2::bytes::Regex> = OnceLock::new();
    let re = COMMA.get_or_init(|| preg_compile(r"/,\s?/").expect("a valid pattern"));
    let bytes = s.as_bytes();
    let mut values = Vec::new();
    let mut start = 0;
    for m in re.find_iter(bytes).flatten() {
        values.push(text(&bytes[start..m.start()]));
        start = m.end();
    }
    values.push(text(&bytes[start..]));
    Value::Array(values.into_iter().enumerate().map(|(i, v)| (Key::Int(i as i64), Value::String(v))).collect())
}

/// A default `(.+)=*(.+)` (an array, split at commas) or `(.+)=(.+)`, with the name before it.
fn with_default(token: &str) -> Option<(String, Value, bool)> {
    let captures = |cell: &'static OnceLock<pcre2::bytes::Regex>, pattern: &str| {
        let re = cell.get_or_init(|| preg_compile(pattern).expect("a valid pattern"));
        re.captures(token.as_bytes()).ok().flatten().map(|c| (text(&c[1]), text(&c[2])))
    };
    if let Some((name, default)) = captures(&ARRAY_DEFAULT, r"/(.+)\=\*(.+)/") {
        return Some((name, split_list(&default), true));
    }
    captures(&DEFAULT, r"/(.+)\=(.+)/").map(|(name, default)| (name, Value::String(default), false))
}

/// `Parser::parseArgument()`.
fn parse_argument(token: &str) -> InputArgument {
    let token = extract_description(token);
    let (name, mode, default) = if token.ends_with("?*") {
        (trim(&token, "?*").to_string(), InputArgument::IS_ARRAY, Value::Null)
    } else if token.ends_with('*') {
        (trim(&token, "*").to_string(), InputArgument::IS_ARRAY | InputArgument::REQUIRED, Value::Null)
    } else if token.ends_with('?') {
        (trim(&token, "?").to_string(), 2, Value::Null)
    } else {
        match with_default(&token) {
            Some((name, default, true)) => (name, InputArgument::IS_ARRAY, default),
            Some((name, default, false)) => (name, 2, default),
            None => (token, InputArgument::REQUIRED, Value::Null),
        }
    };
    InputArgument::new(name, Some(Some(mode)), default)
}

/// `Parser::parseOption()`: the option's name, shortcut, mode and default.
fn parse_option(token: &str) -> (String, Option<String>, u32, Value) {
    static SHORTCUT: OnceLock<pcre2::bytes::Regex> = OnceLock::new();
    let token = extract_description(token);
    let (shortcut, token) = match split(&SHORTCUT, r"/\s*\|\s*/", &token) {
        Some((shortcut, token)) => (Some(shortcut), token),
        None => (None, token),
    };
    let optional = InputOption::VALUE_OPTIONAL;
    let (name, mode, default) = if token.ends_with('=') {
        (trim(&token, "=").to_string(), optional, Value::Null)
    } else if token.ends_with("=*") {
        (trim(&token, "=*").to_string(), optional | InputOption::VALUE_IS_ARRAY, Value::Null)
    } else {
        match with_default(&token) {
            Some((name, default, true)) => (name, optional | InputOption::VALUE_IS_ARRAY, default),
            Some((name, default, false)) => (name, optional, default),
            None => (token, InputOption::VALUE_NONE, Value::Null),
        }
    };
    (name, shortcut, mode, default)
}

/// `preg_split($pattern, $s, 2)` when it splits.
fn split(
    cell: &'static OnceLock<pcre2::bytes::Regex>,
    pattern: &str,
    s: &str,
) -> Option<(String, String)> {
    let re = cell.get_or_init(|| preg_compile(pattern).expect("a valid pattern"));
    let m = re.find(s.as_bytes()).ok()??;
    Some((
        text(&s.as_bytes()[..m.start()]),
        text(&s.as_bytes()[m.end()..]),
    ))
}

/// `Parser::extractDescription()`: the token without its ` : description`.
fn extract_description(token: &str) -> String {
    static DESCRIPTION: OnceLock<pcre2::bytes::Regex> = OnceLock::new();
    let trimmed = text(pun_tokens::php_compat::trim(token.as_bytes()));
    match split(&DESCRIPTION, r"/\s+:\s+/", &trimmed) {
        Some((token, _)) => token,
        None => token.to_string(),
    }
}

/// The method's declaration, when another class than `base` declares it: `None` for the base's own.
fn declared<'c>(
    codebase: &'c CodebaseMetadata,
    k: &ClassLikeMetadata,
    method: &[u8],
    base: &[u8],
) -> Option<(&'c ClassLikeMetadata, Span)> {
    let id = k.declaring_method_ids.get(&word(method))?;
    if id.get_class_name().as_bytes().eq_ignore_ascii_case(base) {
        return None;
    }
    let declaring = codebase.get_class_like(id.get_class_name().as_bytes())?;
    let m = codebase.get_method(declaring.name.as_bytes(), method)?;
    Some((declaring, m.name_span?))
}

/// Runs `f` on the body of the concrete method named at `name_span` in its file, with the file's names. `None` when the file is
/// not one Pun reads.
fn with_method<R>(
    files: &HashMap<FileId, &Path>,
    name_span: Span,
    f: impl FnOnce(&Block, &ResolvedNames) -> Option<R>,
) -> Option<R> {
    let file = read(files.get(&name_span.file_id)?, FileType::Host).ok()??;
    let arena = LocalArena::new();
    let program = parse_file(&arena, &file);
    let names = NameResolver::new(&arena).resolve(program);
    let mut finder = Find(name_span, None);
    finder.walk_program(program, &mut ());
    f(finder.1?, &names)
}

/// The body of the concrete method named at the span.
struct Find<'ast, 'arena>(Span, Option<&'ast Block<'arena>>);

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Find<'ast, 'arena> {
    fn walk_in_method(&mut self, m: &'ast Method<'arena>, _: &mut ()) {
        if m.name.span == self.0
            && let MethodBody::Concrete(body) = &m.body
        {
            self.1 = Some(body);
        }
    }
}

/// `getArguments()` or `getOptions()` (`options`) as `specifyParameters()` adds them: the method must return an
/// array literal of argument lists and `new InputArgument(...)` / `new InputOption(...)`.
fn inputs(
    codebase: &CodebaseMetadata,
    files: &HashMap<FileId, &Path>,
    k: &ClassLikeMetadata,
    method: &[u8],
    options: bool,
    definition: &mut Definition,
) -> Option<()> {
    let Some((_, span)) = declared(codebase, k, method, COMMAND) else {
        return Some(());
    };
    with_method(files, span, |body, names| {
        let value = body.statements.iter().find_map(|s| match s {
            Statement::Return(r) => Some(r.value),
            _ => None,
        })??;
        let elements = match value {
            Expression::Array(a) => &a.elements,
            Expression::LegacyArray(a) => &a.elements,
            _ => return None,
        };
        for element in elements.iter() {
            let ArrayElement::Value(v) = element else {
                return None;
            };
            let parameters = match v.value {
                Expression::Array(a) => array_parameters(&a.elements)?,
                Expression::LegacyArray(a) => array_parameters(&a.elements)?,
                e => instantiation(names, e)?.1,
            };
            add(names, definition, options, &parameters)?;
        }
        Some(())
    })
}

/// The parameters an array spreads into a call (`...$arguments`): its values, and its string keys as names.
fn array_parameters<'a, 'arena>(
    elements: &'a TokenSeparatedSequence<'arena, ArrayElement<'arena>>,
) -> Option<Parameters<'a, 'arena>> {
    elements
        .iter()
        .map(|e| match e {
            ArrayElement::Value(v) => Some((None, v.value)),
            ArrayElement::KeyValue(kv) => match kv.key {
                Expression::Literal(Literal::String(s)) => Some((Some(s.value?), kv.value)),
                _ => None,
            },
            _ => None,
        })
        .collect()
}

type Parameters<'a, 'arena> = Vec<(Option<&'a [u8]>, &'a Expression<'arena>)>;

fn call_parameters<'a, 'arena>(list: &'a ArgumentList<'arena>) -> Option<Parameters<'a, 'arena>> {
    list.arguments
        .iter()
        .map(|a| match a {
            mago_syntax::cst::Argument::Positional(p) if p.ellipsis.is_none() => Some((None, p.value)),
            mago_syntax::cst::Argument::Named(n) => Some((Some(n.name.value), n.value)),
            _ => None,
        })
        .collect()
}

/// `new InputArgument(...)` (false) or `new InputOption(...)` (true), with its parameters.
fn instantiation<'a, 'arena>(
    names: &ResolvedNames,
    e: &'a Expression<'arena>,
) -> Option<(bool, Parameters<'a, 'arena>)> {
    let Expression::Instantiation(i) = e else {
        return None;
    };
    let Expression::Identifier(id) = i.class else {
        return None;
    };
    let class = names.get(&id.span());
    let option = if class.eq_ignore_ascii_case(INPUT_OPTION) {
        true
    } else if class.eq_ignore_ascii_case(INPUT_ARGUMENT) {
        false
    } else {
        return None;
    };
    Some((option, call_parameters(i.argument_list.as_ref()?)?))
}

/// `addArgument($name, $mode, $description, $default)` or `addOption($name, $shortcut, $mode, $description,
/// $default)` with a constant name and shortcut.
fn add(
    names: &ResolvedNames,
    definition: &mut Definition,
    option: bool,
    parameters: &[(Option<&[u8]>, &Expression)],
) -> Option<()> {
    let parameter = |position: usize, name: &[u8]| {
        parameters
            .iter()
            .find(|(n, _)| *n == Some(name))
            .or_else(|| parameters.get(position).filter(|(n, _)| n.is_none()))
            .map(|(_, e)| *e)
    };
    let name = string(parameter(0, b"name")?)?;
    let mode = |position: usize| parameter(position, b"mode").map(|e| mode(names, e));
    let default = |position: usize| parameter(position, b"default").map_or(Value::Null, |e| value(e));
    if !option {
        definition.add_argument(InputArgument::new(name, mode(1), default(3)));
        return Some(());
    }
    let shortcut = match parameter(1, b"shortcut") {
        None | Some(Expression::Literal(Literal::Null(_) | Literal::False(_))) => None,
        Some(Expression::Array(a)) => Some(
            array_parameters(&a.elements)?
                .into_iter()
                .map(|(_, e)| string(e))
                .collect::<Option<Vec<_>>>()?
                .join("|"),
        ),
        Some(e) => Some(string(e)?),
    };
    definition.add_option(&name, shortcut.as_deref(), mode(2), default(4))
}

/// A mode: an int, InputArgument's or InputOption's constants, and `|` of them.
fn mode(names: &ResolvedNames, e: &Expression) -> Option<u32> {
    match e.unparenthesized() {
        Expression::Literal(Literal::Integer(i)) => u32::try_from(i.value?).ok(),
        Expression::Literal(Literal::Null(_)) => None,
        Expression::Binary(b) if matches!(b.operator, BinaryOperator::BitwiseOr(_)) => {
            Some(mode(names, b.lhs)? | mode(names, b.rhs)?)
        }
        Expression::Access(Access::ClassConstant(a)) => {
            let Expression::Identifier(id) = a.class else {
                return None;
            };
            let ClassLikeConstantSelector::Identifier(constant) = &a.constant else {
                return None;
            };
            let class = names.get(&id.span());
            let constants: &[(&[u8], u32)] = if class.eq_ignore_ascii_case(INPUT_ARGUMENT) {
                &[(b"REQUIRED", 1), (b"OPTIONAL", 2), (b"IS_ARRAY", 4)]
            } else if class.eq_ignore_ascii_case(INPUT_OPTION) {
                &[
                    (b"VALUE_NONE", 1),
                    (b"VALUE_REQUIRED", 2),
                    (b"VALUE_OPTIONAL", 4),
                    (b"VALUE_IS_ARRAY", 8),
                    (b"VALUE_NEGATABLE", 16),
                ]
            } else {
                return None;
            };
            constants.iter().find(|(n, _)| *n == constant.value).map(|(_, v)| *v)
        }
        _ => None,
    }
}

/// A literal's value.
fn value(e: &Expression) -> Value {
    match e.unparenthesized() {
        Expression::Literal(Literal::Null(_)) => Value::Null,
        Expression::Literal(Literal::True(_)) => Value::Bool(true),
        Expression::Literal(Literal::False(_)) => Value::Bool(false),
        Expression::Literal(Literal::Integer(i)) => i.value.and_then(|v| i64::try_from(v).ok()).map_or(Value::Unknown, Value::Int),
        Expression::Literal(Literal::Float(f)) => Value::Float(f.value.into_inner()),
        Expression::Literal(Literal::String(s)) => s.value.map_or(Value::Unknown, |v| Value::String(text(v))),
        Expression::Array(a) => array_value(&a.elements),
        Expression::LegacyArray(a) => array_value(&a.elements),
        _ => Value::Unknown,
    }
}

fn array_value(elements: &TokenSeparatedSequence<ArrayElement>) -> Value {
    let mut items: Vec<(Key, Value)> = Vec::new();
    let mut next = 0;
    for element in elements.iter() {
        let (key, v) = match element {
            ArrayElement::Value(v) => (Key::Int(next), v.value),
            ArrayElement::KeyValue(kv) => match value(kv.key) {
                Value::Int(i) => (Key::Int(i), kv.value),
                Value::String(s) => (Key::String(s), kv.value),
                _ => return Value::Unknown,
            },
            _ => return Value::Unknown,
        };
        if let Key::Int(i) = key {
            next = next.max(i + 1);
        }
        let v = value(v);
        match items.iter_mut().find(|(k, _)| *k == key) {
            Some(item) => item.1 = v,
            None => items.push((key, v)),
        }
    }
    Value::Array(items)
}

fn string(e: &Expression) -> Option<String> {
    match e {
        Expression::Literal(Literal::String(s)) => s.value.map(text),
        _ => None,
    }
}

/// `configure()`'s `addArgument()` and `addOption()` calls, following `parent::configure()`.
fn configure(
    codebase: &CodebaseMetadata,
    files: &HashMap<FileId, &Path>,
    k: &ClassLikeMetadata,
    definition: &mut Definition,
) -> Option<()> {
    let Some((declaring, span)) = declared(codebase, k, b"configure", SYMFONY_COMMAND) else {
        return Some(());
    };
    let calls_parent = with_method(files, span, |body, names| {
        let mut calls = Configure {
            names,
            definition,
            known: true,
            parent: false,
        };
        calls.walk_block(body, &mut ());
        calls.known.then_some(calls.parent)
    })?;
    match declaring.direct_parent_class {
        Some(parent) if calls_parent => configure(
            codebase,
            files,
            codebase.get_class_like(parent.as_bytes())?,
            definition,
        ),
        _ => Some(()),
    }
}

struct Configure<'n, 'd, 'arena> {
    names: &'n ResolvedNames<'arena>,
    definition: &'d mut Definition,
    known: bool,
    parent: bool,
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Configure<'_, '_, 'arena> {
    fn walk_in_method_call(&mut self, call: &'ast MethodCall<'arena>, _: &mut ()) {
        let ClassLikeMemberSelector::Identifier(m) = &call.method else {
            return;
        };
        let method = m.value.to_ascii_lowercase();
        let added = match method.as_slice() {
            b"addargument" | b"addoption" => {
                let parameters = call_parameters(&call.argument_list);
                match parameters.as_deref() {
                    // InputDefinition::addArgument(new InputArgument(...)) and addOption(new InputOption(...))
                    Some([(None, e)]) if let Some((option, p)) = instantiation(self.names, e) => {
                        add(self.names, self.definition, option, &p)
                    }
                    Some(p) => add(self.names, self.definition, method == b"addoption", p),
                    None => None,
                }
            }
            b"setname" | b"setdefinition" | b"setaliases" => None,
            _ => Some(()),
        };
        self.known &= added.is_some();
    }

    fn walk_in_static_method_call(&mut self, call: &'ast StaticMethodCall<'arena>, _: &mut ()) {
        self.parent |= matches!(call.class, Expression::Parent(_))
            && matches!(&call.method, ClassLikeMemberSelector::Identifier(m) if m.value.eq_ignore_ascii_case(b"configure"));
    }
}
