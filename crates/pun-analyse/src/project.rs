//! Which files are analysed and which are only scanned for symbols, as PHPStan decides it: `paths` minus
//! `excludePaths`, plus `scanFiles`, `scanDirectories` and what Composer autoloads (the project's composer.json and
//! vendor/composer/installed.json, as PHPStan's ComposerJsonAndInstalledJsonSourceLocatorMaker reads them).

use std::path::{Path, PathBuf};

use pun_phpstan::Array;
use pun_phpstan::Key;
use pun_phpstan::Value;
use pun_phpstan::ignore::IgnoredErrors;
use pun_phpstan::php::FileExcluder;
use rayon::prelude::*;
use serde_json::Value as Json;

use crate::relative_path::{RelativePath, absolute};
use crate::{display, php_files};

pub struct Project {
    /// Reported on.
    pub analysed: Vec<PathBuf>,
    /// Symbols only.
    pub scanned: Vec<PathBuf>,
    /// Composer's psr-4 and psr-0 files: symbols only, scanned when an analysed or scanned file names the class their
    /// path holds, as PHPStan's PSR locators find classes on demand.
    pub autoloaded: Vec<Autoloaded>,
    /// `PHP_VERSION_ID`.
    pub php_version: u32,
    /// IncludedFilePathResolver's `currentWorkingDirectory`, independent of the analysed script's directory.
    pub current_working_directory: PathBuf,
    /// PHP's runtime `include_path`, supplied without starting PHP. The portable default is `.`; callers that
    /// need a distro's php.ini paths or bootstrap changes must supply those entries explicitly.
    pub include_paths: Vec<PathBuf>,
    /// `featureToggles.magicDirInInclude`: check the __DIR__ concatenation fallback (off by default).
    pub check_magic_dir_in_include: bool,
    /// `usePathConstantsAsConstantString`: retain literal __DIR__/__FILE__ types (off by default).
    pub use_path_constants_as_constant_string: bool,
    /// `reportUnmatchedIgnoredErrors`.
    pub report_unmatched_ignores: bool,
    /// `reportIgnoresWithoutComments`.
    pub report_ignores_without_comments: bool,
    /// `ignoreErrors` (with the baseline's entries).
    pub ignore_errors: IgnoredErrors,
    /// PHPStan's `checkClassCaseSensitivity` (see `rules::Checks`): false, as at level 0, until the presets set it.
    pub check_class_case_sensitivity: bool,
    /// `checkInternalClassCaseSensitivity`: include built-in classes in case checks (off by default).
    pub check_internal_class_case_sensitivity: bool,
    /// `featureToggles.checkImportedClassNameCase`: check a one-part name's written case (on with bleedingEdge).
    pub check_imported_class_name_case: bool,
    /// `checkFunctionNameCase`: off by default at every level, independently of class-name case checks.
    pub check_function_name_case: bool,
    /// `cliArgumentsVariablesRegistered`: exempt argc and argv in the main scope only.
    pub cli_arguments_variables_registered: bool,
    /// `checkMaybeUndefinedVariables`: enabled by config.level1.neon, independently overridable.
    pub check_maybe_undefined_variables: bool,
    /// `polluteScopeWithLoopInitialAssignments`: keep for/while entry assignments (on by default).
    pub pollute_scope_with_loop_initial_assignments: bool,
    /// `polluteScopeWithAlwaysIterableForeach`: narrow iterable bodies and retain new exit holders.
    pub pollute_scope_with_always_iterable_foreach: bool,
    /// `polluteScopeWithBlock`: retain the actual standalone block scope (on by default).
    pub pollute_scope_with_block: bool,
    /// The rule level: `level` (`max` is 10), 0 when unset.
    pub level: u8,
    /// `checkFunctionArgumentTypes`: `None` follows the level (on from 5), including after callers change it.
    pub check_function_argument_types: Option<bool>,
    /// `checkArgumentsPassedByReference`: independently overridable, otherwise on from level 5.
    pub check_arguments_passed_by_reference: Option<bool>,
    /// `checkThisOnly`: RuleLevelHelper's expression eligibility, otherwise on below level 2.
    pub check_this_only: Option<bool>,
    /// `checkAdvancedIsset`: independently overridable; `None` follows the current level (on from 4).
    pub check_advanced_isset: Option<bool>,
    /// `treatPhpDocTypesAsCertain`: independently overridable; `None` defaults to true. Used by IssetCheck only.
    pub treat_php_doc_types_as_certain: Option<bool>,
    /// `featureToggles.checkGenericIterableClasses` (on with bleedingEdge).
    pub check_generic_iterable_classes: bool,
    /// `featureToggles.internalTag` (on with bleedingEdge): the `@internal` usage rules.
    pub internal_tag: bool,
    /// `featureToggles.newStaticInAbstractClassStaticMethod` (on with bleedingEdge).
    pub new_static_in_abstract_class_static_method: bool,
    /// `featureToggles.reportPreciseLineForUnusedFunctionParameter` (on with bleedingEdge): unused closure uses and
    /// constructor parameters are reported on their own line, not the function's.
    pub precise_unused_parameter_line: bool,
    /// Every analysed path is a file (FileFinderResult's `isOnlyFiles()`).
    pub only_files: bool,
    /// `featureToggles.bleedingEdge`.
    pub bleeding_edge: bool,
    /// `featureToggles.unusedLabel` (on with bleedingEdge).
    pub unused_label: bool,
    /// `featureToggles.switchConditionAlwaysFalse` (on with bleedingEdge): SwitchConditionRule.
    pub switch_condition: bool,
    /// `featureToggles.sortWithoutEffect` (on with bleedingEdge): SortWithoutEffectRule.
    pub sort_without_effect: bool,
    /// `featureToggles.checkNonStringableDynamicAccess` (on with bleedingEdge): the `*.nameNotString` checks.
    pub check_non_stringable_dynamic_access: bool,
    /// `featureToggles.reportMethodPurityOverride` (on with bleedingEdge): MethodSignatureRule's purity checks.
    pub report_method_purity_override: bool,
    /// `featureToggles.assignToByRefForeachExpr` (on with bleedingEdge): AssignToByRefExprFromForeachRule.
    pub assign_to_by_ref_foreach_expr: bool,
    /// `featureToggles.checkDynamicConstantNameValues` (on with bleedingEdge): ValueAssignedToGlobalConstantRule and
    /// ValueAssignedToDefineRule.
    pub check_dynamic_constant_name_values: bool,
    /// `featureToggles.checkDateIntervalConstructor` (on with bleedingEdge): DateIntervalInstantiationRule.
    pub check_date_interval_constructor: bool,
    /// `dynamicConstantNames`' entries with a configured type (name, type string).
    pub dynamic_constant_types: Vec<(String, String)>,
    /// `checkUninitializedProperties`: UninitializedPropertyRule (config.neon's conditional rule).
    pub check_uninitialized_properties: bool,
    /// `checkMissingCallableSignature`: MissingTypehintCheck's `missingType.callable` (off by default).
    pub check_missing_callable_signature: bool,
    /// `deprecationRulesInstalled`: the config includes phpstan-deprecation-rules' rules.neon.
    pub deprecation_rules: bool,
    /// The `strictRules` switches of phpstan-strict-rules' rules.neon that Pun ports (all off without it).
    pub strict_rules: StrictRules,
    /// The `exceptions.*` parameters of the throw point rules.
    pub exceptions: Exceptions,
    /// TooWideTypeCheck's switches the param-out rules read.
    pub too_wide: TooWide,
    /// How anonymous class names print their file: relative to the working directory and the analysed paths.
    pub relative_path: RelativePath,
    /// Larastan's rules and their switches, when the config includes Larastan's extension.neon.
    pub larastan: Option<crate::Larastan>,
    /// `typeAliases`: the global type aliases (name, type string).
    pub type_aliases: Vec<(String, String)>,
    /// `stubFiles`: scanned as Mago patches, whose declarations override the symbols they name.
    pub stubs: Vec<PathBuf>,
    /// Which identifiers the run reports, `None` for all: the others are dropped before any ignore sees them, as
    /// PHPStan never reports the errors of a rule it doesn't run.
    pub reported: Option<Reported>,
    /// Independent registrations of the method rules that produce or join named-argument records.
    pub method_rules: MethodRules,
}

/// [`Project::reported`].
pub type Reported = std::sync::Arc<dyn Fn(&str) -> bool + Send + Sync>;

/// Rule registration, not identifier filtering or FunctionCallParametersCheck's argument switches.
#[derive(Clone, Copy)]
pub struct MethodRules {
    /// CallMethodsRule, including its ordinary diagnostics and named-argument call records.
    pub call_methods: bool,
    /// OverridingMethodRule, including MethodSignatureRule and parameter-rename records.
    pub overriding_method: bool,
    /// ConsistentConstructorRule, independently of OverridingMethodRule.
    pub consistent_constructor: bool,
    /// MethodCallWithPossiblyRenamedNamedArgumentRule's post-file join.
    pub renamed_named_arguments: bool,
}

impl Default for MethodRules {
    fn default() -> Self {
        Self {
            call_methods: true,
            overriding_method: true,
            consistent_constructor: true,
            renamed_named_arguments: true,
        }
    }
}

/// phpstan-strict-rules' `strictRules.*` switches of the ported rules.
#[derive(Clone, Copy, Default)]
pub struct StrictRules {
    /// `closureUsesThis`: ClosureUsesThisRule.
    pub closure_uses_this: bool,
    /// `requireParentConstructorCall`: RequireParentConstructCallRule.
    pub require_parent_constructor_call: bool,
    /// `matchingInheritedMethodNames`: WrongCaseOfInheritedMethodRule.
    pub matching_inherited_method_names: bool,
    /// `dynamicCallOnStaticMethod`: DynamicCallOnStaticMethodsRule and DynamicCallOnStaticMethodsCallableRule.
    pub dynamic_call_on_static_method: bool,
    /// `booleansInConditions`: BooleanInBooleanAndRule, BooleanInBooleanOrRule and BooleanInBooleanNotRule.
    pub booleans_in_conditions: bool,
    /// `numericOperandsInArithmeticOperators`: OperandsInArithmetic*Rule and OperandInArithmetic*Rule.
    pub numeric_operands_in_arithmetic_operators: bool,
    /// `booleansInLoopConditions`: BooleanInWhileConditionRule and BooleanInDoWhileConditionRule.
    pub booleans_in_loop_conditions: bool,
    /// `uselessCast`: UselessCastRule.
    pub useless_cast: bool,
    /// `switchConditionsMatchingType`: MatchingTypeInSwitchCaseConditionRule.
    pub switch_conditions_matching_type: bool,
}

impl StrictRules {
    fn from_params(params: &Array) -> StrictRules {
        let strict = params.get_str("strictRules").and_then(Value::as_array);
        // ConditionalTagsExtension: a list of switches is on when all of them are
        let on = |k: &str| match strict.and_then(|s| s.get_str(k)) {
            Some(Value::Bool(b)) => *b,
            Some(Value::Array(a)) => a.values().all(|v| *v == Value::Bool(true)),
            _ => false,
        };
        StrictRules {
            closure_uses_this: on("closureUsesThis"),
            require_parent_constructor_call: on("requireParentConstructorCall"),
            matching_inherited_method_names: on("matchingInheritedMethodNames"),
            dynamic_call_on_static_method: on("dynamicCallOnStaticMethod"),
            booleans_in_conditions: on("booleansInConditions"),
            numeric_operands_in_arithmetic_operators: on("numericOperandsInArithmeticOperators"),
            booleans_in_loop_conditions: on("booleansInLoopConditions"),
            useless_cast: on("uselessCast"),
            switch_conditions_matching_type: on("switchConditionsMatchingType"),
        }
    }
}

/// The `exceptions.*` parameters and `checkTooWideThrowTypesInProtectedAndPublicMethods` (config.neon's defaults).
#[derive(Clone, Debug, PartialEq)]
pub struct Exceptions {
    /// `implicitThrows`: a call without `@throws` may throw anything.
    pub implicit_throws: bool,
    /// `reportUncheckedExceptionDeadCatch`.
    pub report_unchecked_exception_dead_catch: bool,
    /// DefaultExceptionTypeResolver's `uncheckedExceptionRegexes`, `uncheckedExceptionClasses`,
    /// `checkedExceptionRegexes` and `checkedExceptionClasses`.
    pub unchecked_exception_regexes: Vec<String>,
    pub unchecked_exception_classes: Vec<String>,
    pub checked_exception_regexes: Vec<String>,
    pub checked_exception_classes: Vec<String>,
    /// `check.missingCheckedExceptionInThrows`: the MissingCheckedExceptionIn*ThrowsRule rules.
    pub missing_checked_exception_in_throws: bool,
    /// `check.tooWideThrowType`: the TooWide*ThrowTypeRule rules (level 4).
    pub too_wide_throw_type: bool,
    /// `check.tooWideImplicitThrowType`: TooWideMethodThrowTypeRule also for an inherited `@throws`.
    pub too_wide_implicit_throw_type: bool,
    /// `check.throwTypeCovariance`: MethodThrowTypeCovarianceRule.
    pub throw_type_covariance: bool,
    /// `checkTooWideThrowTypesInProtectedAndPublicMethods`.
    pub check_too_wide_throw_types_in_protected_and_public_methods: bool,
}

impl Default for Exceptions {
    fn default() -> Self {
        Exceptions {
            implicit_throws: true,
            report_unchecked_exception_dead_catch: true,
            unchecked_exception_regexes: Vec::new(),
            unchecked_exception_classes: vec!["Error".into()],
            checked_exception_regexes: Vec::new(),
            checked_exception_classes: Vec::new(),
            missing_checked_exception_in_throws: false,
            too_wide_throw_type: true,
            too_wide_implicit_throw_type: false,
            throw_type_covariance: false,
            check_too_wide_throw_types_in_protected_and_public_methods: false,
        }
    }
}

impl Exceptions {
    fn from_params(params: &Array) -> Exceptions {
        let d = Exceptions::default();
        let exceptions = params.get_str("exceptions").and_then(Value::as_array);
        let check = exceptions.and_then(|e| e.get_str("check")).and_then(Value::as_array);
        let flag = |a: Option<&Array>, k: &str, default: bool| match a.and_then(|a| a.get_str(k)) {
            Some(Value::Bool(b)) => *b,
            _ => default,
        };
        let list = |k: &str, default: Vec<String>| match exceptions.and_then(|e| e.get_str(k)).and_then(Value::as_array) {
            Some(a) => a.values().filter_map(Value::as_str).map(str::to_string).collect(),
            None => default,
        };
        Exceptions {
            implicit_throws: flag(exceptions, "implicitThrows", d.implicit_throws),
            report_unchecked_exception_dead_catch: flag(
                exceptions,
                "reportUncheckedExceptionDeadCatch",
                d.report_unchecked_exception_dead_catch,
            ),
            unchecked_exception_regexes: list("uncheckedExceptionRegexes", d.unchecked_exception_regexes),
            unchecked_exception_classes: list("uncheckedExceptionClasses", d.unchecked_exception_classes),
            checked_exception_regexes: list("checkedExceptionRegexes", d.checked_exception_regexes),
            checked_exception_classes: list("checkedExceptionClasses", d.checked_exception_classes),
            missing_checked_exception_in_throws: flag(
                check,
                "missingCheckedExceptionInThrows",
                d.missing_checked_exception_in_throws,
            ),
            too_wide_throw_type: flag(check, "tooWideThrowType", d.too_wide_throw_type),
            too_wide_implicit_throw_type: flag(check, "tooWideImplicitThrowType", d.too_wide_implicit_throw_type),
            throw_type_covariance: flag(check, "throwTypeCovariance", d.throw_type_covariance),
            check_too_wide_throw_types_in_protected_and_public_methods: flag(
                Some(params),
                "checkTooWideThrowTypesInProtectedAndPublicMethods",
                d.check_too_wide_throw_types_in_protected_and_public_methods,
            ),
        }
    }
}

/// TooWideTypeCheck's `featureToggles.reportTooWideBool` (on with bleedingEdge) and
/// `featureToggles.reportNestedTooWideType`, and TooWideMethodParameterOutTypeRule's
/// `checkTooWideParameterOutInProtectedAndPublicMethods`.
#[derive(Clone, Copy, Default)]
pub struct TooWide {
    pub report_bool: bool,
    pub report_nested: bool,
    pub param_out_in_protected_and_public_methods: bool,
}

/// PHP 8.5.11, the oracle's runtime, which PHPStan uses when `phpVersion` is null.
pub(crate) const RUNTIME_PHP_VERSION: u32 = 80511;

fn strings<'a>(params: &'a Array, key: &str) -> Vec<&'a str> {
    params
        .get_str(key)
        .and_then(Value::as_array)
        .map(|a| a.values().filter_map(Value::as_str).collect())
        .unwrap_or_default()
}

impl Project {
    pub fn from_config(root: &Path, params: &Array) -> Project {
        let extensions: Vec<String> = strings(params, "fileExtensions")
            .into_iter()
            .map(str::to_string)
            .collect();
        let excludes = params.get_str("excludePaths").and_then(Value::as_array);
        let group = |k: &str| {
            excludes
                .and_then(|e| e.get_str(k))
                .and_then(Value::as_array)
                .map(|a| a.values().filter_map(Value::as_str).collect::<Vec<_>>())
                .unwrap_or_default()
        };
        let both = FileExcluder::new(group("analyseAndScan"));
        let analyse_only = FileExcluder::new(group("analyse"));
        let mut analysed = Vec::new();
        let mut scanned = Vec::new();
        let paths: Vec<String> = strings(params, "paths")
            .into_iter()
            .map(|p| absolute(Path::new(p)))
            .collect();
        for path in strings(params, "paths") {
            let p = Path::new(path);
            let found = if p.is_file() {
                vec![p.to_path_buf()]
            } else {
                php_files(p, &extensions)
            };
            for f in found {
                let name = display(&f);
                if both.is_excluded(&name) {
                    continue;
                }
                if analyse_only.is_excluded(&name) {
                    scanned.push(f);
                } else {
                    analysed.push(f);
                }
            }
        }
        scanned.extend(strings(params, "scanFiles").into_iter().map(PathBuf::from));
        for dir in strings(params, "scanDirectories") {
            scanned.extend(php_files(Path::new(dir), &extensions));
        }
        // PHPStan's PSR locators look classes up whatever excludePaths says
        let (composer, autoloaded) = composer_files(root, &extensions);
        scanned.extend(composer);
        scanned.retain(|f| !both.is_excluded(&display(f)));
        let php_version = params
            .get_str("phpVersion")
            // the schema's `{min, max}` structure requires both as ints
            .and_then(|v| match v {
                Value::Array(a) => a.get_str("min"),
                v => Some(v),
            })
            .and_then(|v| match v {
                Value::Int(i) => u32::try_from(*i).ok(),
                _ => None,
            })
            .unwrap_or(RUNTIME_PHP_VERSION);
        let flag = |k: &str| params.get_str(k) == Some(&Value::Bool(true));
        let optional_flag = |k: &str| match params.get_str(k) {
            Some(Value::Bool(value)) => Some(*value),
            _ => None,
        };
        let toggle = |k: &str| {
            params
                .get_str("featureToggles")
                .and_then(Value::as_array)
                .and_then(|t| t.get_str(k))
                == Some(&Value::Bool(true))
        };
        let report_unmatched_ignores = flag("reportUnmatchedIgnoredErrors");
        let empty = Array::new();
        let ignore_errors = params
            .get_str("ignoreErrors")
            .and_then(Value::as_array)
            .unwrap_or(&empty);
        let level = match params.get_str("level") {
            Some(Value::Int(i)) => u8::try_from(*i).unwrap_or(10).min(10),
            Some(Value::Str(s)) if s == "max" => 10,
            _ => 0,
        };
        Project {
            analysed,
            scanned,
            autoloaded,
            php_version,
            current_working_directory: root.to_path_buf(),
            include_paths: vec![PathBuf::from(".")],
            check_magic_dir_in_include: toggle("magicDirInInclude"),
            use_path_constants_as_constant_string: flag("usePathConstantsAsConstantString"),
            report_unmatched_ignores,
            report_ignores_without_comments: flag("reportIgnoresWithoutComments"),
            ignore_errors: IgnoredErrors::new(ignore_errors, report_unmatched_ignores),
            // config.neon's false, which config.level2.neon turns on
            check_class_case_sensitivity: flag("checkClassCaseSensitivity"),
            check_internal_class_case_sensitivity: flag("checkInternalClassCaseSensitivity"),
            check_imported_class_name_case: toggle("checkImportedClassNameCase"),
            check_function_name_case: flag("checkFunctionNameCase"),
            cli_arguments_variables_registered: flag("cliArgumentsVariablesRegistered"),
            check_maybe_undefined_variables: flag("checkMaybeUndefinedVariables"),
            pollute_scope_with_loop_initial_assignments: !matches!(params.get_str("polluteScopeWithLoopInitialAssignments"), Some(Value::Bool(false))),
            pollute_scope_with_always_iterable_foreach: !matches!(params.get_str("polluteScopeWithAlwaysIterableForeach"), Some(Value::Bool(false))),
            pollute_scope_with_block: !matches!(params.get_str("polluteScopeWithBlock"), Some(Value::Bool(false))),
            level,
            check_function_argument_types: optional_flag("checkFunctionArgumentTypes"),
            check_arguments_passed_by_reference: optional_flag("checkArgumentsPassedByReference"),
            check_this_only: optional_flag("checkThisOnly"),
            check_advanced_isset: optional_flag("checkAdvancedIsset"),
            treat_php_doc_types_as_certain: optional_flag("treatPhpDocTypesAsCertain"),
            check_generic_iterable_classes: toggle("checkGenericIterableClasses"),
            internal_tag: toggle("internalTag"),
            new_static_in_abstract_class_static_method: toggle("newStaticInAbstractClassStaticMethod"),
            only_files: strings(params, "paths").iter().all(|p| Path::new(p).is_file()),
            bleeding_edge: toggle("bleedingEdge"),
            unused_label: toggle("unusedLabel"),
            switch_condition: toggle("switchConditionAlwaysFalse"),
            sort_without_effect: toggle("sortWithoutEffect"),
            check_non_stringable_dynamic_access: toggle("checkNonStringableDynamicAccess"),
            report_method_purity_override: toggle("reportMethodPurityOverride"),
            assign_to_by_ref_foreach_expr: toggle("assignToByRefForeachExpr"),
            check_dynamic_constant_name_values: toggle("checkDynamicConstantNameValues"),
            check_date_interval_constructor: toggle("checkDateIntervalConstructor"),
            // ConstantResolver::getConfiguredGlobalConstantType(): only the string keys have a type
            dynamic_constant_types: params
                .get_str("dynamicConstantNames")
                .and_then(Value::as_array)
                .map(|a| {
                    a.iter()
                        .filter(|(k, _)| matches!(k, Key::Str(_)))
                        .filter_map(|(k, v)| Some((k.as_string(), v.as_str()?.to_string())))
                        .collect()
                })
                .unwrap_or_default(),
            check_uninitialized_properties: flag("checkUninitializedProperties"),
            check_missing_callable_signature: flag("checkMissingCallableSignature"),
            deprecation_rules: flag("deprecationRulesInstalled"),
            strict_rules: StrictRules::from_params(params),
            exceptions: Exceptions::from_params(params),
            too_wide: TooWide {
                report_bool: toggle("reportTooWideBool"),
                report_nested: toggle("reportNestedTooWideType"),
                param_out_in_protected_and_public_methods: flag("checkTooWideParameterOutInProtectedAndPublicMethods"),
            },
            precise_unused_parameter_line: toggle("reportPreciseLineForUnusedFunctionParameter"),
            relative_path: RelativePath::of(&paths),
            larastan: crate::Larastan::from_params(root, params),
            type_aliases: params
                .get_str("typeAliases")
                .and_then(Value::as_array)
                .map(|a| {
                    a.iter()
                        .filter_map(|(k, v)| Some((k.as_string(), v.as_str()?.to_string())))
                        .collect()
                })
                .unwrap_or_default(),
            stubs: strings(params, "stubFiles")
                .into_iter()
                .map(PathBuf::from)
                .collect(),
            reported: None,
            method_rules: MethodRules::default(),
        }
    }

    /// PHPStan's defaults (no `ignoreErrors`, unmatched ignores reported) over the given files.
    pub fn new(analysed: Vec<PathBuf>, scanned: Vec<PathBuf>) -> Project {
        let paths: Vec<String> = analysed.iter().map(|p| absolute(p)).collect();
        Project {
            relative_path: RelativePath::of(&paths),
            analysed,
            scanned,
            autoloaded: Vec::new(),
            php_version: RUNTIME_PHP_VERSION,
            current_working_directory: std::env::current_dir().unwrap_or_default(),
            include_paths: vec![PathBuf::from(".")],
            check_magic_dir_in_include: false,
            use_path_constants_as_constant_string: false,
            report_unmatched_ignores: true,
            report_ignores_without_comments: false,
            ignore_errors: IgnoredErrors::new(&Array::new(), true),
            check_class_case_sensitivity: false,
            check_internal_class_case_sensitivity: false,
            check_imported_class_name_case: false,
            check_function_name_case: false,
            cli_arguments_variables_registered: true,
            check_maybe_undefined_variables: false,
            pollute_scope_with_loop_initial_assignments: true,
            pollute_scope_with_always_iterable_foreach: true,
            pollute_scope_with_block: true,
            level: 0,
            check_function_argument_types: None,
            check_arguments_passed_by_reference: None,
            check_this_only: None,
            check_advanced_isset: None,
            treat_php_doc_types_as_certain: None,
            check_generic_iterable_classes: false,
            internal_tag: false,
            new_static_in_abstract_class_static_method: false,
            only_files: true,
            bleeding_edge: false,
            unused_label: false,
            switch_condition: false,
            sort_without_effect: false,
            check_non_stringable_dynamic_access: false,
            report_method_purity_override: false,
            assign_to_by_ref_foreach_expr: false,
            check_dynamic_constant_name_values: false,
            check_date_interval_constructor: false,
            dynamic_constant_types: Vec::new(),
            check_uninitialized_properties: false,
            check_missing_callable_signature: false,
            deprecation_rules: false,
            strict_rules: StrictRules::default(),
            exceptions: Exceptions::default(),
            too_wide: TooWide::default(),
            precise_unused_parameter_line: false,
            larastan: None,
            type_aliases: Vec::new(),
            stubs: Vec::new(),
            reported: None,
            method_rules: MethodRules::default(),
        }
    }
}

/// A psr-4 or psr-0 file and the class its path holds.
pub struct Autoloaded {
    pub path: PathBuf,
    /// The class name, ASCII-lowercased: the psr-4 prefix and the path below the directory, or for psr-0 the path
    /// alone, with `\` for `/` and without the extension.
    pub class: Vec<u8>,
    /// psr-0, where `_` in the class's own name is a directory too.
    pub psr0: bool,
}

/// Where Composer's autoloader finds files, in its order.
enum Source {
    /// A psr-4 directory and its namespace prefix.
    Psr4(String, PathBuf),
    /// A psr-0 directory.
    Psr0(PathBuf),
    /// A classmap entry: a directory or a file.
    Classmap(PathBuf),
    /// A `files` entry.
    File(PathBuf),
}

/// Files Composer can autoload, of the root package (with autoload-dev) and every installed package: classmap entries
/// and `files`, then the psr-4 and psr-0 directories' files, by class.
fn composer_files(root: &Path, extensions: &[String]) -> (Vec<PathBuf>, Vec<Autoloaded>) {
    let read =
        |p: PathBuf| -> Option<Json> { serde_json::from_slice(&std::fs::read(p).ok()?).ok() };
    let mut sources = Vec::new();
    let mut add = |base: &Path, autoload: &Json| {
        for kind in ["psr-4", "psr-0"] {
            for (prefix, dirs) in autoload[kind].as_object().into_iter().flatten() {
                let dirs = match dirs {
                    Json::Array(a) => a.iter().filter_map(Json::as_str).collect(),
                    d => d.as_str().into_iter().collect::<Vec<_>>(),
                };
                for d in dirs {
                    let dir = base.join(d);
                    sources.push(if kind == "psr-4" {
                        Source::Psr4(prefix.clone(), dir)
                    } else {
                        Source::Psr0(dir)
                    });
                }
            }
        }
        for entry in autoload["classmap"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(Json::as_str)
        {
            sources.push(Source::Classmap(base.join(entry)));
        }
        for f in autoload["files"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(Json::as_str)
        {
            sources.push(Source::File(base.join(f)));
        }
    };
    if let Some(composer) = read(root.join("composer.json")) {
        add(root, &composer["autoload"]);
        add(root, &composer["autoload-dev"]);
    }
    let installed_dir = root.join("vendor/composer");
    if let Some(installed) = read(installed_dir.join("installed.json")) {
        let packages = installed.get("packages").unwrap_or(&installed);
        for p in packages.as_array().into_iter().flatten() {
            if let Some(path) = p["install-path"].as_str() {
                add(&installed_dir.join(path), &p["autoload"]);
            }
        }
    }
    // every package's directories are walked at once
    let found: Vec<(Vec<PathBuf>, Vec<Autoloaded>)> = sources
        .into_par_iter()
        .map(|source| match source {
            Source::Psr4(prefix, d) => (
                Vec::new(),
                by_class(&d, prefix.as_bytes(), false, extensions),
            ),
            Source::Psr0(d) => (Vec::new(), by_class(&d, b"", true, extensions)),
            Source::Classmap(p) if p.is_dir() => (php_files(&p, extensions), Vec::new()),
            Source::Classmap(p) | Source::File(p) => (vec![p], Vec::new()),
        })
        .collect();
    let mut out = Vec::new();
    let mut autoloaded = Vec::new();
    for (files, classes) in found {
        out.extend(files);
        autoloaded.extend(classes);
    }
    // Composer's own classes, in every generated classmap (PHPStan finds it through the project's autoloader)
    // ponytail: other classes only the runtime autoloader knows (autoload_classmap.php) aren't scanned
    for class in ["InstalledVersions.php", "ClassLoader.php"] {
        let path = installed_dir.join(class);
        if path.is_file() {
            out.push(path);
        }
    }
    (out, autoloaded)
}

/// The files below a psr-4 or psr-0 directory, each with the class its path holds.
fn by_class(dir: &Path, prefix: &[u8], psr0: bool, extensions: &[String]) -> Vec<Autoloaded> {
    php_files(dir, extensions)
        .into_iter()
        .map(|path| {
            let mut class = prefix.to_ascii_lowercase();
            let rel = path.strip_prefix(dir).unwrap_or(&path).with_extension("");
            for (i, part) in rel.iter().enumerate() {
                if i > 0 {
                    class.push(b'\\');
                }
                class.extend(part.as_encoded_bytes().iter().map(u8::to_ascii_lowercase));
            }
            Autoloaded { path, class, psr0 }
        })
        .collect()
}
