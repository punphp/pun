//! The per-file driver: one [`Checker`] walk of each analysed file, then of each trait its classes use in the
//! class's context, with the `@phpstan-ignore*` comments applied.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use mago_allocator::LocalArena;
use mago_codex::metadata::CodebaseMetadata;
use mago_codex::symbol::SymbolKind;
use mago_database::file::{File, FileId};
use mago_names::ResolvedNames;
use mago_span::Span;
use mago_syntax::cst::Program;
use mago_syntax::walker::MutWalker;
use pun_phpstan::comments::lines_to_ignore;
use pun_phpstan::ignore::{
    Error, ignores_without_comments, parse_errors, unmatched_line_errors,
};
use super::named_argument::deferred_local_ignores as local_ignores;

use super::{
    Checker, anonymous, arrays, assign, class_names, infer::infer, property_access, reflection,
    signatures, syntax,
};
use crate::Project;
use crate::codebase::{Sources, parse};
use crate::relative_path::RelativePath;

/// What the rules read of the project (its `ignoreErrors` stay on the calling thread).
pub(crate) struct Checks {
    /// The declarations a stub patches, by span, with the stub declaration's span.
    patched_docs: HashMap<Span, Span>,
    /// The classes the rules built, shared by every file.
    classes: super::declared::ClassCache,
    php_version: u32,
    report_ignores_without_comments: bool,
    report_unmatched_ignores: bool,
    /// PHPStan's `checkClassCaseSensitivity` parameter: off at level 0 (conf/config.neon), on from level 2
    /// (config.level2.neon). The rules that pass it to ClassNameCheck (FunctionDefinitionCheck,
    /// ExistingClassInInstanceOfRule, CaughtExceptionExistenceRule, ExistingClassesInPropertiesRule) report
    /// `*.nameCase` only with it on. Set from [`Project::check_class_case_sensitivity`], which the presets will set.
    check_class_case_sensitivity: bool,
    /// [`Project::check_internal_class_case_sensitivity`].
    check_internal_class_case_sensitivity: bool,
    /// [`Project::check_imported_class_name_case`].
    check_imported_class_name_case: bool,
    /// [`Project::check_function_name_case`].
    check_function_name_case: bool,
    /// [`Project::level`] and [`Project::check_generic_iterable_classes`].
    pub(super) level: u8,
    check_argument_types: bool,
    check_arguments_passed_by_reference: bool,
    check_this_only: bool,
    /// [`Project::check_advanced_isset`], otherwise config.level4.neon's default.
    check_advanced_isset: bool,
    /// [`Project::treat_php_doc_types_as_certain`], otherwise config.neon's true default (IssetCheck only).
    treat_php_doc_types_as_certain: bool,
    generic_iterable_classes: bool,
    /// [`Project::new_static_in_abstract_class_static_method`].
    new_static_in_abstract_class_static_method: bool,
    /// [`Project::relative_path`].
    relative_path: RelativePath,
    /// [`Project::larastan`].
    larastan: Option<super::Larastan>,
    /// The member names the restricted-usage extensions can report (whose expressions get typed), when one runs.
    internal: Option<super::internal::Names>,
    /// `featureToggles.internalTag`: RestrictedInternalFunctionUsageExtension & co. (config.level0.neon).
    internal_tag: bool,
    /// [`Project::type_aliases`].
    pub(super) type_aliases: Vec<(String, String)>,
    /// [`Project::reported`].
    reported: Option<crate::project::Reported>,
    /// [`Project::method_rules`], independent of identifier filtering.
    method_rules: crate::MethodRules,
    /// [`Project::precise_unused_parameter_line`].
    precise_unused_parameter_line: bool,
    /// The method and property extensions of `featureToggles.internalTag`, at level 2 (config.level2.neon).
    internal_members: bool,
    /// [`Project::deprecation_rules`] and [`Project::bleeding_edge`].
    deprecations: bool,
    bleeding_edge: bool,
    /// UnusedLabelRule: level 4 with [`Project::unused_label`].
    unused_label: bool,
    /// [`Project::switch_condition`].
    switch_condition: bool,
    /// [`Project::sort_without_effect`].
    sort_without_effect: bool,
    /// [`Project::check_non_stringable_dynamic_access`].
    non_stringable_names: bool,
    cli_arguments_variables_registered: bool,
    check_maybe_undefined_variables: bool,
    pollute_scope_with_loop_initial_assignments: bool,
    pollute_scope_with_always_iterable_foreach: bool,
    pollute_scope_with_block: bool,
    /// [`Project::report_method_purity_override`].
    method_purity_override: bool,
    /// [`Project::check_uninitialized_properties`].
    uninitialized_properties: bool,
    /// [`Project::check_missing_callable_signature`].
    missing_callable_signature: bool,
    /// [`Project::strict_rules`].
    pub(super) strict: crate::StrictRules,
    /// [`Project::exceptions`].
    pub(super) exceptions: crate::Exceptions,
    /// [`Project::too_wide`].
    too_wide: crate::TooWide,
    /// RequireFileExistsRule's path and feature-toggle settings.
    include_file: super::include_file::Settings,
    /// The rules registered by a feature toggle at their level.
    toggled: Toggled,
}

/// Rules that config.levelN.neon registers with a feature toggle (all on with bleedingEdge).
#[derive(Default)]
pub(super) struct Toggled {
    /// AssignToByRefExprFromForeachRule: level 1 with `featureToggles.assignToByRefForeachExpr`.
    pub(super) by_ref_foreach: bool,
    /// ValueAssignedToGlobalConstantRule and ValueAssignedToDefineRule: level 2 with
    /// `featureToggles.checkDynamicConstantNameValues`, over `dynamicConstantNames`' typed entries.
    pub(super) constant_types: Option<Vec<(String, String)>>,
    /// DateIntervalInstantiationRule: level 5 with `featureToggles.checkDateIntervalConstructor`.
    pub(super) date_interval: bool,
    /// ParameterCastableToNumberRule: level 5 with `featureToggles.checkParameterCastableToNumberFunctions`.
    pub(super) castable_to_number: bool,
    /// PrintfParameterTypeRule: level 5 with `featureToggles.checkPrintfParameterTypes`, then whether
    /// `checkStrictPrintfPlaceholderTypes` is on.
    pub(super) printf_types: Option<bool>,
    /// ImpossibleInArrayHaystackFiniteTypesRule: level 4 with `featureToggles.finiteTypesInHaystack`.
    pub(super) finite_haystack: bool,
    /// UnusedVariableRule: level 4 with `featureToggles.unusedVariable`.
    pub(super) unused_variable: bool,
    /// Unused{Function,Method}ParametersRule: level 4 with `featureToggles.unusedParameters`.
    pub(super) unused_parameters: bool,
    /// `featureToggles.unusedParameters` itself: the closure use and constructor parameter flow findings.
    pub(super) unused_flow: bool,
    /// InstantiationRule's `featureToggles.newOnNonObject` check.
    pub(super) new_on_non_object: bool,
}

/// Each declaration a stub with a PHPDoc patches (a method, a function, a class-like) with the stub declaration's span:
/// PHPStan reads a stub's PHPDoc in place of the original's (StubPhpDocProvider).
fn patched_docs(codebase: &CodebaseMetadata) -> HashMap<Span, Span> {
    let functions = codebase.patch_function_likes.iter().filter(|(_, p)| p.has_docblock).filter_map(|(key, patch)| {
        Some((codebase.function_likes.get(key)?.span, patch.span))
    });
    let classes = codebase.patch_class_likes.iter().filter(|(_, p)| p.has_docblock).filter_map(|(key, patch)| {
        Some((codebase.class_likes.get(key)?.span, patch.span))
    });
    functions.chain(classes).filter(|(original, patch)| original != patch).collect()
}

impl Checks {
    /// Drops the errors of identifiers the run doesn't report.
    pub(crate) fn keep_reported(&self, errors: &mut Vec<Error>) {
        if let Some(reported) = &self.reported {
            errors.retain(|e| {
                let id = e.identifier.as_deref().unwrap_or_default();
                id == "phpstan.parse" || reported(id)
            });
        }
    }
}

impl Checks {
    pub(crate) fn of(project: &Project, codebase: &CodebaseMetadata) -> Checks {
        let internal_members = project.level >= 2 && project.internal_tag;
        let mut larastan = project.larastan.clone();
        if let Some(l) = &mut larastan {
            l.load(project, codebase);
        }
        Checks {
            check_class_case_sensitivity: project.check_class_case_sensitivity,
            check_internal_class_case_sensitivity: project.check_internal_class_case_sensitivity,
            check_imported_class_name_case: project.check_imported_class_name_case,
            check_function_name_case: project.check_function_name_case,
            level: project.level,
            check_argument_types: project.check_function_argument_types.unwrap_or(project.level >= 5),
            check_arguments_passed_by_reference: project.check_arguments_passed_by_reference.unwrap_or(project.level >= 5),
            check_this_only: project.check_this_only.unwrap_or(project.level < 2),
            check_advanced_isset: project.check_advanced_isset.unwrap_or(project.level >= 4),
            treat_php_doc_types_as_certain: project.treat_php_doc_types_as_certain.unwrap_or(true),
            generic_iterable_classes: project.check_generic_iterable_classes,
            new_static_in_abstract_class_static_method: project.new_static_in_abstract_class_static_method,
            php_version: project.php_version,
            report_ignores_without_comments: project.report_ignores_without_comments,
            report_unmatched_ignores: project.report_unmatched_ignores,
            relative_path: project.relative_path.clone(),
            larastan,
            internal: (internal_members || project.deprecation_rules)
                .then(|| super::internal::Names::of(codebase, project.php_version, internal_members, project.deprecation_rules)),
            internal_tag: project.internal_tag,
            type_aliases: project.type_aliases.clone(),
            reported: project.reported.clone(),
            method_rules: project.method_rules,
            precise_unused_parameter_line: project.precise_unused_parameter_line,
            internal_members,
            deprecations: project.deprecation_rules,
            patched_docs: patched_docs(codebase),
            classes: Default::default(),
            bleeding_edge: project.bleeding_edge,
            unused_label: project.level >= 4 && project.unused_label,
            switch_condition: project.switch_condition,
            sort_without_effect: project.sort_without_effect,
            non_stringable_names: project.check_non_stringable_dynamic_access,
            cli_arguments_variables_registered: project.cli_arguments_variables_registered,
            check_maybe_undefined_variables: project.check_maybe_undefined_variables,
            pollute_scope_with_loop_initial_assignments: project.pollute_scope_with_loop_initial_assignments,
            pollute_scope_with_always_iterable_foreach: project.pollute_scope_with_always_iterable_foreach,
            pollute_scope_with_block: project.pollute_scope_with_block,
            method_purity_override: project.report_method_purity_override,
            uninitialized_properties: project.check_uninitialized_properties,
            missing_callable_signature: project.check_missing_callable_signature,
            strict: project.strict_rules,
            exceptions: project.exceptions.clone(),
            too_wide: project.too_wide,
            include_file: super::include_file::Settings {
                cwd: project.current_working_directory.clone(),
                include_paths: project.include_paths.clone(),
                magic_dir: project.check_magic_dir_in_include,
                path_constants: project.use_path_constants_as_constant_string,
            },
            toggled: Toggled {
                by_ref_foreach: project.level >= 1 && project.assign_to_by_ref_foreach_expr,
                constant_types: (project.level >= 2 && project.check_dynamic_constant_name_values)
                    .then(|| project.dynamic_constant_types.clone()),
                date_interval: project.level >= 5 && project.check_date_interval_constructor,
                castable_to_number: project.level >= 5 && project.check_parameter_castable_to_number_functions,
                printf_types: (project.level >= 5 && project.check_printf_parameter_types)
                    .then_some(project.check_strict_printf_placeholder_types),
                finite_haystack: project.level >= 4 && project.finite_types_in_haystack,
                unused_variable: project.level >= 4 && project.unused_variable,
                unused_parameters: project.level >= 4 && project.unused_parameters,
                unused_flow: project.unused_parameters,
                new_on_non_object: project.new_on_non_object,
            },
        }
    }
}

/// `use T { ... }` in class `class`: the aliases with a new name that apply to T (`foo as bar`, `T::foo as bar`).
pub(super) struct TraitUse {
    pub(super) class: String,
    pub(super) trait_name: String,
    pub(super) renames: HashMap<String, String>,
    /// The `PHP_VERSION_ID` guards at the `use`: the trait is analysed in that scope.
    pub(super) php_versions: Vec<super::scope::Guard>,
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
    trait_class: Option<&str>,
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
    typed.extend(super::debug::typed_expressions(names, codebase, program));
    typed.extend(arrays::typed_expressions(program));
    typed.extend(super::names::typed_class_constants(program, project.level));
    typed.extend(super::scope::typed_method_receivers(program));
    typed.extend(super::function_callable::typed_expressions(program, project.level));
    typed.extend(super::method_callable::typed_expressions(program, project.level));
    typed.extend(super::types::typed_expressions(program, &file.contents, project.level,
        project.check_argument_types || project.check_arguments_passed_by_reference, project.check_this_only));
    typed.extend(super::strict::typed_expressions(project, codebase, program));
    typed.extend(super::dead_code::bodies(program, project.level, project.toggled.unused_parameters || project.toggled.unused_variable));
    typed.extend(super::operators::typed_expressions(project.strict, program));
    typed.extend(super::types::throw_points::typed(
        program,
        project.level >= 4 || project.exceptions.missing_checked_exception_in_throws,
    ));
    let (include_typed, include_state) = super::include_file::prepare(arena, file, program, names, codebase, php_version, project.pollute_scope_with_loop_initial_assignments, project.pollute_scope_with_always_iterable_foreach, project.pollute_scope_with_block, &project.include_file);
    typed.extend(include_typed);
    let mut variables = super::variables::Observations::default();
    let calls = if project.method_rules.call_methods {
        super::named_argument::requests(program)
    } else {
        Vec::new()
    };
    let mut variable_writes = HashMap::new();
    let mut property_initializations = HashMap::new();
    let (mut types, defined, origins, call_terminations, named_call_scopes, keep_void_types, native_types, nullsafe_method_receiver_types) = infer(arena, file, program, names, codebase, php_version, project.level, project.pollute_scope_with_loop_initial_assignments, project.pollute_scope_with_always_iterable_foreach, project.pollute_scope_with_block, typed, calls, &mut variables, &mut variable_writes, &mut property_initializations, project.larastan.as_ref(), trait_class);
    if project.level >= 4 {
        let (subjects, unknown) = super::dead_code::match_subject_types(file, program, codebase, php_version, project.pollute_scope_with_loop_initial_assignments, project.pollute_scope_with_always_iterable_foreach, project.pollute_scope_with_block);
        types.extend(subjects);
        for span in unknown {
            types.remove(&span);
        }
    }
    Checker {
        anonymous: anonymous::names(file, program, names, &project.relative_path),
        property_access: property_access::state(program),
        relative_path: &project.relative_path,
        file,
        has_compiler_halt: false,
        types,
        keep_void_types,
        native_types,
        nullsafe_method_receiver_types,
        variable_writes,
        property_initializations,
        call_terminations,
        named_call_scopes,
        defined,
        variables,
        origins,
        names,
        codebase,
        classes: Vec::new(),
        guards: Vec::new(),
        marks: Vec::new(),
        vars: HashMap::new(),
        saved: Vec::new(),
        functions: Vec::new(),
        var_tag_functions: Vec::new(),
        in_exists_call: 0,
        top_level: HashSet::new(),
        piped: HashSet::new(),
        rebound: HashMap::new(),
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
        patched_docs: &project.patched_docs,
        class_cache: &project.classes,
        syntax: syntax::State::default(),
        assign: assign::State::default(),
        signatures: signatures::State::default(),
        class_names: class_names::State::default(),
        arrays: arrays::State::default(),
        standalone_throws: HashSet::new(),
        case_sensitivity: project.check_class_case_sensitivity,
        check_internal_class_case_sensitivity: project.check_internal_class_case_sensitivity,
        check_imported_class_name_case: project.check_imported_class_name_case,
        check_function_name_case: project.check_function_name_case,
        level: project.level,
        check_argument_types: project.check_argument_types,
        check_arguments_passed_by_reference: project.check_arguments_passed_by_reference,
        check_this_only: project.check_this_only,
        check_advanced_isset: project.check_advanced_isset,
        treat_php_doc_types_as_certain: project.treat_php_doc_types_as_certain,
        generic_iterable_classes: project.generic_iterable_classes,
        new_static_in_abstract_class_static_method: project.new_static_in_abstract_class_static_method,
        larastan: project.larastan.as_ref(),
        larastan_state: Default::default(),
        internal: project.internal.as_ref(),
        internal_tag: project.internal_tag,
        precise_unused_parameter_line: project.precise_unused_parameter_line,
        unreachable: Default::default(),
        conditions: Default::default(),
        internal_members: project.internal_members,
        deprecations: project.deprecations,
        bleeding_edge: project.bleeding_edge,
        unused_label: project.unused_label,
        switch_condition: project.switch_condition,
        sort_without_effect: project.sort_without_effect,
        non_stringable_names: project.non_stringable_names,
        cli_arguments_variables_registered: project.cli_arguments_variables_registered,
        check_maybe_undefined_variables: project.check_maybe_undefined_variables,
        pollute_scope_with_loop_initial_assignments: project.pollute_scope_with_loop_initial_assignments,
        pollute_scope_with_always_iterable_foreach: project.pollute_scope_with_always_iterable_foreach,
        pollute_scope_with_block: project.pollute_scope_with_block,
        method_purity_override: project.method_purity_override,
        uninitialized_properties: project.uninitialized_properties,
        missing_callable_signature: project.missing_callable_signature,
        strict: project.strict,
        exceptions: &project.exceptions,
        too_wide: project.too_wide,
        include_settings: &project.include_file,
        include_state,
        closure_bind: 0,
        toggled: &project.toggled,
        by_ref_foreach: Vec::new(),
        method_rules: project.method_rules,
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
    let errors = crate::parser::errors(file, program, crate::codebase::php_version(project.php_version));
    if !errors.is_empty() {
        return errors;
    }
    let mut checker = checker(
        &arena, file, program, &names, codebase, project, files, sources, None,
    );
    walk(&mut checker, program, project);
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
    project.keep_reported(&mut errors);
    let mut out = Vec::new();
    if project.report_ignores_without_comments {
        out.extend(ignores_without_comments(&name, &mut lines, &mut unmatched));
    }
    let (kept, _) = local_ignores(&name, errors, &mut lines, &mut unmatched);
    out.extend(kept);
    if project.report_unmatched_ignores {
        out.extend(unmatched_line_errors(&name, &unmatched));
    }
    enum TraitWalk {
        Enter(TraitUse),
        Exit(String),
    }
    let mut pending: Vec<_> = std::mem::take(&mut checker.trait_uses)
        .into_iter()
        .map(TraitWalk::Enter)
        .collect();
    // TraitUseHandler::processTraitUse() guards only the active path: another use can have different aliases.
    let mut active = HashSet::new();
    while let Some(step) = pending.pop() {
        let u = match step {
            TraitWalk::Enter(u) => u,
            TraitWalk::Exit(name) => {
                active.remove(&name);
                continue;
            }
        };
        let trait_name = u.trait_name.to_ascii_lowercase();
        if !active.insert(trait_name.clone()) {
            continue;
        }
        let (errors, uses) = in_trait_context(&name, &u, codebase, project, files, sources);
        out.extend(errors);
        pending.push(TraitWalk::Exit(trait_name));
        pending.extend(uses.into_iter().map(TraitWalk::Enter));
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
        &arena, &file, program, &names, codebase, project, files, sources, Some(class),
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
    checker.guards = u.php_versions.clone();
    checker.trait_context = Some(super::TraitContext {
        class_file: class_file.to_string(),
        class: class.to_string(),
        trait_name: String::from_utf8_lossy(t.name.as_bytes()).into_owned(),
        renames: u.renames.clone(),
    });
    walk(&mut checker, program, project);
    let label = checker.label.clone();
    let (mut lines, _) = lines_to_ignore(&file.contents);
    lines.retain(|line, _| {
        checker
            .trait_lines
            .iter()
            .any(|(a, b)| a <= line && line <= b)
    });
    let mut unmatched = lines.clone();
    let mut errors = std::mem::take(&mut checker.errors);
    project.keep_reported(&mut errors);
    let (kept, _) = local_ignores(&label, errors, &mut lines, &mut unmatched);
    (kept, checker.trait_uses)
}

/// Walk a file with the `typeAliases` parameter and Larastan's type node extensions in effect.
fn walk<'a, 'arena>(checker: &mut super::Checker<'a, 'arena>, program: &'arena Program<'arena>, project: &Checks) {
    let (codebase, larastan) = (checker.codebase, checker.larastan);
    let nodes = |name: &str, args: &[pun_phpstan::types::Type]| {
        larastan.and_then(|l| super::larastan::type_node(codebase, l, name, args))
    };
    pun_phpstan::resolve::with_type_aliases(&project.type_aliases, || {
        pun_phpstan::resolve::with_type_nodes(&nodes, || checker.walk_program(program, &mut ()))
    });
}
