//! PHPStan 2.3.1's RequireFileExistsRule and IncludedFilePathResolver. The checker walks a trait's declaring
//! file, not the using class's file, so both compile-time path constants and the script-directory fallback use it.
//!
//! Runtime boundary: Pun does not start PHP or execute bootstrap code. `Project::include_paths` supplies PHP's
//! runtime include_path (the portable default is `.`). Local files, file:// and local phar:// members are stat'd.
//! PHP's zip:// has no URL-stat handler, even for readable members. Runtime-registered wrappers, PHAR aliases and
//! mounts need bootstrap execution and remain unsupported. PHAR stat models the pinned runtime's default
//! require_hash=1, zlib present and bz2 absent; OpenSSL signatures/sidecars are not supported. Missing-file
//! dependencies are not cached here: the analyser checks the filesystem again on every run.

use std::collections::{HashMap, HashSet};
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};

use mago_allocator::LocalArena;
use mago_codex::metadata::CodebaseMetadata;
use mago_codex::ttype::union::TUnion;
use mago_database::file::File;
use mago_names::ResolvedNames;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;
use pun_phpstan::types::Type;
use sha2::Digest;

use super::types::flow::unparen;
use super::{Checker, infer, scope};

pub(super) struct Settings {
    pub cwd: PathBuf,
    pub include_paths: Vec<PathBuf>,
    pub magic_dir: bool,
    pub path_constants: bool,
}

/// The same inference engine with generalized path constants, isolated from the other rules' types.
pub(super) struct State {
    types: HashMap<Span, TUnion>,
}

struct Wanted {
    spans: Vec<Span>,
    in_include: usize,
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Wanted {
    fn walk_in_expression(&mut self, e: &'ast Expression<'arena>, _: &mut ()) {
        if self.in_include > 0 {
            self.spans.push(e.span());
        }
    }
    fn walk_in_construct(&mut self, construct: &'ast Construct<'arena>, _: &mut ()) {
        self.in_include += usize::from(included(construct).is_some());
    }
    fn walk_out_construct(&mut self, construct: &'ast Construct<'arena>, _: &mut ()) {
        self.in_include -= usize::from(included(construct).is_some());
    }
}

/// InitializerExprTypeResolver generalizes __DIR__/__FILE__ by default. Mago already supplies that contract when
/// File.path is absent. Use it for this rule's requested expressions only, preserving real paths for every other
/// rule and for resolveFilePaths()'s __DIR__ fallback. This also handles aliases and control-flow joins without
/// guessing provenance from literal contents or implementing another type/data-flow engine.
pub(super) fn prepare<'arena>(
    arena: &'arena LocalArena,
    file: &File,
    program: &'arena Program<'arena>,
    names: &ResolvedNames<'arena>,
    codebase: &CodebaseMetadata,
    php_version: u32,
    settings: &Settings,
) -> (Vec<Span>, State) {
    let mut wanted = Wanted {
        spans: Vec::new(),
        in_include: 0,
    };
    wanted.walk_program(program, &mut ());
    if settings.path_constants || wanted.spans.is_empty() {
        return (
            wanted.spans,
            State {
                types: HashMap::new(),
            },
        );
    }
    let file = File::new(
        file.name.clone(),
        file.file_type,
        None,
        file.contents.clone(),
    );
    let (types, _, _) = infer::analyse(
        arena,
        &file,
        program,
        names,
        codebase,
        php_version,
        wanted.spans,
        Vec::new(),
    );
    (Vec::new(), State { types })
}

fn included<'a, 'arena>(
    construct: &'a Construct<'arena>,
) -> Option<(&'a Expression<'arena>, &'static str, &'static str)> {
    Some(match construct {
        Construct::Include(i) => (i.value, "include", "include.fileNotFound"),
        Construct::IncludeOnce(i) => (i.value, "include_once", "includeOnce.fileNotFound"),
        Construct::Require(i) => (i.value, "require", "require.fileNotFound"),
        Construct::RequireOnce(i) => (i.value, "require_once", "requireOnce.fileNotFound"),
        _ => return None,
    })
}

/// processNode(), doesFileExist() and getErrorMessage(). All four diagnostics are ignorable.
pub(super) fn construct(c: &mut Checker, construct: &Construct) {
    let Some((expr, kind, identifier)) = included(construct) else {
        return;
    };
    let key = print_expr(c, expr);
    if c.guards
        .iter()
        .any(|g| matches!(g, scope::Guard::File(g) if g.key == key && g.valid))
    {
        return;
    }
    let mut fallback = false;
    for path in resolve_file_paths(c, expr, &mut fallback) {
        if does_file_exist(c, &path) {
            continue;
        }
        let path_expr = if fallback {
            key.clone()
        } else {
            format!("\"{path}\"")
        };
        c.report(
            construct,
            identifier,
            format!("Path in {kind}() {path_expr} is not a file or it does not exist."),
            None,
        );
    }
}

fn resolve_file_paths(c: &Checker, expr: &Expression, fallback: &mut bool) -> Vec<String> {
    *fallback = false;
    if !c.include_settings.magic_dir {
        return strings(c, expr);
    }
    let Expression::Binary(b) = unparen(expr) else {
        return strings(c, expr);
    };
    if !matches!(b.operator, BinaryOperator::StringConcat(_)) {
        return strings(c, expr);
    }
    if matches!(
        unparen(b.lhs),
        Expression::MagicConstant(MagicConstant::Directory(_))
    ) {
        *fallback = true;
        let dir = Path::new(&c.file_name())
            .parent()
            .unwrap_or(Path::new(""))
            .to_string_lossy()
            .into_owned();
        return strings(c, b.rhs)
            .into_iter()
            .map(|s| format!("{dir}{s}"))
            .collect();
    }
    // Upstream passes the same reference through right, then left: the last recursive call owns the flag.
    let right = resolve_file_paths(c, b.rhs, fallback);
    let left = resolve_file_paths(c, b.lhs, fallback);
    let mut paths = Vec::new();
    for left in left {
        for right in &right {
            let path = normalize_separator(&format!("{left}{right}"));
            if !paths.contains(&path) {
                paths.push(path);
            }
        }
    }
    paths
}

fn expression_type(c: &Checker, expr: &Expression) -> Option<Type> {
    if c.include_settings.path_constants {
        c.phpstan_type(expr)
    } else {
        c.include_state
            .types
            .get(&expr.span())
            .map(|t| c.to_phpstan(t))
    }
}

fn strings(c: &Checker, expr: &Expression) -> Vec<String> {
    // ConstantResolver deliberately models both OS separators, even on Unix. Rebuild concatenations from their
    // typed operands so Mago's platform-specific DIRECTORY_SEPARATOR cannot discard an alternative.
    match unparen(expr) {
        Expression::ConstantAccess(a)
            if c.written_or_resolved(&a.name).trim_start_matches('\\') == "DIRECTORY_SEPARATOR"
                && (c.resolved(&a.name) == "DIRECTORY_SEPARATOR"
                    || !c.codebase.constant_exists(c.names.get(&a.name.span()))) =>
        {
            vec!["/".into(), "\\".into()]
        }
        Expression::ConstantAccess(a)
            if c.written_or_resolved(&a.name).trim_start_matches('\\') == "PATH_SEPARATOR"
                && (c.resolved(&a.name) == "PATH_SEPARATOR"
                    || !c.codebase.constant_exists(c.names.get(&a.name.span()))) =>
        {
            vec![":".into(), ";".into()]
        }
        Expression::Binary(b) if matches!(b.operator, BinaryOperator::StringConcat(_)) => {
            let left = strings(c, b.lhs);
            let right = strings(c, b.rhs);
            if left.is_empty() || right.is_empty() {
                return expression_type(c, expr).map_or_else(Vec::new, |t| constant_strings(&t));
            }
            left.iter()
                .flat_map(|l| right.iter().map(move |r| format!("{l}{r}")))
                .collect()
        }
        _ => expression_type(c, expr).map_or_else(Vec::new, |t| constant_strings(&t)),
    }
}

/// UnionType::pickFromTypes() rejects the entire union if any member has no constant strings. In contrast,
/// IntersectionType collects the strings of every member (accessories contribute none).
fn constant_strings(t: &Type) -> Vec<String> {
    match t {
        Type::ConstString(s) => vec![s.clone()],
        Type::Union(ts) | Type::Benevolent(ts) => {
            let parts: Vec<_> = ts.iter().map(constant_strings).collect();
            if parts.iter().any(Vec::is_empty) {
                Vec::new()
            } else {
                parts.concat()
            }
        }
        Type::Intersection(ts) => ts.iter().flat_map(constant_strings).collect(),
        Type::Template(t) => constant_strings(&t.bound),
        _ => Vec::new(),
    }
}

fn does_file_exist(c: &Checker, path: &str) -> bool {
    if let Some((scheme, rest)) = path.split_once("://")
        && scheme.eq_ignore_ascii_case("phar")
    {
        return phar_file(&c.include_settings.cwd, rest).unwrap_or(false);
    }
    candidates(c, path).iter().any(|p| p.is_file())
}

/// PHP 8.5.12 ext/phar/phar.c: phar_detect_phar_fname_ext(), phar_check_str(), phar_fix_filepath().
/// The first existing archive filename with a non-leading extension wins, even if its contents are invalid.
/// Relative wrapper paths use cwd alone: FileHelper::absolutizePath() preserves stream URLs unchanged.
fn phar_file(cwd: &Path, path: &str) -> Option<bool> {
    if path.contains('\0') {
        return None;
    }
    for (end, _) in path.match_indices('/') {
        let archive = &path[..end];
        let name = archive.rsplit('/').next()?;
        if !name.match_indices('.').any(|(dot, _)| {
            dot > 0
                && name.len() - dot < 50
                && name.as_bytes().get(dot + 1).is_some_and(|b| *b != b'.')
        }) {
            continue;
        }
        let archive = cwd.join(archive);
        if !archive.is_file() {
            continue;
        }
        let mut parts = Vec::new();
        for part in path[end + 1..].split('/').filter(|s| !s.is_empty()) {
            if part.len() >= 2 && part.bytes().all(|b| b == b'.') {
                parts.pop();
            } else if part != "." {
                parts.push(part);
            }
        }
        let member = parts.join("/");
        let bytes = std::fs::read(archive).ok()?;
        return archive_file(&bytes, member.as_bytes(), 3);
    }
    None
}

/// PHP dispatches by contents, not by the archive extension. Stat only needs the manifest/index: member PHP,
/// serialized metadata and stubs are never evaluated, extracted or deserialized.
fn archive_file(bytes: &[u8], member: &[u8], recursion: u8) -> Option<bool> {
    if bytes.starts_with(b"\x1f\x8b\x08") {
        let mut unpacked = Vec::new();
        flate2::read::GzDecoder::new(bytes)
            .read_to_end(&mut unpacked)
            .ok()?;
        // ext/phar/phar.c rejects --recursion_count == 0 before restarting archive dispatch.
        let recursion = recursion - 1;
        if recursion == 0 {
            return None;
        }
        return archive_file(&unpacked, member, recursion);
    }
    if bytes.starts_with(b"PK\x03\x04") {
        return zip_file(bytes, member);
    }
    if let Some(found) = tar_file(bytes, member) {
        return Some(found);
    }
    const HALT: &[u8] = b"__HALT_COMPILER();";
    if let Some(halt) = bytes
        .windows(HALT.len())
        .position(|s| s.eq_ignore_ascii_case(HALT))
    {
        return native_phar_file(bytes, halt + HALT.len(), member);
    }
    None
}

/// Native PHAR manifest, ext/phar/phar.c: phar_parse_pharfile(). The phar 0.0.3 crate was evaluated but only
/// accepts the exact `__HALT_COMPILER(); ?>\r\n` stub and does not expose directory flags for stat. Keep this
/// bounded byte parser instead; ZIP/TAR indexing and gzip decoding use their existing libraries below.
fn native_phar_file(bytes: &[u8], mut halt: usize, member: &[u8]) -> Option<bool> {
    if matches!(bytes.get(halt..halt + 3), Some(b" ?>" | b"\n?>")) {
        halt += 3;
        if bytes.get(halt) == Some(&b'\r') {
            halt += 1;
            if bytes.get(halt) != Some(&b'\n') {
                return None;
            }
        }
        if bytes.get(halt) == Some(&b'\n') {
            halt += 1;
        }
    }
    let mut data = bytes.get(halt..)?;
    let size = word(&mut data)? as usize;
    if size > 100 * 1024 * 1024 {
        return None;
    }
    let mut manifest = take(&mut data, size)?;
    let count = word(&mut manifest)?;
    let version = u16::from_be_bytes(take(&mut manifest, 2)?.try_into().ok()?);
    let flags = word(&mut manifest)?;
    if count == 0 || version & 0xfff0 < 0x1000 || flags & 0x10000 == 0 {
        return None;
    }
    let mut trailer = bytes.get(bytes.len().checked_sub(8)?..)?;
    let kind = word(&mut trailer)?;
    if trailer != b"GBMB" {
        return None;
    }
    let signature_size = match kind {
        1 => 16,
        2 => 20,
        3 => 32,
        4 => 64,
        _ => return None,
    };
    let signed_end = bytes.len().checked_sub(8 + signature_size)?;
    if !signature(
        kind,
        &[&bytes[..signed_end]],
        &bytes[signed_end..bytes.len() - 8],
    ) {
        return None;
    }
    for _ in 0..2 {
        // alias and serialized metadata: length-prefixed opaque bytes
        let size = word(&mut manifest)? as usize;
        take(&mut manifest, size)?;
    }
    let mut found = false;
    for _ in 0..count {
        let size = word(&mut manifest)? as usize;
        let name = take(&mut manifest, size)?;
        if name.is_empty() {
            return None;
        }
        let size = word(&mut manifest)?;
        word(&mut manifest)?; // timestamp
        let compressed_size = word(&mut manifest)?;
        word(&mut manifest)?; // crc32 (not checked by stat)
        let flags = word(&mut manifest)?;
        let metadata_size = word(&mut manifest)? as usize;
        take(&mut manifest, metadata_size)?;
        if flags & 0xf000 == 0x2000 || (flags & 0xf000 != 0x1000 && size != compressed_size) {
            return None;
        }
        found |= name == member && !name.ends_with(b"/");
    }
    Some(found)
}

fn word(data: &mut &[u8]) -> Option<u32> {
    Some(u32::from_le_bytes(take(data, 4)?.try_into().ok()?))
}

fn take<'a>(data: &mut &'a [u8], size: usize) -> Option<&'a [u8]> {
    let (head, tail) = data.split_at_checked(size)?;
    *data = tail;
    Some(head)
}

/// PHAR's four digest signatures. OpenSSL signatures additionally need runtime key/sidecar configuration.
fn signature(kind: u32, parts: &[&[u8]], expected: &[u8]) -> bool {
    fn digest<D: Digest>(parts: &[&[u8]], expected: &[u8]) -> bool {
        let mut digest = D::new();
        for part in parts {
            digest.update(part);
        }
        digest.finalize().as_slice() == expected
    }
    match kind {
        1 => digest::<md5::Md5>(parts, expected),
        2 => digest::<sha1::Sha1>(parts, expected),
        3 => digest::<sha2::Sha256>(parts, expected),
        4 => digest::<sha2::Sha512>(parts, expected),
        _ => false,
    }
}

/// ext/phar/zip.c: phar_parse_zipfile(). PHP's phar:// wrapper accepts ordinary ZIP data archives as well as
/// executable ZIP PHARs. Only the latter require a signature; a present signature is always verified.
fn zip_file(bytes: &[u8], member: &[u8]) -> Option<bool> {
    let mut zip = zip::ZipArchive::new(Cursor::new(bytes)).ok()?;
    let central = zip.central_directory_start() as usize;
    let comment = zip.comment().to_vec();
    let mut executable = false;
    let mut signed = false;
    let mut found = false;
    let count = zip.len();
    for index in 0..count {
        let file = zip.by_index_raw(index).ok()?;
        if file.encrypted()
            || !matches!(
                file.compression(),
                zip::CompressionMethod::STORE | zip::CompressionMethod::DEFLATE
            )
        {
            return None;
        }
        let name = file.name_raw();
        if name == b".phar/signature.bin" {
            let start = file.data_start()?;
            let end = start.checked_add(file.size())?;
            let mut contents = bytes.get(start as usize..end as usize)?;
            let kind = word(&mut contents)?;
            word(&mut contents)?; // signature length (PHP verifies the actual remaining bytes)
            signed = signature(
                kind,
                &[
                    bytes.get(..file.header_start() as usize)?,
                    bytes.get(central..file.central_header_start() as usize)?,
                    &comment,
                ],
                contents,
            );
            if !signed || index + 1 != count {
                return None;
            }
        } else {
            executable |= name == b".phar/stub.php";
            // PHP only treats '/' as a separator here, unlike zip::ZipFile::is_dir()'s Windows spelling.
            found |= name == member && !name.ends_with(b"/");
        }
    }
    Some(found && (!executable || signed))
}

/// ext/phar/tar.c: phar_parse_tarfile(). PHAR's signature member is hidden from its manifest. Link entries are
/// files for stat; hard links require a preceding target in the manifest, while dangling symlinks are accepted.
fn tar_file(bytes: &[u8], member: &[u8]) -> Option<bool> {
    let mut tar = tar::Archive::new(Cursor::new(bytes));
    let mut executable = false;
    let mut signed = false;
    let mut found = false;
    let mut names = HashSet::<Vec<u8>>::new();
    for entry in tar.entries().ok()? {
        let entry = entry.ok()?;
        if signed {
            return None;
        }
        let name = entry.path_bytes();
        if name.as_ref() == b".phar/signature.bin" {
            let start = entry.raw_file_position() as usize;
            let end = start.checked_add(entry.size() as usize)?;
            let mut contents = bytes.get(start..end)?;
            let kind = word(&mut contents)?;
            word(&mut contents)?;
            signed = signature(
                kind,
                &[bytes.get(..entry.raw_header_position() as usize)?],
                contents,
            );
            if !signed {
                return None;
            }
        } else {
            if entry.header().entry_type().is_hard_link()
                && !names.contains(entry.link_name_bytes()?.as_ref())
            {
                return None;
            }
            names.insert(name.to_vec());
            executable |= name.as_ref() == b".phar/stub.php";
            found |= name == member && !entry.header().entry_type().is_dir();
        }
    }
    Some(found && (!executable || signed))
}

/// IncludedFilePathResolver::resolve(). Absolute paths bypass directory prefixes, but are not canonicalized:
/// `missing-directory/../existing.php` is not a file. Only FileHelper's directory gets normalizePath().
fn candidates(c: &Checker, path: &str) -> Vec<PathBuf> {
    static SCHEME: std::sync::OnceLock<pcre2::bytes::Regex> = std::sync::OnceLock::new();
    let scheme = SCHEME.get_or_init(|| {
        pcre2::bytes::Regex::new("(?i)^([a-z0-9+\\-.]+)://").expect("upstream wrapper pattern")
    });
    let path = if scheme.is_match(path.as_bytes()).expect("wrapper match") {
        let (scheme, rest) = path.split_once("://").expect("matched separator");
        if !scheme.eq_ignore_ascii_case("file") {
            return Vec::new();
        }
        let local = rest
            .strip_prefix("localhost/")
            .map_or_else(|| rest.to_string(), |s| format!("/{s}"));
        if !Path::new(&local).is_absolute() {
            return Vec::new();
        }
        local
    } else {
        path.to_string()
    };
    let script = PathBuf::from(c.file_name());
    let mut paths = Vec::new();
    for dir in std::iter::once(c.include_settings.cwd.as_path())
        .chain(
            c.include_settings
                .include_paths
                .iter()
                .map(PathBuf::as_path),
        )
        .chain(script.parent())
    {
        if dir.as_os_str().is_empty() {
            continue;
        }
        let dir = normalize_directory(&dir.to_string_lossy());
        let candidate = if Path::new(&path).is_absolute() {
            PathBuf::from(&path)
        } else {
            let joined = PathBuf::from(format!(
                "{}/{}",
                dir.trim_end_matches('/'),
                path.trim_start_matches(['/', '\\'])
            ));
            if joined.is_absolute() {
                joined
            } else {
                c.include_settings.cwd.join(joined)
            }
        };
        if !paths.contains(&candidate) {
            paths.push(candidate);
        }
    }
    paths
}

fn normalize_separator(path: &str) -> String {
    path.replace('\\', "/")
        .replace("//", "/")
        .replace("///", "/")
        .replace("////", "/")
}

fn normalize_directory(path: &str) -> String {
    let path = normalize_separator(path);
    let mut parts = Vec::new();
    for part in path.trim_matches('/').split('/') {
        match part {
            "." => {}
            ".." => {
                parts.pop();
            }
            part => parts.push(part),
        }
    }
    format!(
        "{}{}",
        if path.starts_with('/') { "/" } else { "" },
        parts.join("/")
    )
}

/// ExprPrinter's concatenation spacing, associativity and interpolated-variable braces. The remaining operands
/// retain their source spelling, as php-parser retains a literal's quote kind and a constant's written name.
fn print_expr(c: &Checker, expr: &Expression) -> String {
    match unparen(expr) {
        Expression::Binary(b) if matches!(b.operator, BinaryOperator::StringConcat(_)) => {
            let left = print_expr(c, b.lhs);
            let right = print_expr(c, b.rhs);
            let right = if matches!(unparen(b.rhs), Expression::Binary(b) if matches!(b.operator, BinaryOperator::StringConcat(_)))
            {
                format!("({right})")
            } else {
                right
            };
            format!("{left} . {right}")
        }
        Expression::CompositeString(CompositeString::Interpolated(s)) => {
            let mut out = String::from("\"");
            for part in s.parts.iter() {
                match part {
                    StringPart::Literal(l) => out.push_str(&String::from_utf8_lossy(l.raw)),
                    StringPart::Expression(e) => out.push_str(&format!("{{{}}}", print_expr(c, e))),
                    StringPart::BracedExpression(e) => {
                        out.push_str(&format!("{{{}}}", print_expr(c, e.expression)))
                    }
                }
            }
            out.push('"');
            out
        }
        e => c.written(e.span()),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct FileGuard {
    function: String,
    key: String,
    variables: Vec<Vec<u8>>,
    valid: bool,
}

fn variables(expr: &Expression) -> Vec<Vec<u8>> {
    struct Vars(Vec<Vec<u8>>);
    impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Vars {
        fn walk_in_direct_variable(&mut self, v: &'ast DirectVariable<'arena>, _: &mut ()) {
            self.0.push(v.name.to_vec());
        }
    }
    let mut vars = Vars(Vec::new());
    vars.walk_expression(expr, &mut ());
    vars.0
}

/// The synthetic fully-qualified, lowercase calls in isInFileExists() are expression keys, not a generic
/// implication that every file check establishes every other one. A namespace-local call (or uppercase spelling)
/// has a different key in PHPStan; the oracle controls deliberately retain this distinction.
pub(super) fn guards(c: &Checker, expr: &Expression, truth: bool) -> Vec<scope::Guard> {
    let recurse = |e, truth| guards(c, e, truth);
    match unparen(expr) {
        Expression::UnaryPrefix(u) if matches!(u.operator, UnaryPrefixOperator::Not(_)) => {
            recurse(u.operand, !truth)
        }
        Expression::Binary(b)
            if matches!(
                b.operator,
                BinaryOperator::And(_)
                    | BinaryOperator::LowAnd(_)
                    | BinaryOperator::Or(_)
                    | BinaryOperator::LowOr(_)
            ) =>
        {
            let left = recurse(b.lhs, truth);
            let right = recurse(b.rhs, truth);
            if matches!(
                b.operator,
                BinaryOperator::And(_) | BinaryOperator::LowAnd(_)
            ) == truth
            {
                left.into_iter().chain(right).collect()
            } else {
                left.into_iter().filter(|g| right.contains(g)).collect()
            }
        }
        Expression::Binary(b) => {
            let literal = |e| match unparen(e) {
                Expression::Literal(Literal::True(_)) => Some(true),
                Expression::Literal(Literal::False(_)) => Some(false),
                _ => None,
            };
            let equal = matches!(
                b.operator,
                BinaryOperator::Equal(_) | BinaryOperator::Identical(_)
            );
            if equal
                || matches!(
                    b.operator,
                    BinaryOperator::NotEqual(_) | BinaryOperator::NotIdentical(_)
                )
            {
                if let Some(value) = literal(b.rhs) {
                    return recurse(b.lhs, value == (truth == equal));
                }
                if let Some(value) = literal(b.lhs) {
                    return recurse(b.rhs, value == (truth == equal));
                }
            }
            Vec::new()
        }
        Expression::Call(Call::Function(call)) if truth => {
            if let Expression::Identifier(id) = call.function
                && matches!(
                    c.resolved(id).as_str(),
                    "file_exists"
                        | "is_file"
                        | "is_readable"
                        | "is_writable"
                        | "is_writeable"
                        | "is_executable"
                )
                && let Some(arg) = call.argument_list.arguments.first()
            {
                return vec![scope::Guard::File(FileGuard {
                    function: c.resolved(id),
                    key: print_expr(c, arg.value()),
                    variables: variables(arg.value()),
                    valid: true,
                })];
            }
            Vec::new()
        }
        _ => Vec::new(),
    }
}

/// MutatingScope invalidates remembered calls when an argument changes. Keep the guard slots (block marks index
/// them), but invalidate only those depending on an assigned variable.
pub(super) fn assigned(c: &mut Checker, lhs: &Expression) {
    let vars = variables(lhs);
    for guard in &mut c.guards {
        if let scope::Guard::File(g) = guard
            && g.variables.iter().any(|v| vars.contains(v))
        {
            g.valid = false;
        }
    }
}
