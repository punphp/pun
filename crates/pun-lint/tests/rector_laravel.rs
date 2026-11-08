//! rector-laravel's own fixtures for the ported `laravel` rules, copied verbatim from driftingly/rector-laravel 2.7.3
//! (its source at 699f25b, tests/Rector/...) into tests/rector/<Rector class>/Fixture. Oracle/ holds extra inputs with
//! what Rector 2.8.4 and rector-laravel 2.7.3 (`.oracle/rector-ecosystem`, the rule alone, `phpVersion(80400)`) wrote
//! for them; Pun/ holds the cases where Pun keeps the code valid or the meaning where Rector does not, its expected
//! output derived by hand. A fixture is `input -----\n expected`, or only the input when the rule must not change it.
//! Each one must pass as `format(pun lint --fix) == format(expected)` with format being the laravel preset, and the text
//! edits must give the expected bytes before format too.

mod lint_c;

use lint_c::run_class;
use pun_lint::Rule;
use pun_lint::rules::{
    NoNegatedBlankFilled, PreferAbortIf, PreferBackHelper, PreferEnvHelper,
    PreferExceptionClassString, PreferFacadeOverApp, PreferNullsafe, PreferReportIf,
    PreferRequestServer, PreferSessionFacade, PreferSleepFacade, PreferStrStartsWith,
    PreferThrowIf, PreferToRouteHelper, PreferToday,
};
use pun_lint::rules::{
    PreferDateFacade, PreferDefaultArgument, PreferExplicitJsonCall, PreferRequestFacade,
    PreferResolveHelper, PreferTravelTo,
};

fn check(rule: impl Rule + 'static, class: &str) {
    run_class(vec![Box::new(rule)], class);
}

#[test]
fn not_filled_blank_func_call_to_blank_filled_func_call() {
    check(
        NoNegatedBlankFilled,
        "NotFilledBlankFuncCallToBlankFilledFuncCallRector",
    );
}

#[test]
fn redirect_back_to_back_helper() {
    check(PreferBackHelper, "RedirectBackToBackHelperRector");
}

#[test]
fn redirect_route_to_to_route_helper() {
    check(PreferToRouteHelper, "RedirectRouteToToRouteHelperRector");
}

#[test]
fn env_variable_to_env_helper() {
    check(PreferEnvHelper, "EnvVariableToEnvHelperRector");
}

#[test]
fn throw_if_and_throw_unless_exceptions_to_use_class_string() {
    check(
        PreferExceptionClassString,
        "ThrowIfAndThrowUnlessExceptionsToUseClassStringRector",
    );
}

#[test]
fn sleep_func_to_sleep_static_call() {
    check(PreferSleepFacade, "SleepFuncToSleepStaticCallRector");
}

#[test]
fn now_func_with_start_of_day_method_call_to_today_func() {
    check(
        PreferToday,
        "NowFuncWithStartOfDayMethodCallToTodayFuncRector",
    );
}

#[test]
fn server_variable_to_request_facade() {
    check(PreferRequestServer, "ServerVariableToRequestFacadeRector");
}

#[test]
fn sub_str_to_starts_with_or_ends_with_static_method_call() {
    check(
        PreferStrStartsWith,
        "SubStrToStartsWithOrEndsWithStaticMethodCallRector",
    );
}

#[test]
fn optional_to_nullsafe_operator() {
    check(
        PreferNullsafe {
            exclude_methods: vec!["present".to_string()],
        },
        "OptionalToNullsafeOperatorRector",
    );
}

#[test]
fn session_variable_to_session_facade() {
    check(PreferSessionFacade, "SessionVariableToSessionFacadeRector");
}

#[test]
fn abort_if() {
    check(PreferAbortIf, "AbortIfRector");
}

#[test]
fn report_if() {
    check(PreferReportIf, "ReportIfRector");
}

#[test]
fn throw_if() {
    check(PreferThrowIf, "ThrowIfRector");
}

#[test]
fn helper_func_call_to_facade_class() {
    check(PreferFacadeOverApp, "HelperFuncCallToFacadeClassRector");
}

#[test]
fn optional_to_nullsafe_operator_options() {
    let mut rule = PreferNullsafe::default();
    assert!(rule.configure(&serde_json::json!(null)).is_ok());
    assert!(rule.configure(&serde_json::json!(true)).is_err());
    assert!(rule.configure(&serde_json::json!({"other": []})).is_err());
    assert!(
        rule.configure(&serde_json::json!({"exclude-methods": [1]}))
            .is_err()
    );
    rule.configure(&serde_json::json!({"exclude-methods": ["present"]}))
        .unwrap();
    check(rule, "OptionalToNullsafeOperatorRector");
}

#[test]
fn app_to_resolve() {
    check(PreferResolveHelper, "AppToResolveRector");
}

#[test]
fn carbon_to_date_facade() {
    check(PreferDateFacade, "CarbonToDateFacadeRector");
}

#[test]
fn request_variables_to_request_facade() {
    check(PreferRequestFacade, "RequestVariablesToRequestFacadeRector");
}

#[test]
fn apply_default_instead_of_null_coalesce() {
    check(
        PreferDefaultArgument,
        "ApplyDefaultInsteadOfNullCoalesceRector",
    );
}

#[test]
fn json_call_to_explicit_json_call() {
    check(PreferExplicitJsonCall, "JsonCallToExplicitJsonCallRector");
}

#[test]
fn carbon_set_test_now_to_travel_to() {
    check(PreferTravelTo, "CarbonSetTestNowToTravelToRector");
}
