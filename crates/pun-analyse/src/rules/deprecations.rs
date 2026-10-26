//! phpstan-deprecation-rules (rules.neon, on when the config sets `deprecationRulesInstalled`): the
//! RestrictedDeprecated*UsageExtension classes the restricted-usage rules run, DeprecatedScopeHelper with
//! DefaultDeprecatedScopeResolver, FetchingDeprecatedConstRule and CallWithDeprecatedIniOptionRule (bleedingEdge).
//! A symbol is deprecated by its `@deprecated` tag (methods, properties and class constants inherit it through
//! PhpDocInheritanceResolver); built-in functions and methods by PHPStan's reflection, without a description.

use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::metadata::function_like::FunctionLikeMetadata;
use mago_codex::symbol::SymbolKind;
use mago_codex::visibility::Visibility;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_word::word;

use super::Checker;
use super::declarations::doc_before;
use super::members::Method;

/// A `@deprecated` tag: `Some(description)` (phpdoc-parser's DeprecatedTagValueNode, `None` when empty).
pub type Deprecation = Option<String>;

/// What a PHPDoc says: `None` without one (`getResolvedPhpDoc()` is null), else its `@deprecated` and whether it has
/// `@not-deprecated`.
struct Resolved {
    deprecated: Option<Deprecation>,
    not_deprecated: bool,
}

/// The lines of a PHPDoc's body, after the leading `*` and the one space after it.
fn lines(doc: &[u8]) -> Vec<String> {
    let body = doc.strip_prefix(b"/**").unwrap_or(doc);
    let body = body.strip_suffix(b"*/").unwrap_or(body);
    String::from_utf8_lossy(body)
        .split('\n')
        .map(|line| {
            let line = line.trim_start();
            match line.strip_prefix('*') {
                Some(rest) => rest.strip_prefix(' ').unwrap_or(rest).trim_end().to_string(),
                None => line.trim_end().to_string(),
            }
        })
        .collect()
}

fn tag_name(line: &str) -> Option<&str> {
    let rest = line.trim_start().strip_prefix('@')?;
    let len = rest
        .find(|ch: char| !(ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '\\' | ':')))
        .unwrap_or(rest.len());
    Some(&rest[..len])
}

/// PhpDocNodeResolver::resolveDeprecatedTag() and resolveIsNotDeprecated() of a PHPDoc.
fn resolve(doc: &[u8]) -> Resolved {
    let lines = lines(doc);
    let not_deprecated = lines.iter().any(|l| tag_name(l) == Some("not-deprecated"));
    let deprecated = lines.iter().position(|l| tag_name(l) == Some("deprecated")).map(|at| {
        let first = lines[at].trim_start()["@deprecated".len()..].to_string();
        let rest = lines[at + 1..].iter().take_while(|l| tag_name(l).is_none());
        let text = std::iter::once(first).chain(rest.cloned()).collect::<Vec<_>>().join("\n");
        let text = text.trim();
        (!text.is_empty()).then(|| text.to_string())
    });
    Resolved {
        deprecated,
        not_deprecated,
    }
}

impl Checker<'_, '_> {
    /// The PHPDoc right before `span` in its file.
    fn doc_of(&self, span: Span) -> Option<Resolved> {
        let contents = self.file_contents(span.file_id)?;
        doc_before(&contents, span.start.offset as usize).map(resolve)
    }

    /// The PHPDoc of the constant declared at `span` (an item of a `const` statement, or a `define()` call).
    fn constant_doc(&self, span: Span) -> Option<Resolved> {
        let contents = self.file_contents(span.file_id)?;
        let starts = super::overriding_property::declaration_starts(span.file_id, &contents);
        let start = starts.get(&span.start.offset).copied().unwrap_or(span.start.offset);
        doc_before(&contents, start as usize).map(resolve)
    }

    /// `ClassReflection::isDeprecated()` and `getDeprecatedDescription()` (the class's own PHPDoc).
    pub(super) fn class_deprecation(&self, k: &ClassLikeMetadata) -> Option<Deprecation> {
        self.doc_of(k.span)?.deprecated
    }

    /// A method's merged `@deprecated` (PhpDocInheritanceResolver::resolvePhpDocForMethod()).
    pub(super) fn method_deprecation(&self, m: &Method) -> Option<Deprecation> {
        match m {
            Method::User { declaring, f, .. } => self.resolved_method(declaring, f, 0)?.deprecated,
            Method::Builtin { f, .. } => f.deprecated.then_some(None),
        }
    }

    fn resolved_method(&self, declaring: &ClassLikeMetadata, f: &FunctionLikeMetadata, depth: usize) -> Option<Resolved> {
        let own = self.doc_of(f.span);
        if own.as_ref().is_some_and(|r| r.deprecated.is_some() || r.not_deprecated) || depth > 64 {
            return own;
        }
        let name = f.original_name.as_bytes();
        let get = |n: &mago_word::Word| self.codebase.get_class_like(n.as_bytes());
        // an ancestor's method: a user one's merged PHPDoc, a built-in's deprecation
        let inherited = |m: Method| match m {
            Method::User { declaring, f, .. } => self.resolved_method(declaring, f, depth + 1),
            Method::Builtin { f, .. } => Some(Resolved { deprecated: f.deprecated.then_some(None), not_deprecated: false }),
        };
        let parent = declaring.direct_parent_class.as_ref().and_then(get).and_then(|p| {
            let m = super::members::method(self, p, name)?;
            let visible = match &m {
                Method::User { f, .. } => f.method_metadata.as_ref()?.visibility != Visibility::Private,
                Method::Builtin { .. } => !name.eq_ignore_ascii_case(b"__construct"),
            };
            visible.then(|| inherited(m)).flatten()
        });
        let interfaces = || {
            declaring
                .direct_parent_interfaces
                .iter()
                .filter_map(get)
                .find_map(|i| inherited(super::members::method(self, i, name)?))
        };
        let traits = || {
            declaring.used_traits.iter().filter_map(get).find_map(|t| {
                let f = self.codebase.get_method(t.name.as_bytes(), &name.to_ascii_lowercase())?;
                f.method_metadata.as_ref().filter(|m| m.is_abstract)?;
                self.doc_of(f.span)
            })
        };
        merge(own, parent.or_else(interfaces).or_else(traits))
    }

    /// A property's merged `@deprecated` (resolvePhpDocForProperty()): `declaring` declares it.
    pub(super) fn property_deprecation(&self, declaring: &ClassLikeMetadata, name: &[u8]) -> Option<Deprecation> {
        self.resolved_property(declaring, name, 0)?.deprecated
    }

    fn resolved_property(&self, declaring: &ClassLikeMetadata, name: &[u8], depth: usize) -> Option<Resolved> {
        let (_, _, p) = self.declared_property(declaring, name)?;
        let own = self.declaration(p).and_then(|(span, _)| self.doc_of(span));
        if own.as_ref().is_some_and(|r| r.deprecated.is_some() || r.not_deprecated) || depth > 64 {
            return own;
        }
        let get = |n: &mago_word::Word| self.codebase.get_class_like(n.as_bytes());
        let inherited = declaring
            .direct_parent_class
            .iter()
            .chain(declaring.direct_parent_interfaces.iter())
            .filter_map(get)
            .find_map(|parent| {
                let (d, _, p) = self.declared_property(parent, name)?;
                (p.read_visibility != Visibility::Private)
                    .then(|| self.resolved_property(d, name, depth + 1))
                    .flatten()
            });
        merge(own, inherited)
    }

    /// A class constant's merged `@deprecated` (resolveConstantPhpDocFromParentClass()).
    pub(super) fn constant_deprecation(&self, declaring: &ClassLikeMetadata, name: &[u8]) -> Option<Deprecation> {
        self.resolved_constant(declaring, name, 0)?.deprecated
    }

    fn resolved_constant(&self, declaring: &ClassLikeMetadata, name: &[u8], depth: usize) -> Option<Resolved> {
        let own = match declaring.constants.get(&word(name)) {
            Some(constant) => self.constant_doc(constant.span),
            None => self.doc_of(declaring.enum_cases.get(&word(name))?.span),
        };
        if own.as_ref().is_some_and(|r| r.deprecated.is_some() || r.not_deprecated) || depth > 64 {
            return own;
        }
        let get = |n: &mago_word::Word| self.codebase.get_class_like(n.as_bytes());
        let inherited = declaring
            .direct_parent_class
            .iter()
            .chain(declaring.direct_parent_interfaces.iter())
            .filter_map(get)
            .find_map(|parent| {
                let constant = parent.constants.get(&word(name))?;
                (constant.visibility != Visibility::Private)
                    .then(|| {
                        let owner = super::constants::declaring(self, parent, constant);
                        self.resolved_constant(owner, name, depth + 1)
                    })
                    .flatten()
            });
        merge(own, inherited)
    }

    /// `FunctionReflection::isDeprecated()` of a user function (its own PHPDoc).
    pub(super) fn function_deprecation(&self, f: &FunctionLikeMetadata) -> Option<Deprecation> {
        self.doc_of(f.span)?.deprecated
    }

    /// DeprecatedScopeHelper::isScopeDeprecated() with DefaultDeprecatedScopeResolver.
    pub(super) fn scope_deprecated(&self) -> bool {
        self.class_scope().is_some_and(|k| self.class_deprecation(k).is_some())
            || self
                .in_trait()
                .and_then(|t| self.codebase.get_class_like(t.as_bytes()))
                .is_some_and(|t| self.class_deprecation(t).is_some())
            || self.functions.last().is_some_and(|f| f.deprecated)
    }

    /// Whether the function or method being entered is deprecated (`getFunction()->isDeprecated()`).
    pub(super) fn entered_deprecated(&self, name: &str, method: bool, span: Span) -> bool {
        if !self.deprecations {
            return false;
        }
        if !method {
            return self.doc_of(span).is_some_and(|r| r.deprecated.is_some());
        }
        self.class_scope()
            .and_then(|k| super::members::method(self, k, name.as_bytes()))
            .is_some_and(|m| self.method_deprecation(&m).is_some())
    }
}

/// ResolvedPhpDocBlock::merge()'s mergeDeprecatedTags(): `own`, else `parent`'s.
fn merge(own: Option<Resolved>, parent: Option<Resolved>) -> Option<Resolved> {
    let Some(parent) = parent else {
        return own;
    };
    let own = own.unwrap_or(Resolved {
        deprecated: None,
        not_deprecated: false,
    });
    Some(Resolved {
        deprecated: own.deprecated.or(if own.not_deprecated { None } else { parent.deprecated }),
        not_deprecated: own.not_deprecated,
    })
}

/// `getClassTypeDescription()`.
pub(super) fn type_description(k: &ClassLikeMetadata) -> &'static str {
    match k.kind {
        SymbolKind::Interface => "Interface",
        SymbolKind::Trait => "Trait",
        SymbolKind::Enum => "Enum",
        _ => "Class",
    }
}

fn name(k: &ClassLikeMetadata) -> String {
    String::from_utf8_lossy(k.original_name.as_bytes()).into_owned()
}

fn with(message: String, description: &Deprecation) -> String {
    match description {
        None => format!("{message}."),
        Some(d) => format!("{message}:\n{d}"),
    }
}

/// RestrictedDeprecatedFunctionUsageExtension.
pub(super) fn function(c: &Checker, name: &str, deprecation: Option<Deprecation>) -> Option<(String, String)> {
    if !c.deprecations || c.scope_deprecated() {
        return None;
    }
    let description = deprecation?;
    Some((with(format!("Call to deprecated function {name}()"), &description), "function.deprecated".to_string()))
}

/// RestrictedDeprecatedMethodUsageExtension, `declaring` standing in for the method's declaring class when given.
pub(super) fn method(c: &Checker, m: &Method, declaring: Option<&ClassLikeMetadata>) -> Option<(String, String)> {
    if !c.deprecations || c.scope_deprecated() {
        return None;
    }
    let kind = if m.is_static() { "staticMethod" } else { "method" };
    let method = m.name();
    let (class, class_deprecation, description) = match (m, declaring) {
        (Method::User { declaring: own, .. }, d) => {
            let k = d.unwrap_or(own);
            (name(k), c.class_deprecation(k), type_description(k))
        }
        (Method::Builtin { .. }, Some(k)) => (name(k), c.class_deprecation(k), type_description(k)),
        (Method::Builtin { declaring: b, .. }, None) => (b.name.clone(), None, builtin_description(b)),
    };
    if let Some(d) = class_deprecation {
        let message = format!("Call to method {method}() of deprecated {} {class}", description.to_ascii_lowercase());
        return Some((with(message, &d), format!("{kind}.deprecated{description}")));
    }
    let d = c.method_deprecation(m)?;
    if method.eq_ignore_ascii_case("__toString") {
        let message = format!("Casting class {class} to string is deprecated.");
        return Some((
            match d {
                None => message,
                Some(d) => format!("{message}:\n{d}"),
            },
            "class.toStringDeprecated".to_string(),
        ));
    }
    let message = format!("Call to deprecated method {method}() of {} {class}", description.to_ascii_lowercase());
    Some((with(message, &d), format!("{kind}.deprecated")))
}

fn builtin_description(b: &pun_phpstan::symbols::Class) -> &'static str {
    [(b.is_interface, "Interface"), (b.is_trait, "Trait"), (b.is_enum, "Enum")]
        .into_iter()
        .find_map(|(is, description)| is.then_some(description))
        .unwrap_or("Class")
}

/// RestrictedDeprecatedPropertyUsageExtension: property `$name` that `declaring` declares (`owner` standing in for
/// it when given).
pub(super) fn property(
    c: &Checker,
    declaring: &ClassLikeMetadata,
    owner: Option<&ClassLikeMetadata>,
    property: &str,
    is_static: bool,
    deprecation: Option<Deprecation>,
) -> Option<(String, String)> {
    if !c.deprecations || c.scope_deprecated() {
        return None;
    }
    let (kind, prefix) = if is_static { ("staticProperty", "static ") } else { ("property", "") };
    let k = owner.unwrap_or(declaring);
    let description = type_description(k);
    if let Some(d) = c.class_deprecation(k) {
        let message = format!(
            "Access to {prefix}property ${property} of deprecated {} {}",
            description.to_ascii_lowercase(),
            name(k)
        );
        return Some((with(message, &d), format!("{kind}.deprecated{description}")));
    }
    let d = deprecation?;
    let message = format!(
        "Access to deprecated {prefix}property ${property} of {} {}",
        description.to_ascii_lowercase(),
        name(k)
    );
    Some((with(message, &d), format!("{kind}.deprecated")))
}

/// RestrictedDeprecatedClassConstantUsageExtension: constant `constant` that `declaring` declares.
pub(super) fn class_constant(
    c: &Checker,
    declaring: &ClassLikeMetadata,
    constant: &str,
    deprecation: Option<Deprecation>,
) -> Option<(String, String)> {
    if !c.deprecations || c.scope_deprecated() {
        return None;
    }
    let description = type_description(declaring);
    if let Some(d) = c.class_deprecation(declaring) {
        let message = format!(
            "Fetching class constant {constant} of deprecated {} {}",
            description.to_ascii_lowercase(),
            name(declaring)
        );
        return Some((with(message, &d), format!("classConstant.deprecated{description}")));
    }
    let d = deprecation?;
    let message = format!(
        "Fetching deprecated class constant {constant} of {} {}",
        description.to_ascii_lowercase(),
        name(declaring)
    );
    Some((with(message, &d), "classConstant.deprecated".to_string()))
}

/// The global constants PHPStan's stubs deprecate: name, the PHP version from which, and the message
/// (`getDeprecatedDescription() ?? 'Use of constant %s is deprecated.'`, through sprintf). From the oracle's run of
/// FetchingDeprecatedConstRule on every built-in constant at each `phpVersion`.
const BUILTIN_CONSTANTS: &[(&str, u32, Option<&str>)] = &[
    ("CURLOPT_BINARYTRANSFER", 0, None),
    ("CURLOPT_FTP_SSL", 0, Some("use <b>CURLOPT_USE_SSL</b> instead.")),
    ("CURLOPT_MUTE", 0, Some("use <b>CURLOPT_RETURNTRANSFER</b> instead since cURL 7.15.5")),
    ("CURLOPT_SAFE_UPLOAD", 0, Some("7.0 Use <b>CURLFile</b> for uploads instead.")),
    ("CURLOPT_SOCKS5_GSSAPI_SERVICE", 0, Some("Use <b>CURLOPT_PROXY_SERVICE_NAME</b> instead.")),
    ("CURLPIPE_HTTP1", 0, None),
    ("DATE_ISO8601", 0, None),
    ("FILTER_DEFAULT", 0, Some("8.5 use FILTER_UNSAFE_RAW instead")),
    ("ENCHANT_ISPELL", 80000, None),
    ("ENCHANT_MYSPELL", 80000, None),
    ("FILE_BINARY", 80100, None),
    ("FILE_TEXT", 80100, None),
    ("FILTER_SANITIZE_STRING", 80100, None),
    ("FILTER_SANITIZE_STRIPPED", 80100, None),
    ("NIL", 80100, None),
    ("MT_RAND_PHP", 80300, None),
    ("DOM_PHP_ERR", 80400, None),
    ("E_STRICT", 80400, None),
    ("MYSQLI_STORE_RESULT_COPY_DATA", 80400, None),
    ("SOAP_FUNCTIONS_ALL", 80400, None),
    ("SUNFUNCS_RET_DOUBLE", 80400, None),
    ("SUNFUNCS_RET_STRING", 80400, None),
    ("SUNFUNCS_RET_TIMESTAMP", 80400, None),
    ("DATE_RFC7231", 80500, None),
];

/// FetchingDeprecatedConstRule.
pub(super) fn constant_fetch(c: &mut Checker, at: Span, id: &Identifier) {
    if !c.deprecations || c.scope_deprecated() {
        return;
    }
    // ponytail: a branch that a `PHP_VERSION_ID` guard rules out for the configured version is dead to PHPStan (not
    // analysed); only this rule skips it
    let version = i64::from(c.php_version);
    if super::scope::version_range(&c.guards).is_some_and(|(lo, hi)| lo.is_some_and(|l| version < l) || hi.is_some_and(|h| version > h)) {
        return;
    }
    let resolved = c.names.get(&id.span());
    let user = match c.codebase.get_constant(resolved) {
        Some(k) => Some((k, resolved)),
        None => match id {
            Identifier::Local(l) if !c.names.is_imported(&l.span) => c.codebase.get_constant(l.value).map(|k| (k, l.value)),
            _ => None,
        },
    }
    .filter(|(k, _)| k.flags.is_user_defined());
    let message = if let Some((k, name)) = user {
        // a `const` statement's PHPDoc (a define()d constant has none)
        let statement = c.file_contents(k.span.file_id).is_some_and(|contents| {
            super::overriding_property::declaration_starts(k.span.file_id, &contents).contains_key(&k.span.start.offset)
        });
        if !statement {
            return;
        }
        let Some(Some(d)) = c.constant_doc(k.span).map(|r| r.deprecated) else {
            return;
        };
        // ponytail: the name as the fetch resolves it, where PHPStan prints the declared one (they differ in case only)
        let name = String::from_utf8_lossy(name).into_owned();
        php_sprintf(d.as_deref().unwrap_or("Use of constant %s is deprecated."), &name)
    } else {
        let written = String::from_utf8_lossy(id.value().strip_prefix(b"\\").unwrap_or(id.value())).into_owned();
        let Some((name, _, d)) = BUILTIN_CONSTANTS
            .iter()
            .find(|(n, since, _)| written.eq_ignore_ascii_case(n) && c.php_version >= *since)
        else {
            return;
        };
        php_sprintf(d.unwrap_or("Use of constant %s is deprecated."), name)
    };
    c.report(at, "constant.deprecated", message, None);
}

/// `sprintf($format, $name)` for a format with at most one `%s`.
fn php_sprintf(format: &str, name: &str) -> String {
    format.replacen("%s", name, 1).replace("%%", "%")
}

/// CallWithDeprecatedIniOptionRule's DEPRECATED_OPTIONS.
const DEPRECATED_OPTIONS: &[(&str, u32)] = &[
    ("mbstring.http_input", 0),
    ("mbstring.http_output", 0),
    ("mbstring.internal_encoding", 0),
    ("pdo_odbc.db2_instance_name", 0),
    ("enable_dl", 0),
    ("iconv.input_encoding", 50600),
    ("iconv.output_encoding", 50600),
    ("iconv.internal_encoding", 50600),
    ("mbstring.func_overload", 70200),
    ("track_errors", 70200),
    ("allow_url_include", 70400),
    ("assert.quiet_eval", 80000),
    ("filter.default", 80100),
    ("oci8.old_oci_close_semantics", 80100),
    ("assert.active", 80300),
    ("assert.exception", 80300),
    ("assert.bail", 80300),
    ("assert.warning", 80300),
    ("session.sid_length", 80400),
    ("session.sid_bits_per_character", 80400),
];

/// CallWithDeprecatedIniOptionRule (registered with bleedingEdge): an ini function called with a deprecated option
/// given as a string literal.
pub(super) fn ini_option(c: &mut Checker, call: &FunctionCall) {
    if !c.deprecations || !c.bleeding_edge || c.scope_deprecated() {
        return;
    }
    let Expression::Identifier(id) = call.function else {
        return;
    };
    let lower = super::function_checks::function_name(c.names, c.codebase, id);
    if !["ini_get", "ini_set", "ini_alter", "ini_restore", "get_cfg_var"].contains(&lower.as_str()) {
        return;
    }
    let Some(first) = call.argument_list.arguments.first() else {
        return;
    };
    // ponytail: a literal string only, where PHPStan takes any constant string type of the argument
    let Expression::Literal(Literal::String(LiteralString { value: Some(value), .. })) = first.value() else {
        return;
    };
    let value = String::from_utf8_lossy(value);
    if DEPRECATED_OPTIONS.iter().any(|(o, since)| *o == value && c.php_version >= *since) {
        let function = c.builtins.functions.get(&lower).map_or(lower.clone(), |f| f.name.clone());
        let message = format!("Call to function {function}() with deprecated option '{value}'.");
        c.report(call.span(), "function.deprecated", message, None);
    }
}
