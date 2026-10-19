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

const COMMAND: &[u8] = b"illuminate\\console\\command";
const SYMFONY_COMMAND: &[u8] = b"symfony\\component\\console\\command\\command";
const ISOLATABLE: &[u8] = b"illuminate\\contracts\\console\\isolatable";
const INPUT_ARGUMENT: &[u8] = b"Symfony\\Component\\Console\\Input\\InputArgument";
const INPUT_OPTION: &[u8] = b"Symfony\\Component\\Console\\Input\\InputOption";

/// A registered command and its merged definition.
#[derive(Clone, Debug)]
pub(crate) struct Command {
    /// The class, lowercase.
    pub(crate) class: String,
    /// Its keys in `Application::all()`: its name and aliases, or `#[AsCommand]`'s name for a lazy command.
    pub(crate) names: Vec<String>,
    definition: Definition,
}

impl Command {
    /// `getDefinition()->hasArgument()`.
    pub(crate) fn has_argument(&self, name: &str) -> bool {
        self.definition.arguments.contains(name)
    }

    /// `getDefinition()->hasOption() || getDefinition()->hasShortcut()`.
    pub(crate) fn has_option(&self, name: &str) -> bool {
        self.definition.options.contains(name) || self.definition.shortcuts.contains(name)
    }
}

/// An InputDefinition's argument names, option names and option shortcuts.
#[derive(Clone, Debug, Default)]
struct Definition {
    arguments: HashSet<String>,
    options: HashSet<String>,
    shortcuts: HashSet<String>,
}

impl Definition {
    /// `InputDefinition::addOption()`: the name without `--`, and each shortcut (InputOption's constructor).
    fn add_option(&mut self, name: &str, shortcut: Option<&str>) -> Option<()> {
        let name = name.strip_prefix("--").unwrap_or(name);
        if name.is_empty() {
            return None;
        }
        self.options.insert(name.to_string());
        if let Some(shortcut) = shortcut.filter(|s| !s.is_empty()) {
            // preg_split('{(\|)-?}', ltrim($shortcut, '-')), without the empty ones
            let shortcuts: Vec<String> = lossy(ltrim_chars(shortcut.as_bytes(), b"-"))
                .split('|')
                .map(|s| s.strip_prefix('-').unwrap_or(s).to_string())
                .filter(|s| !s.is_empty())
                .collect();
            if shortcuts.is_empty() {
                return None;
            }
            self.shortcuts.extend(shortcuts);
        }
        Some(())
    }

    /// Symfony's `Application::getDefaultInputDefinition()` and Laravel's `--env`.
    fn application() -> Definition {
        let mut d = Definition::default();
        d.arguments.insert("command".into());
        for (name, shortcut) in [
            ("help", Some("h")),
            ("silent", None),
            ("quiet", Some("q")),
            ("verbose", Some("v|vv|vvv")),
            ("version", Some("V")),
            ("ansi", None),
            ("no-interaction", Some("n")),
            ("env", None),
        ] {
            d.add_option(name, shortcut);
        }
        d
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
            definition.arguments.extend(arguments);
            for (option, shortcut) in options {
                definition.add_option(&option, shortcut.as_deref())?;
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
        definition.add_option("isolated", None);
    }
    let application = Definition::application();
    definition.arguments.extend(application.arguments);
    definition.options.extend(application.options);
    definition.shortcuts.extend(application.shortcuts);
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
        definition,
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

/// A signature's name, argument names, and option names with their shortcuts.
type Signature = (String, Vec<String>, Vec<(String, Option<String>)>);

/// Laravel's `Parser::parse()`: the name, the arguments' names and the options' names and shortcuts. `None` when
/// the signature has no name (Laravel throws).
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
        let token = lossy(&m.ok()?[1]);
        match re(&OPTION, r"/^-{2,}(.*)/")
            .captures(token.as_bytes())
            .ok()?
        {
            Some(o) => options.push(parse_option(&lossy(&o[1]))),
            None => arguments.push(parse_argument(&token)),
        }
    }
    Some((lossy(name.as_bytes()), arguments, options))
}

fn lossy(b: &[u8]) -> String {
    String::from_utf8_lossy(b).into_owned()
}

/// `trim($s, $chars)`.
fn trim<'a>(s: &'a str, chars: &str) -> &'a str {
    std::str::from_utf8(ltrim_chars(
        rtrim_chars(s.as_bytes(), chars.as_bytes()),
        chars.as_bytes(),
    ))
    .unwrap_or_default()
}

/// The first group of `pattern` in `token`.
fn group(
    cell: &'static OnceLock<pcre2::bytes::Regex>,
    pattern: &str,
    token: &str,
) -> Option<String> {
    let re = cell.get_or_init(|| preg_compile(pattern).expect("a valid pattern"));
    re.captures(token.as_bytes()).ok()?.map(|c| lossy(&c[1]))
}

static ARRAY_DEFAULT: OnceLock<pcre2::bytes::Regex> = OnceLock::new();
static DEFAULT: OnceLock<pcre2::bytes::Regex> = OnceLock::new();

/// `Parser::parseArgument()`: the argument's name.
fn parse_argument(token: &str) -> String {
    let token = extract_description(token);
    if token.ends_with("?*") {
        trim(&token, "?*").to_string()
    } else if token.ends_with('*') {
        trim(&token, "*").to_string()
    } else if token.ends_with('?') {
        trim(&token, "?").to_string()
    } else {
        group(&ARRAY_DEFAULT, r"/(.+)\=\*(.+)/", &token)
            .or_else(|| group(&DEFAULT, r"/(.+)\=(.+)/", &token))
            .unwrap_or(token)
    }
}

/// `Parser::parseOption()`: the option's name and shortcut.
fn parse_option(token: &str) -> (String, Option<String>) {
    static SHORTCUT: OnceLock<pcre2::bytes::Regex> = OnceLock::new();
    let token = extract_description(token);
    let (shortcut, token) = match split(&SHORTCUT, r"/\s*\|\s*/", &token) {
        Some((shortcut, token)) => (Some(shortcut), token),
        None => (None, token),
    };
    let name = if token.ends_with('=') {
        trim(&token, "=").to_string()
    } else if token.ends_with("=*") {
        trim(&token, "=*").to_string()
    } else {
        group(&ARRAY_DEFAULT, r"/(.+)\=\*(.+)/", &token)
            .or_else(|| group(&DEFAULT, r"/(.+)\=(.+)/", &token))
            .unwrap_or(token)
    };
    (name, shortcut)
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
        lossy(&s.as_bytes()[..m.start()]),
        lossy(&s.as_bytes()[m.end()..]),
    ))
}

/// `Parser::extractDescription()`: the token without its ` : description`.
fn extract_description(token: &str) -> String {
    static DESCRIPTION: OnceLock<pcre2::bytes::Regex> = OnceLock::new();
    let trimmed = lossy(pun_tokens::php_compat::trim(token.as_bytes()));
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
            add(definition, options, &parameters)?;
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
            Argument::Positional(p) if p.ellipsis.is_none() => Some((None, p.value)),
            Argument::Named(n) => Some((Some(n.name.value), n.value)),
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

/// `addArgument($name, ...)` or `addOption($name, $shortcut, ...)` with constant parameters.
fn add(
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
    if !option {
        definition.arguments.insert(name);
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
    definition.add_option(&name, shortcut.as_deref())
}

fn string(e: &Expression) -> Option<String> {
    match e {
        Expression::Literal(Literal::String(s)) => s.value.map(lossy),
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
                        add(self.definition, option, &p)
                    }
                    Some(p) => add(self.definition, method == b"addoption", p),
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
