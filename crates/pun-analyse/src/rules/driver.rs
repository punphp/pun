//! The per-file driver: one [`Checker`] walk of each analysed file, then of each trait its classes use in the
//! class's context, with the `@phpstan-ignore*` comments applied.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use mago_allocator::LocalArena;
use mago_codex::metadata::CodebaseMetadata;
use mago_codex::symbol::SymbolKind;
use mago_database::file::{File, FileId};
use mago_names::ResolvedNames;
use mago_syntax::cst::Program;
use mago_syntax::walker::MutWalker;
use pun_phpstan::comments::lines_to_ignore;
use pun_phpstan::ignore::{
    Error, ignores_without_comments, local_ignores, parse_errors, unmatched_line_errors,
};

use super::{
    Checker, anonymous, arrays, assign, class_names, infer::infer, property_access, reflection,
    signatures, syntax,
};
use crate::Project;
use crate::codebase::{Sources, parse};
use crate::relative_path::RelativePath;

/// What the rules read of the project (its `ignoreErrors` stay on the calling thread).
pub(crate) struct Checks {
    php_version: u32,
    report_ignores_without_comments: bool,
    report_unmatched_ignores: bool,
    /// PHPStan's `checkClassCaseSensitivity` parameter: off at level 0 (conf/config.neon), on from level 2
    /// (config.level2.neon). The rules that pass it to ClassNameCheck (FunctionDefinitionCheck,
    /// ExistingClassInInstanceOfRule, CaughtExceptionExistenceRule, ExistingClassesInPropertiesRule) report
    /// `*.nameCase` only with it on. Set from [`Project::check_class_case_sensitivity`], which the presets will set.
    check_class_case_sensitivity: bool,
    /// [`Project::level`] and [`Project::check_generic_iterable_classes`].
    level: u8,
    generic_iterable_classes: bool,
    /// [`Project::relative_path`].
    relative_path: RelativePath,
    /// [`Project::larastan`].
    larastan: Option<super::Larastan>,
    /// The `@internal` method names, at level 2 with `featureToggles.internalTag` (config.level2.neon).
    internal: Option<super::internal::Names>,
    /// `featureToggles.internalTag`: RestrictedInternalFunctionUsageExtension (config.level0.neon).
    internal_tag: bool,
}

impl Checks {
    pub(crate) fn of(project: &Project, codebase: &CodebaseMetadata) -> Checks {
        let mut larastan = project.larastan.clone();
        if let Some(l) = &mut larastan {
            l.load(project, codebase);
        }
        Checks {
            check_class_case_sensitivity: project.check_class_case_sensitivity,
            level: project.level,
            generic_iterable_classes: project.check_generic_iterable_classes,
            php_version: project.php_version,
            report_ignores_without_comments: project.report_ignores_without_comments,
            report_unmatched_ignores: project.report_unmatched_ignores,
            relative_path: project.relative_path.clone(),
            larastan,
            internal: (project.level >= 2 && project.internal_tag)
                .then(|| super::internal::Names::of(codebase)),
            internal_tag: project.internal_tag,
        }
    }
}

/// `use T { ... }` in class `class`: the aliases with a new name that apply to T (`foo as bar`, `T::foo as bar`).
pub(super) struct TraitUse {
    pub(super) class: String,
    pub(super) trait_name: String,
    pub(super) renames: HashMap<String, String>,
}

/// A checker for one walk of `file`.
#[allow(clippy::too_many_arguments)]
fn checker<'a, 'arena>(
    arena: &'arena LocalArena,
    file: &'a File,
    program: &'arena Program<'arena>,
    names: &'a ResolvedNames<'arena>,
    codebase: &'a CodebaseMetadata,
    project: &'a Checks,
    files: &'a HashMap<FileId, PathBuf>,
    sources: &'a Sources,
) -> Checker<'a, 'arena> {
    let php_version = project.php_version;
    let mut typed = super::larastan::typed_expressions(project.larastan.as_ref(), names, program);
    typed.extend(super::internal::typed_expressions(
        project.internal.as_ref(),
        program,
    ));
    typed.extend(super::function_checks::typed_expressions(
        names, codebase, program,
    ));
    Checker {
        anonymous: anonymous::names(file, program, names, &project.relative_path),
        property_access: property_access::state(program),
        relative_path: &project.relative_path,
        file,
        types: infer(
            arena,
            file,
            program,
            names,
            codebase,
            php_version,
            project.level,
            typed,
        ),
        names,
        codebase,
        classes: Vec::new(),
        guards: Vec::new(),
        marks: Vec::new(),
        vars: HashMap::new(),
        saved: Vec::new(),
        functions: Vec::new(),
        in_exists_call: 0,
        top_level: HashSet::new(),
        piped: HashSet::new(),
        rebound: HashSet::new(),
        label: String::from_utf8_lossy(&file.name).into_owned(),
        trait_context: None,
        in_target: false,
        trait_uses: Vec::new(),
        renamed: None,
        php_version,
        builtins: reflection::builtins(php_version),
        trait_lines: Vec::new(),
        frames: Vec::new(),
        namespaces: Vec::new(),
        magic_defaults: HashSet::new(),
        first_statement: true,
        declares_not_first: Vec::new(),
        decls: Vec::new(),
        immutable: HashMap::new(),
        files,
        sources,
        syntax: syntax::State::default(),
        assign: assign::State::default(),
        signatures: signatures::State::default(),
        class_names: class_names::State::default(),
        arrays: arrays::State::default(),
        standalone_throws: HashSet::new(),
        case_sensitivity: project.check_class_case_sensitivity,
        level: project.level,
        generic_iterable_classes: project.generic_iterable_classes,
        larastan: project.larastan.as_ref(),
        larastan_state: Default::default(),
        internal: project.internal.as_ref(),
        internal_tag: project.internal_tag,
        errors: Vec::new(),
    }
}

/// The file's errors after its `@phpstan-ignore*` comments (LocalIgnoresProcessor), with the unmatched ones, and
/// the errors of the traits its classes use, analysed in each class's context. `files` finds a symbol's file.
pub(crate) fn check_file(
    path: &Path,
    codebase: &CodebaseMetadata,
    project: &Checks,
    files: &HashMap<FileId, PathBuf>,
    sources: &Sources,
) -> Vec<Error> {
    // the scan read every analysed file it could
    let Some(file) = sources.get(path) else {
        return Vec::new();
    };
    let arena = LocalArena::new();
    let (program, names) = parse(&arena, file);
    let mut checker = checker(
        &arena, file, program, &names, codebase, project, files, sources,
    );
    checker.walk_program(program, &mut ());
    let name = checker.file_name();
    let (mut lines, parse) = lines_to_ignore(&file.contents);
    let mut unmatched = lines.clone();
    unmatched.retain(|line, _| {
        !checker
            .trait_lines
            .iter()
            .any(|(a, b)| a <= line && line <= b)
    });
    let mut errors = std::mem::take(&mut checker.errors);
    errors.extend(parse_errors(&name, &parse));
    let mut out = Vec::new();
    if project.report_ignores_without_comments {
        out.extend(ignores_without_comments(&name, &mut lines, &mut unmatched));
    }
    let (kept, _) = local_ignores(&name, errors, &mut lines, &mut unmatched);
    out.extend(kept);
    if project.report_unmatched_ignores {
        out.extend(unmatched_line_errors(&name, &unmatched));
    }
    let mut pending = std::mem::take(&mut checker.trait_uses);
    let mut seen = HashSet::new();
    while let Some(u) = pending.pop() {
        if !seen.insert((
            u.class.to_ascii_lowercase(),
            u.trait_name.to_ascii_lowercase(),
        )) {
            continue;
        }
        let (errors, uses) = in_trait_context(&name, &u, codebase, project, files, sources);
        out.extend(errors);
        pending.extend(uses);
    }
    out
}

/// NodeScopeResolver::processTraitUse() and InTraitNode: the trait of `u` walked in the context of its class
/// (declared in `class_file`), its errors labelled `T.php (in context of class C)` and matched against the trait's
/// own `@phpstan-ignore` comments. Also returns the traits it uses. Only a trait in an analysed file is walked: one
/// PHPStan only scans (scanDirectories, vendor) reports nothing.
fn in_trait_context(
    class_file: &str,
    u: &TraitUse,
    codebase: &CodebaseMetadata,
    project: &Checks,
    files: &HashMap<FileId, PathBuf>,
    sources: &Sources,
) -> (Vec<Error>, Vec<TraitUse>) {
    let class = u.class.as_str();
    let Some(t) = codebase
        .get_class_like(u.trait_name.as_bytes())
        .filter(|t| t.kind == SymbolKind::Trait)
    else {
        return (Vec::new(), Vec::new());
    };
    // the analysed files are the ones the scan kept the sources of
    let Some(file) = files
        .get(&t.span.file_id)
        .and_then(|p| sources.get(p).cloned())
    else {
        return (Vec::new(), Vec::new());
    };
    let arena = LocalArena::new();
    let (program, names) = parse(&arena, &file);
    let mut checker = checker(
        &arena, &file, program, &names, codebase, project, files, sources,
    );
    let display = codebase
        .get_class_like(class.as_bytes())
        .map_or_else(|| class.to_string(), |k| checker.display(k));
    // getFileDescription(): an anonymous class's name stands alone
    let context = if class.starts_with("{anonymous-class:") {
        display
    } else {
        format!("class {display}")
    };
    checker.label = format!("{} (in context of {context})", checker.file_name());
    checker.trait_context = Some(super::TraitContext {
        class_file: class_file.to_string(),
        class: class.to_string(),
        trait_name: String::from_utf8_lossy(t.name.as_bytes()).into_owned(),
        renames: u.renames.clone(),
    });
    checker.walk_program(program, &mut ());
    let label = checker.label.clone();
    let (mut lines, _) = lines_to_ignore(&file.contents);
    lines.retain(|line, _| {
        checker
            .trait_lines
            .iter()
            .any(|(a, b)| a <= line && line <= b)
    });
    let mut unmatched = lines.clone();
    let (kept, _) = local_ignores(
        &label,
        std::mem::take(&mut checker.errors),
        &mut lines,
        &mut unmatched,
    );
    (kept, checker.trait_uses)
}
