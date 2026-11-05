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

bitflags::bitflags! {
    /// The boolean parameters of a [`Project`].
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct ProjectOptions: u64 {
        /// `featureToggles.magicDirInInclude`: check the __DIR__ concatenation fallback (off by default).
        const CHECK_MAGIC_DIR_IN_INCLUDE = 1 << 0;
        /// `usePathConstantsAsConstantString`: retain literal __DIR__/__FILE__ types (off by default).
        const USE_PATH_CONSTANTS_AS_CONSTANT_STRING = 1 << 1;
        /// `reportUnmatchedIgnoredErrors`.
        const REPORT_UNMATCHED_IGNORES = 1 << 2;
        /// `reportIgnoresWithoutComments`.
        const REPORT_IGNORES_WITHOUT_COMMENTS = 1 << 3;
        /// PHPStan's `checkClassCaseSensitivity` (see `rules::Checks`): false, as at level 0, until the presets set it.
        const CHECK_CLASS_CASE_SENSITIVITY = 1 << 4;
        /// `checkInternalClassCaseSensitivity`: include built-in classes in case checks (off by default).
        const CHECK_INTERNAL_CLASS_CASE_SENSITIVITY = 1 << 5;
        /// `featureToggles.checkImportedClassNameCase`: check a one-part name's written case (on with bleedingEdge).
        const CHECK_IMPORTED_CLASS_NAME_CASE = 1 << 6;
        /// `checkFunctionNameCase`: off by default at every level, independently of class-name case checks.
        const CHECK_FUNCTION_NAME_CASE = 1 << 7;
        /// `cliArgumentsVariablesRegistered`: exempt argc and argv in the main scope only.
        const CLI_ARGUMENTS_VARIABLES_REGISTERED = 1 << 8;
        /// `checkMaybeUndefinedVariables`: enabled by config.level1.neon, independently overridable.
        const CHECK_MAYBE_UNDEFINED_VARIABLES = 1 << 9;
        /// `polluteScopeWithLoopInitialAssignments`: keep for/while entry assignments (on by default).
        const POLLUTE_SCOPE_WITH_LOOP_INITIAL_ASSIGNMENTS = 1 << 10;
        /// `polluteScopeWithAlwaysIterableForeach`: narrow iterable bodies and retain new exit holders.
        const POLLUTE_SCOPE_WITH_ALWAYS_ITERABLE_FOREACH = 1 << 11;
        /// `polluteScopeWithBlock`: retain the actual standalone block scope (on by default).
        const POLLUTE_SCOPE_WITH_BLOCK = 1 << 12;
        /// `featureToggles.checkGenericIterableClasses` (on with bleedingEdge).
        const CHECK_GENERIC_ITERABLE_CLASSES = 1 << 13;
        /// `featureToggles.internalTag` (on with bleedingEdge): the `@internal` usage rules.
        const INTERNAL_TAG = 1 << 14;
        /// `featureToggles.newStaticInAbstractClassStaticMethod` (on with bleedingEdge).
        const NEW_STATIC_IN_ABSTRACT_CLASS_STATIC_METHOD = 1 << 15;
        /// `featureToggles.reportPreciseLineForUnusedFunctionParameter` (on with bleedingEdge): unused closure uses and
        /// constructor parameters are reported on their own line, not the function's.
        const PRECISE_UNUSED_PARAMETER_LINE = 1 << 17;
        /// `featureToggles.bleedingEdge`.
        const BLEEDING_EDGE = 1 << 18;
        /// `featureToggles.unusedLabel` (on with bleedingEdge).
        const UNUSED_LABEL = 1 << 19;
        /// `featureToggles.switchConditionAlwaysFalse` (on with bleedingEdge): SwitchConditionRule.
        const SWITCH_CONDITION = 1 << 20;
        /// `featureToggles.sortWithoutEffect` (on with bleedingEdge): SortWithoutEffectRule.
        const SORT_WITHOUT_EFFECT = 1 << 21;
        /// `featureToggles.checkNonStringableDynamicAccess` (on with bleedingEdge): the `*.nameNotString` checks.
        const CHECK_NON_STRINGABLE_DYNAMIC_ACCESS = 1 << 22;
        /// `featureToggles.reportMethodPurityOverride` (on with bleedingEdge): MethodSignatureRule's purity checks.
        const REPORT_METHOD_PURITY_OVERRIDE = 1 << 23;
        /// `featureToggles.assignToByRefForeachExpr` (on with bleedingEdge): AssignToByRefExprFromForeachRule.
        const ASSIGN_TO_BY_REF_FOREACH_EXPR = 1 << 24;
        /// `featureToggles.checkDynamicConstantNameValues` (on with bleedingEdge): ValueAssignedToGlobalConstantRule and
        /// ValueAssignedToDefineRule.
        const CHECK_DYNAMIC_CONSTANT_NAME_VALUES = 1 << 25;
        /// `featureToggles.checkDateIntervalConstructor` (on with bleedingEdge): DateIntervalInstantiationRule.
        const CHECK_DATE_INTERVAL_CONSTRUCTOR = 1 << 26;
        /// `featureToggles.checkParameterCastableToNumberFunctions` (on with bleedingEdge): ParameterCastableToNumberRule.
        const CHECK_PARAMETER_CASTABLE_TO_NUMBER_FUNCTIONS = 1 << 27;
        /// `featureToggles.checkPrintfParameterTypes` (on with bleedingEdge): PrintfParameterTypeRule.
        const CHECK_PRINTF_PARAMETER_TYPES = 1 << 28;
        /// `checkStrictPrintfPlaceholderTypes`: PrintfParameterTypeRule's strict placeholder types (off by default).
        const CHECK_STRICT_PRINTF_PLACEHOLDER_TYPES = 1 << 29;
        /// `featureToggles.finiteTypesInHaystack` (on with bleedingEdge): ImpossibleInArrayHaystackFiniteTypesRule.
        const FINITE_TYPES_IN_HAYSTACK = 1 << 30;
        /// `featureToggles.unusedVariable` (on with bleedingEdge): UnusedVariableRule.
        const UNUSED_VARIABLE = 1 << 31;
        /// `featureToggles.unusedParameters` (on with bleedingEdge): Unused{Function,Method}ParametersRule and the
        /// `*.unusedParameterFlow`/`closure.unusedUseFlow` findings.
        const UNUSED_PARAMETERS = 1 << 32;
        /// `featureToggles.newOnNonObject` (on with bleedingEdge): InstantiationRule's `new.nonObject`.
        const NEW_ON_NON_OBJECT = 1 << 33;
        /// `checkUninitializedProperties`: UninitializedPropertyRule (config.neon's conditional rule).
        const CHECK_UNINITIALIZED_PROPERTIES = 1 << 34;
        /// `checkMissingCallableSignature`: MissingTypehintCheck's `missingType.callable` (off by default).
        const CHECK_MISSING_CALLABLE_SIGNATURE = 1 << 35;
        /// `deprecationRulesInstalled`: the config includes phpstan-deprecation-rules' rules.neon.
        const DEPRECATION_RULES = 1 << 36;
    }
}

pub struct Project {
    /// The boolean parameters.
    pub options: ProjectOptions,
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
    /// `ignoreErrors` (with the baseline's entries).
    pub ignore_errors: IgnoredErrors,
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
    /// `checkExplicitMixed`: RuleLevelHelper's explicit `mixed` checks; `None` follows the level (on from 9).
    pub check_explicit_mixed: Option<bool>,
    /// `checkImplicitMixed`: RuleLevelHelper's implicit `mixed` checks; `None` follows the level (on at 10).
    pub check_implicit_mixed: Option<bool>,
    /// `treatPhpDocTypesAsCertain`: independently overridable; `None` defaults to true. Used by IssetCheck only.
    pub treat_php_doc_types_as_certain: Option<bool>,
    /// Every analysed path is a file (FileFinderResult's `isOnlyFiles()`).
    pub only_files: bool,
    /// `dynamicConstantNames`' entries with a configured type (name, type string).
    pub dynamic_constant_types: Vec<(String, String)>,
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

bitflags::bitflags! {
    /// phpstan-strict-rules' `strictRules.*` switches of the ported rules.
    #[derive(Clone, Copy, Default)]
    pub struct StrictRules: u16 {
        /// `closureUsesThis`: ClosureUsesThisRule.
        const CLOSURE_USES_THIS = 1;
        /// `requireParentConstructorCall`: RequireParentConstructCallRule.
        const REQUIRE_PARENT_CONSTRUCTOR_CALL = 1 << 1;
        /// `matchingInheritedMethodNames`: WrongCaseOfInheritedMethodRule.
        const MATCHING_INHERITED_METHOD_NAMES = 1 << 2;
        /// `dynamicCallOnStaticMethod`: DynamicCallOnStaticMethodsRule and DynamicCallOnStaticMethodsCallableRule.
        const DYNAMIC_CALL_ON_STATIC_METHOD = 1 << 3;
        /// `booleansInConditions`: BooleanInBooleanAndRule, BooleanInBooleanOrRule and BooleanInBooleanNotRule.
        const BOOLEANS_IN_CONDITIONS = 1 << 4;
        /// `numericOperandsInArithmeticOperators`: OperandsInArithmetic*Rule and OperandInArithmetic*Rule.
        const NUMERIC_OPERANDS_IN_ARITHMETIC_OPERATORS = 1 << 5;
        /// `booleansInLoopConditions`: BooleanInWhileConditionRule and BooleanInDoWhileConditionRule.
        const BOOLEANS_IN_LOOP_CONDITIONS = 1 << 6;
        /// `uselessCast`: UselessCastRule.
        const USELESS_CAST = 1 << 7;
        /// `switchConditionsMatchingType`: MatchingTypeInSwitchCaseConditionRule.
        const SWITCH_CONDITIONS_MATCHING_TYPE = 1 << 8;
    }
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
        let mut rules = StrictRules::empty();
        rules.set(StrictRules::CLOSURE_USES_THIS, on("closureUsesThis"));
        rules.set(StrictRules::REQUIRE_PARENT_CONSTRUCTOR_CALL, on("requireParentConstructorCall"));
        rules.set(StrictRules::MATCHING_INHERITED_METHOD_NAMES, on("matchingInheritedMethodNames"));
        rules.set(StrictRules::DYNAMIC_CALL_ON_STATIC_METHOD, on("dynamicCallOnStaticMethod"));
        rules.set(StrictRules::BOOLEANS_IN_CONDITIONS, on("booleansInConditions"));
        rules.set(StrictRules::NUMERIC_OPERANDS_IN_ARITHMETIC_OPERATORS, on("numericOperandsInArithmeticOperators"));
        rules.set(StrictRules::BOOLEANS_IN_LOOP_CONDITIONS, on("booleansInLoopConditions"));
        rules.set(StrictRules::USELESS_CAST, on("uselessCast"));
        rules.set(StrictRules::SWITCH_CONDITIONS_MATCHING_TYPE, on("switchConditionsMatchingType"));
        rules
    }
}

bitflags::bitflags! {
    /// The `exceptions.*` switches and `checkTooWideThrowTypesInProtectedAndPublicMethods`.
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct ExceptionChecks: u8 {
        /// `implicitThrows`: a call without `@throws` may throw anything.
        const IMPLICIT_THROWS = 1;
        /// `reportUncheckedExceptionDeadCatch`.
        const REPORT_UNCHECKED_EXCEPTION_DEAD_CATCH = 1 << 1;
        /// `check.missingCheckedExceptionInThrows`: the MissingCheckedExceptionIn*ThrowsRule rules.
        const MISSING_CHECKED_EXCEPTION_IN_THROWS = 1 << 2;
        /// `check.tooWideThrowType`: the TooWide*ThrowTypeRule rules (level 4).
        const TOO_WIDE_THROW_TYPE = 1 << 3;
        /// `check.tooWideImplicitThrowType`: TooWideMethodThrowTypeRule also for an inherited `@throws`.
        const TOO_WIDE_IMPLICIT_THROW_TYPE = 1 << 4;
        /// `check.throwTypeCovariance`: MethodThrowTypeCovarianceRule.
        const THROW_TYPE_COVARIANCE = 1 << 5;
        /// `checkTooWideThrowTypesInProtectedAndPublicMethods`.
        const CHECK_TOO_WIDE_THROW_TYPES_IN_PROTECTED_AND_PUBLIC_METHODS = 1 << 6;
    }
}

/// The `exceptions.*` parameters and `checkTooWideThrowTypesInProtectedAndPublicMethods` (config.neon's defaults).
#[derive(Clone, Debug, PartialEq)]
pub struct Exceptions {
    pub checks: ExceptionChecks,
    /// DefaultExceptionTypeResolver's `uncheckedExceptionRegexes`, `uncheckedExceptionClasses`,
    /// `checkedExceptionRegexes` and `checkedExceptionClasses`.
    pub unchecked_exception_regexes: Vec<String>,
    pub unchecked_exception_classes: Vec<String>,
    pub checked_exception_regexes: Vec<String>,
    pub checked_exception_classes: Vec<String>,
}

impl Default for Exceptions {
    fn default() -> Self {
        Exceptions {
            checks: ExceptionChecks::IMPLICIT_THROWS
                | ExceptionChecks::REPORT_UNCHECKED_EXCEPTION_DEAD_CATCH
                | ExceptionChecks::TOO_WIDE_THROW_TYPE,
            unchecked_exception_regexes: Vec::new(),
            unchecked_exception_classes: vec!["Error".into()],
            checked_exception_regexes: Vec::new(),
            checked_exception_classes: Vec::new(),
        }
    }
}

impl Exceptions {
    fn from_params(params: &Array) -> Exceptions {
        let d = Exceptions::default();
        let exceptions = params.get_str("exceptions").and_then(Value::as_array);
        let check = exceptions.and_then(|e| e.get_str("check")).and_then(Value::as_array);
        let mut checks = d.checks;
        let mut flag = |a: Option<&Array>, k: &str, which: ExceptionChecks| {
            if let Some(Value::Bool(b)) = a.and_then(|a| a.get_str(k)) {
                checks.set(which, *b);
            }
        };
        flag(exceptions, "implicitThrows", ExceptionChecks::IMPLICIT_THROWS);
        flag(exceptions, "reportUncheckedExceptionDeadCatch", ExceptionChecks::REPORT_UNCHECKED_EXCEPTION_DEAD_CATCH);
        flag(check, "missingCheckedExceptionInThrows", ExceptionChecks::MISSING_CHECKED_EXCEPTION_IN_THROWS);
        flag(check, "tooWideThrowType", ExceptionChecks::TOO_WIDE_THROW_TYPE);
        flag(check, "tooWideImplicitThrowType", ExceptionChecks::TOO_WIDE_IMPLICIT_THROW_TYPE);
        flag(check, "throwTypeCovariance", ExceptionChecks::THROW_TYPE_COVARIANCE);
        flag(
            Some(params),
            "checkTooWideThrowTypesInProtectedAndPublicMethods",
            ExceptionChecks::CHECK_TOO_WIDE_THROW_TYPES_IN_PROTECTED_AND_PUBLIC_METHODS,
        );
        let list = |k: &str, default: Vec<String>| match exceptions.and_then(|e| e.get_str(k)).and_then(Value::as_array) {
            Some(a) => a.values().filter_map(Value::as_str).map(str::to_string).collect(),
            None => default,
        };
        Exceptions {
            checks,
            unchecked_exception_regexes: list("uncheckedExceptionRegexes", d.unchecked_exception_regexes),
            unchecked_exception_classes: list("uncheckedExceptionClasses", d.unchecked_exception_classes),
            checked_exception_regexes: list("checkedExceptionRegexes", d.checked_exception_regexes),
            checked_exception_classes: list("checkedExceptionClasses", d.checked_exception_classes),
        }
    }
}

bitflags::bitflags! {
    /// TooWideTypeCheck's `featureToggles.reportTooWideBool` (on with bleedingEdge) and
    /// `featureToggles.reportNestedTooWideType`, and TooWideMethodParameterOutTypeRule's
    /// `checkTooWideParameterOutInProtectedAndPublicMethods`.
    #[derive(Clone, Copy, Default)]
    pub struct TooWide: u8 {
        const REPORT_BOOL = 1;
        const REPORT_NESTED = 1 << 1;
        const PARAM_OUT_IN_PROTECTED_AND_PUBLIC_METHODS = 1 << 2;
    }
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
        let mut options = ProjectOptions::empty();
        options.set(ProjectOptions::CHECK_MAGIC_DIR_IN_INCLUDE, toggle("magicDirInInclude"));
        options.set(ProjectOptions::USE_PATH_CONSTANTS_AS_CONSTANT_STRING, flag("usePathConstantsAsConstantString"));
        options.set(ProjectOptions::REPORT_IGNORES_WITHOUT_COMMENTS, flag("reportIgnoresWithoutComments"));
        options.set(ProjectOptions::REPORT_UNMATCHED_IGNORES, report_unmatched_ignores);
        // config.neon's false, which config.level2.neon turns on
        options.set(ProjectOptions::CHECK_CLASS_CASE_SENSITIVITY, flag("checkClassCaseSensitivity"));
        options.set(ProjectOptions::CHECK_INTERNAL_CLASS_CASE_SENSITIVITY, flag("checkInternalClassCaseSensitivity"));
        options.set(ProjectOptions::CHECK_IMPORTED_CLASS_NAME_CASE, toggle("checkImportedClassNameCase"));
        options.set(ProjectOptions::CHECK_FUNCTION_NAME_CASE, flag("checkFunctionNameCase"));
        options.set(ProjectOptions::CLI_ARGUMENTS_VARIABLES_REGISTERED, flag("cliArgumentsVariablesRegistered"));
        options.set(ProjectOptions::CHECK_MAYBE_UNDEFINED_VARIABLES, flag("checkMaybeUndefinedVariables"));
        options.set(ProjectOptions::POLLUTE_SCOPE_WITH_LOOP_INITIAL_ASSIGNMENTS, !matches!(params.get_str("polluteScopeWithLoopInitialAssignments"), Some(Value::Bool(false))));
        options.set(ProjectOptions::POLLUTE_SCOPE_WITH_ALWAYS_ITERABLE_FOREACH, !matches!(params.get_str("polluteScopeWithAlwaysIterableForeach"), Some(Value::Bool(false))));
        options.set(ProjectOptions::POLLUTE_SCOPE_WITH_BLOCK, !matches!(params.get_str("polluteScopeWithBlock"), Some(Value::Bool(false))));
        options.set(ProjectOptions::CHECK_GENERIC_ITERABLE_CLASSES, toggle("checkGenericIterableClasses"));
        options.set(ProjectOptions::INTERNAL_TAG, toggle("internalTag"));
        options.set(ProjectOptions::NEW_STATIC_IN_ABSTRACT_CLASS_STATIC_METHOD, toggle("newStaticInAbstractClassStaticMethod"));
        options.set(ProjectOptions::BLEEDING_EDGE, toggle("bleedingEdge"));
        options.set(ProjectOptions::UNUSED_LABEL, toggle("unusedLabel"));
        options.set(ProjectOptions::SWITCH_CONDITION, toggle("switchConditionAlwaysFalse"));
        options.set(ProjectOptions::SORT_WITHOUT_EFFECT, toggle("sortWithoutEffect"));
        options.set(ProjectOptions::CHECK_NON_STRINGABLE_DYNAMIC_ACCESS, toggle("checkNonStringableDynamicAccess"));
        options.set(ProjectOptions::REPORT_METHOD_PURITY_OVERRIDE, toggle("reportMethodPurityOverride"));
        options.set(ProjectOptions::ASSIGN_TO_BY_REF_FOREACH_EXPR, toggle("assignToByRefForeachExpr"));
        options.set(ProjectOptions::CHECK_DYNAMIC_CONSTANT_NAME_VALUES, toggle("checkDynamicConstantNameValues"));
        options.set(ProjectOptions::CHECK_DATE_INTERVAL_CONSTRUCTOR, toggle("checkDateIntervalConstructor"));
        options.set(ProjectOptions::CHECK_PARAMETER_CASTABLE_TO_NUMBER_FUNCTIONS, toggle("checkParameterCastableToNumberFunctions"));
        options.set(ProjectOptions::CHECK_PRINTF_PARAMETER_TYPES, toggle("checkPrintfParameterTypes"));
        options.set(ProjectOptions::CHECK_STRICT_PRINTF_PLACEHOLDER_TYPES, flag("checkStrictPrintfPlaceholderTypes"));
        options.set(ProjectOptions::FINITE_TYPES_IN_HAYSTACK, toggle("finiteTypesInHaystack"));
        options.set(ProjectOptions::UNUSED_VARIABLE, toggle("unusedVariable"));
        options.set(ProjectOptions::UNUSED_PARAMETERS, toggle("unusedParameters"));
        options.set(ProjectOptions::NEW_ON_NON_OBJECT, toggle("newOnNonObject"));
        options.set(ProjectOptions::CHECK_UNINITIALIZED_PROPERTIES, flag("checkUninitializedProperties"));
        options.set(ProjectOptions::CHECK_MISSING_CALLABLE_SIGNATURE, flag("checkMissingCallableSignature"));
        options.set(ProjectOptions::DEPRECATION_RULES, flag("deprecationRulesInstalled"));
        options.set(ProjectOptions::PRECISE_UNUSED_PARAMETER_LINE, toggle("reportPreciseLineForUnusedFunctionParameter"));
        Project {
            options,
            analysed,
            scanned,
            autoloaded,
            php_version,
            current_working_directory: root.to_path_buf(),
            include_paths: vec![PathBuf::from(".")],
            ignore_errors: IgnoredErrors::new(ignore_errors, report_unmatched_ignores),
            level,
            check_function_argument_types: optional_flag("checkFunctionArgumentTypes"),
            check_arguments_passed_by_reference: optional_flag("checkArgumentsPassedByReference"),
            check_this_only: optional_flag("checkThisOnly"),
            check_advanced_isset: optional_flag("checkAdvancedIsset"),
            check_explicit_mixed: optional_flag("checkExplicitMixed"),
            check_implicit_mixed: optional_flag("checkImplicitMixed"),
            treat_php_doc_types_as_certain: optional_flag("treatPhpDocTypesAsCertain"),
            only_files: strings(params, "paths").iter().all(|p| Path::new(p).is_file()),
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
            strict_rules: StrictRules::from_params(params),
            exceptions: Exceptions::from_params(params),
            too_wide: {
                let mut too_wide = TooWide::empty();
                too_wide.set(TooWide::REPORT_BOOL, toggle("reportTooWideBool"));
                too_wide.set(TooWide::REPORT_NESTED, toggle("reportNestedTooWideType"));
                too_wide.set(
                    TooWide::PARAM_OUT_IN_PROTECTED_AND_PUBLIC_METHODS,
                    flag("checkTooWideParameterOutInProtectedAndPublicMethods"),
                );
                too_wide
            },
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
                // Larastan's StubFilesExtension adds its own
                .chain(match params.get_str("noModelMake") {
                    Some(_) => crate::larastan::stubs::files(root),
                    None => Vec::new(),
                })
                .collect(),
            reported: None,
            method_rules: MethodRules::default(),
        }
    }

    /// PHPStan's defaults (no `ignoreErrors`, unmatched ignores reported) over the given files.
    pub fn new(analysed: Vec<PathBuf>, scanned: Vec<PathBuf>) -> Project {
        let paths: Vec<String> = analysed.iter().map(|p| absolute(p)).collect();
        let mut options = ProjectOptions::empty();
        options.set(ProjectOptions::CHECK_MAGIC_DIR_IN_INCLUDE, false);
        options.set(ProjectOptions::USE_PATH_CONSTANTS_AS_CONSTANT_STRING, false);
        options.set(ProjectOptions::REPORT_UNMATCHED_IGNORES, true);
        options.set(ProjectOptions::REPORT_IGNORES_WITHOUT_COMMENTS, false);
        options.set(ProjectOptions::CHECK_CLASS_CASE_SENSITIVITY, false);
        options.set(ProjectOptions::CHECK_INTERNAL_CLASS_CASE_SENSITIVITY, false);
        options.set(ProjectOptions::CHECK_IMPORTED_CLASS_NAME_CASE, false);
        options.set(ProjectOptions::CHECK_FUNCTION_NAME_CASE, false);
        options.set(ProjectOptions::CLI_ARGUMENTS_VARIABLES_REGISTERED, true);
        options.set(ProjectOptions::CHECK_MAYBE_UNDEFINED_VARIABLES, false);
        options.set(ProjectOptions::POLLUTE_SCOPE_WITH_LOOP_INITIAL_ASSIGNMENTS, true);
        options.set(ProjectOptions::POLLUTE_SCOPE_WITH_ALWAYS_ITERABLE_FOREACH, true);
        options.set(ProjectOptions::POLLUTE_SCOPE_WITH_BLOCK, true);
        options.set(ProjectOptions::CHECK_GENERIC_ITERABLE_CLASSES, false);
        options.set(ProjectOptions::INTERNAL_TAG, false);
        options.set(ProjectOptions::NEW_STATIC_IN_ABSTRACT_CLASS_STATIC_METHOD, false);
        options.set(ProjectOptions::BLEEDING_EDGE, false);
        options.set(ProjectOptions::UNUSED_LABEL, false);
        options.set(ProjectOptions::SWITCH_CONDITION, false);
        options.set(ProjectOptions::SORT_WITHOUT_EFFECT, false);
        options.set(ProjectOptions::CHECK_NON_STRINGABLE_DYNAMIC_ACCESS, false);
        options.set(ProjectOptions::REPORT_METHOD_PURITY_OVERRIDE, false);
        options.set(ProjectOptions::ASSIGN_TO_BY_REF_FOREACH_EXPR, false);
        options.set(ProjectOptions::CHECK_DYNAMIC_CONSTANT_NAME_VALUES, false);
        options.set(ProjectOptions::CHECK_DATE_INTERVAL_CONSTRUCTOR, false);
        options.set(ProjectOptions::CHECK_PARAMETER_CASTABLE_TO_NUMBER_FUNCTIONS, false);
        options.set(ProjectOptions::CHECK_PRINTF_PARAMETER_TYPES, false);
        options.set(ProjectOptions::CHECK_STRICT_PRINTF_PLACEHOLDER_TYPES, false);
        options.set(ProjectOptions::FINITE_TYPES_IN_HAYSTACK, false);
        options.set(ProjectOptions::UNUSED_VARIABLE, false);
        options.set(ProjectOptions::UNUSED_PARAMETERS, false);
        options.set(ProjectOptions::NEW_ON_NON_OBJECT, false);
        options.set(ProjectOptions::CHECK_UNINITIALIZED_PROPERTIES, false);
        options.set(ProjectOptions::CHECK_MISSING_CALLABLE_SIGNATURE, false);
        options.set(ProjectOptions::DEPRECATION_RULES, false);
        options.set(ProjectOptions::PRECISE_UNUSED_PARAMETER_LINE, false);
        Project {
            options,
            relative_path: RelativePath::of(&paths),
            analysed,
            scanned,
            autoloaded: Vec::new(),
            php_version: RUNTIME_PHP_VERSION,
            current_working_directory: std::env::current_dir().unwrap_or_default(),
            include_paths: vec![PathBuf::from(".")],
            ignore_errors: IgnoredErrors::new(&Array::new(), true),
            level: 0,
            check_function_argument_types: None,
            check_arguments_passed_by_reference: None,
            check_this_only: None,
            check_advanced_isset: None,
            check_explicit_mixed: None,
            check_implicit_mixed: None,
            treat_php_doc_types_as_certain: None,
            only_files: true,
            dynamic_constant_types: Vec::new(),
            strict_rules: StrictRules::default(),
            exceptions: Exceptions::default(),
            too_wide: TooWide::default(),
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
