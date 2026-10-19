//! `lint/security/no-debug-calls`, ported from two upstream rules:
//!
//! - Mago 1.57.0's linter rule `no-debug-symbols` (rule/security/no_debug_symbols.rs): a call of a debug function
//!   (`var_dump`, `dd`, `dump`, the Xdebug functions, ...) is reported; `var_export` and `print_r` with a second
//!   argument return the text and are allowed.
//! - pest-plugin-rector's `RemoveDebugExpectationsRector`: a `->dump()`, `->dd()` or `->ray()` in a Pest `expect()`
//!   chain is removed.

use mago_syntax::cst::{Access, Call, ClassLikeMemberSelector, Expression, Node, NodeKind};
use pun_problem::{Edit, Fix, Safety, Span};

use super::lint_b_names::{function_call_matches, function_call_matches_any};
use super::rector_code_quality::call_to;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct NoDebugCalls;

/// Mago's `DEBUG_FUNCTIONS` (rule/utils/consts.rs).
const DEBUG_FUNCTIONS: [&str; 53] = [
    "var_dump",
    "var_export",
    "print_r",
    "debug_zval_dump",
    "debug_print_backtrace",
    "phpinfo",
    "mago\\inspect",
    "mago\\confirm",
    "phpstan\\dumpType",
    "dump",
    "dd",
    "xdebug_debug_zval",
    "xdebug_break",
    "xdebug_call_class",
    "xdebug_call_file",
    "xdebug_call_function",
    "xdebug_call_line",
    "xdebug_code_coverage_started",
    "xdebug_connect_to_client",
    "xdebug_debug_zval",
    "xdebug_debug_zval_stdout",
    "xdebug_dump_superglobals",
    "xdebug_get_code_coverage",
    "xdebug_get_collected_errors",
    "xdebug_get_function_count",
    "xdebug_get_function_stack",
    "xdebug_get_gc_run_count",
    "xdebug_get_gc_total_collected_roots",
    "xdebug_get_gcstats_filename",
    "xdebug_get_headers",
    "xdebug_get_monitored_functions",
    "xdebug_get_profiler_filename",
    "xdebug_get_stack_depth",
    "xdebug_get_tracefile_name",
    "xdebug_info",
    "xdebug_is_debugger_active",
    "xdebug_memory_usage",
    "xdebug_notify",
    "xdebug_peak_memory_usage",
    "xdebug_print_function_stack",
    "xdebug_set_filter",
    "xdebug_start_code_coverage",
    "xdebug_start_error_collection",
    "xdebug_start_function_monitor",
    "xdebug_start_gcstats",
    "xdebug_start_trace",
    "xdebug_stop_code_coverage",
    "xdebug_stop_error_collection",
    "xdebug_stop_function_monitor",
    "xdebug_stop_gcstats",
    "xdebug_stop_trace",
    "xdebug_time_index",
    "xdebug_var_dump",
];

/// `RemoveDebugExpectationsRector::DEBUG_METHODS`.
const DEBUG_METHODS: [&[u8]; 3] = [b"dump", b"dd", b"ray"];

impl Rule for NoDebugCalls {
    fn id(&self) -> &'static str {
        "lint/security/no-debug-calls"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::FunctionCall, NodeKind::MethodCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        match node {
            Node::FunctionCall(call) => {
                // Allow `var_export` and `print_r` if they have the second argument
                if function_call_matches_any(ancestors, call, &["var_export", "print_r"]).is_some()
                    && call.argument_list.arguments.len() > 1
                {
                    return;
                }
                if DEBUG_FUNCTIONS
                    .iter()
                    .any(|f| function_call_matches(ancestors, call, f))
                {
                    out.push(Report {
                        span: node_span(call),
                        message: "Do not commit debug functions.".to_string(),
                        help: Some("Remove this function call.".to_string()),
                        fix: None,
                    });
                }
            }
            Node::MethodCall(call) => {
                let ClassLikeMemberSelector::Identifier(name) = &call.method else {
                    return;
                };
                if !DEBUG_METHODS.contains(&name.value) || !is_expect_chain(call.object) {
                    return;
                }
                // The call becomes its object: `expect($a)->dump()->toBe(1)` is `expect($a)->toBe(1)`.
                let span = node_span(call);
                let removed = Span::new(node_span(call.object).end, span.end);
                out.push(Report {
                    span,
                    message: format!(
                        "Remove the debug call `->{}()` from the expectation.",
                        String::from_utf8_lossy(name.value)
                    ),
                    help: None,
                    fix: Some(Fix {
                        safety: Safety::Unsafe,
                        edits: vec![Edit::new(removed, "")],
                    }),
                });
            }
            _ => {}
        }
    }
}

/// `AbstractRector::isExpectChain` on the object of a method call: under its method calls and property fetches is a
/// call of `expect()`.
fn is_expect_chain<'a>(mut object: &'a Expression<'a>) -> bool {
    loop {
        match unparenthesize(object) {
            Expression::Call(Call::Method(call)) => object = call.object,
            Expression::Access(Access::Property(fetch)) => object = fetch.object,
            object => return call_to(object, "expect").is_some(),
        }
    }
}
