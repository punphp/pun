//! PHPStan's dead-code rules. Expected diagnostics come from the pinned PHPStan rule fixtures.

mod conditions;
mod finally;
mod function_discard;
mod haystack;
mod impure_points;
mod null_coalesce;
mod no_discard;
mod noop;
mod unreachable;
mod unused_private;
mod unused_variable;
pub(super) use unused_variable::check as unused_variables;

pub(super) use conditions::{
    Facts as ConditionFacts, binary, boolean_not, do_while, facts as condition_facts, function_check,
    if_statement, match_expression, match_subject_types, method_check, static_method_check, ternary, while_loop,
};
pub(super) use conditions::switch_statement;
pub(super) mod sort;
pub(super) use sort::call as sort_call;
pub(super) use haystack::call as haystack_call;
pub(crate) use conditions::conditions_in_traits;
pub(crate) use impure_points::resolve as impure_points;
pub(super) use impure_points::{function as impure_function, method as impure_method};
pub(super) use unreachable::{Statements as UnreachableStatements, find as unreachable_statements};
pub(super) use noop::{FLIP, Function, PURE_UNLESS_CALLABLE_IS_IMPURE, function_of};

use std::collections::HashSet;

use mago_codex::ttype::expander::{TypeExpansionOptions, expand_union};
use mago_names::ResolvedNames;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;

use super::{Checker, members};

pub(super) fn try_statement(c: &mut Checker, t: &Try) {
    finally::check(c, t);
}

/// The function-like bodies whose VariableWritesNode a rule reads, so that the analysed program keeps them: the
/// constructors (UnusedConstructorParametersRule), and with `unusedParameters` or `unusedVariable` every body.
pub(in super::super) fn bodies(program: &Program, level: u8, every: bool) -> Vec<Span> {
    struct Bodies(Vec<Span>, bool);
    impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Bodies {
        fn walk_in_function(&mut self, f: &'ast mago_syntax::cst::Function<'arena>, _: &mut ()) {
            if self.1 {
                self.0.push(f.span());
            }
        }
        fn walk_in_method(&mut self, m: &'ast Method<'arena>, _: &mut ()) {
            if self.1 || (m.name.value.eq_ignore_ascii_case(b"__construct") && !m.parameter_list.parameters.is_empty()) {
                self.0.push(m.span());
            }
        }
        fn walk_in_closure(&mut self, f: &'ast Closure<'arena>, _: &mut ()) {
            if self.1 {
                self.0.push(f.span());
            }
        }
    }
    if level < 1 {
        return Vec::new();
    }
    let mut bodies = Bodies(Vec::new(), level >= 4 && every);
    bodies.walk_program(program, &mut ());
    bodies.0
}

pub(super) fn flow_types(program: &Program, level: u8) -> Vec<Span> {
    if level < 4 {
        return Vec::new();
    }
    let mut spans = finally::typed(program);
    spans.extend(unused_private::typed(program));
    spans.extend(conditions::typed(program));
    spans.extend(impure_points::typed(program));
    spans.extend(unused_variable::typed(program));
    spans
}

pub(super) use unused_private::class as unused_private;
pub(super) use null_coalesce::{coalesce as null_coalesce, coalesce_types, empty, isset};

pub(super) fn statement(c: &mut Checker, s: &ExpressionStatement) {
    no_discard::statement(c, s);
    function_discard::statement(c, s);
    noop::statement(c, s);
    impure_points::statement(c, s);
}

pub(super) fn typed_statement(
    s: &ExpressionStatement,
    level: u8,
    php_version: u32,
) -> impl Iterator<Item = Span> {
    [
        no_discard::typed(s, level, php_version),
        function_discard::typed(s, php_version),
    ]
    .into_iter()
    .flatten()
    .chain(if level >= 4 { noop::typed(s) } else { Vec::new() })
}

/// PHP-Parser replaces imported function aliases, but preserves names with namespace fallback.
pub(in super::super) fn node_name<'arena>(names: &ResolvedNames<'arena>, id: &Identifier<'arena>) -> &'arena [u8] {
    if names.is_imported(&id.span()) {
        names.get(&id.span())
    } else {
        id.value().strip_prefix(b"\\").unwrap_or(id.value())
    }
}

/// DefineParametersRule and UselessFunctionReturnValueRule.
pub(super) fn function_call(c: &mut Checker, call: &FunctionCall) {
    let Expression::Identifier(id) = call.function else {
        return;
    };
    if node_name(c.names, id).eq_ignore_ascii_case(b"define")
        && call.argument_list.arguments.len() >= 3
    {
        c.report(call, "argument.unused", "Argument #3 ($case_insensitive) is ignored since declaration of case-insensitive constants is no longer supported.".into(), None);
    }
    if c.level < 4 || c.is_top_level(call.span()) {
        return;
    }
    let fallback = match id {
        Identifier::Local(l) if !c.names.is_imported(&l.span) => Some(l.value),
        _ => None,
    };
    let Some((name, _)) = std::iter::once(c.names.get(&id.span()))
        .chain(fallback)
        .find_map(|n| c.function(n))
    else {
        return;
    };
    let value = match name.as_str() {
        "var_export" => "null",
        "print_r" | "highlight_string" => "true",
        _ => return,
    };
    // ArgumentsNormalizer::reorderArgs: named arguments occupy signature positions; unknown names append.
    let first_name = if name == "highlight_string" {
        b"string".as_slice()
    } else {
        b"value"
    };
    let mut args = Vec::new();
    let mut extra = Vec::new();
    for (i, arg) in call.argument_list.arguments.iter().enumerate() {
        let position = match arg {
            Argument::Positional(_) => Some(i),
            Argument::Named(n) if n.name.value == first_name => Some(0),
            Argument::Named(n) if n.name.value == b"return" => Some(1),
            _ => None,
        };
        if let Some(position) = position {
            args.resize(args.len().max(position + 1), None);
            if args[position].is_none() || matches!(arg, Argument::Positional(_)) {
                args[position] = Some(arg.value());
            }
        } else {
            extra.push(Some(arg.value()));
        }
    }
    if args.first().is_some_and(Option::is_none) {
        return;
    }
    args.extend(extra);
    if args.len() == 1
        || (args.len() >= 2 && args[1].is_none_or(|e| c.type_of(e).is_some_and(|t| t.is_false())))
    {
        c.report(call, "function.uselessReturnValue", format!("Return value of function {name}() is always {value} and the result is printed instead of being returned. Pass in true as parameter #2 $return to return the output instead."), None);
    }
}

/// UnusedClosureUsesRule consumes the semantic producer's VariableWritesNode.
pub(super) fn closure(c: &mut Checker, closure: &Closure) {
    if c.level < 1 {
        return;
    }
    unused_variables(c, closure.span(), Node::Closure(closure));
    let Some(uses) = &closure.use_clause else {
        return;
    };
    let Some(writes) = c.variable_writes.get(&closure.span()) else {
        return;
    };
    if writes.opaque {
        return;
    }
    let flow = c.toggled.unused_flow;
    let mut errors = Vec::new();
    for capture in uses.variables.iter() {
        let span = capture.variable.span();
        let (message, identifier) = if writes.writes.contains_key(&span) {
            // a by-value use imports a value: it is unused unless that value is read on some path
            if writes.used.contains(&span) || writes.all_names {
                continue;
            }
            if writes.read.contains(&span) {
                if !flow || writes.covered.contains(&span) {
                    continue;
                }
                ("Anonymous function has a use {} that only flows into values that are never used.", "closure.unusedUseFlow")
            } else {
                ("Anonymous function has an unused use {}.", "closure.unusedUse")
            }
        } else if writes.is_variable_referenced(mago_word::word(capture.variable.name)) {
            // a by-ref use aliases the outer variable: any mention counts
            continue;
        } else {
            ("Anonymous function has an unused use {}.", "closure.unusedUse")
        };
        let at = if c.on(crate::rules::Switches::PRECISE_UNUSED_PARAMETER_LINE) { span } else { closure.span() };
        errors.push((at, identifier, message.replace("{}", &String::from_utf8_lossy(capture.variable.name))));
    }
    for (at, identifier, message) in errors {
        c.report(at, identifier, message, None);
    }
}

/// UnusedParametersCheck::getUnusedParameterErrors(): `{}` in the messages is the parameter.
#[allow(clippy::too_many_arguments)]
fn unused_parameters(
    c: &mut Checker,
    span: Span,
    parameters: &FunctionLikeParameterList,
    contract: &HashSet<Vec<u8>>,
    unused: (&str, &str),
    exact_line: bool,
    flow: Option<(&str, &str)>,
) {
    let Some(writes) = c.variable_writes.get(&span) else {
        return;
    };
    if writes.opaque {
        return;
    }
    let mut errors = Vec::new();
    for parameter in parameters.parameters.iter() {
        // a promoted parameter is a property: its value is always used
        if !parameter.modifiers.is_empty() || contract.contains(parameter.variable.name) {
            continue;
        }
        let at = parameter.variable.span();
        let (message, identifier) = if writes.writes.contains_key(&at) {
            if writes.used.contains(&at) || writes.all_names {
                continue;
            }
            if writes.read.contains(&at) {
                match flow {
                    Some(flow) if !writes.covered.contains(&at) => flow,
                    _ => continue,
                }
            } else {
                unused
            }
        } else if writes.is_variable_referenced(mago_word::word(parameter.variable.name)) {
            continue;
        } else {
            unused
        };
        errors.push((
            if exact_line { at } else { span },
            identifier,
            message.replace("{}", &String::from_utf8_lossy(parameter.variable.name)),
        ));
    }
    for (at, identifier, message) in errors {
        c.report(at, identifier, message, None);
    }
}

/// getContractReferencedParameterNames(): the subjects of assert tags and the parameters a conditional type in the
/// signature switches on.
fn contract_parameters(signature: &super::declared::Signature) -> HashSet<Vec<u8>> {
    use pun_phpstan::types::{Subject, Type};
    fn conditionals(t: &Type, found: &std::cell::RefCell<HashSet<Vec<u8>>>) {
        if let Type::Conditional(c) = t
            && let Subject::Parameter(name) = &c.subject
        {
            found.borrow_mut().insert(name.clone().into_bytes());
        }
        t.traverse(&|i| {
            conditionals(i, found);
            i.clone()
        });
    }
    // (an assert on `$param->method()` or `$param->property` refers to `$param`)
    let found = std::cell::RefCell::new(
        signature
            .asserts
            .iter()
            .map(|a| format!("${}", a.split(|c: char| !(c.is_alphanumeric() || c == '_')).next().unwrap_or_default()).into_bytes())
            .collect(),
    );
    conditionals(&signature.ret, &found);
    for (_, ty, ..) in &signature.params {
        conditionals(ty, &found);
    }
    // ponytail: decide_type() drops a PHPDoc offset access type PHPStan keeps (`T[($p is 0 ? 0 : key-of<T>)]`)
    for ty in &signature.phpdoc {
        conditionals(ty, &found);
    }
    found.into_inner()
}

/// isNoOpBody(): no statements but comments.
fn no_op(body: &[Statement]) -> bool {
    body.iter().all(|s| matches!(s, Statement::Noop(_)))
}

/// UnusedFunctionParametersRule: level 4 with `featureToggles.unusedParameters`.
pub(super) fn function_parameters(c: &mut Checker, f: &mago_syntax::cst::Function) {
    unused_variables(c, f.span(), Node::Function(f));
    if !c.toggled.unused_parameters || no_op(f.body.statements.as_slice()) || f.parameter_list.parameters.is_empty() {
        return;
    }
    let name = c.resolved_span(f.name.span);
    let signature = super::declared::with_classes(c, || super::declared::function(c, f));
    let contract = contract_parameters(&signature);
    let unused = format!("Function {name}() has an unused parameter {{}}.");
    let flow = format!("Function {name}() has a parameter {{}} that only flows into values that are never used.");
    unused_parameters(
        c,
        f.span(),
        &f.parameter_list,
        &contract,
        (&unused, "function.unusedParameter"),
        true,
        Some((&flow, "function.unusedParameterFlow")),
    );
}

/// UnusedMethodParametersRule (private methods) and UnusedConstructorParametersRule.
pub(super) fn method_parameters(c: &mut Checker, m: &Method, name: &str) {
    let MethodBody::Concrete(body) = &m.body else {
        return;
    };
    unused_variables(c, m.span(), Node::Method(m));
    if m.parameter_list.parameters.is_empty() {
        return;
    }
    let constructor = name.eq_ignore_ascii_case("__construct");
    if constructor {
        if c.level < 1 {
            return;
        }
    } else if !c.toggled.unused_parameters
        || !m.modifiers.contains_private()
        || no_op(body.statements.as_slice())
        || m.name.value.starts_with(b"__")
    {
        return;
    }
    let Some(class) = c.class_scope() else { return };
    if constructor {
        let attributes: Vec<_> = class
            .attributes
            .iter()
            .filter(|a| a.name.as_bytes() == b"Attribute")
            .collect();
        let attribute_class = match attributes.as_slice() {
            [attribute] if attribute.arguments.is_empty() => true,
            [attribute] => attribute
                .arguments
                .iter()
                .find(|a| a.name.is_none())
                .or_else(|| {
                    attribute
                        .arguments
                        .iter()
                        .find(|a| a.name.is_some_and(|n| n.as_bytes() == b"flags"))
                })
                .or_else(|| attribute.arguments.first())
                .and_then(|a| a.value_type.clone())
                .is_some_and(|mut t| {
                    expand_union(c.codebase, &mut t, &TypeExpansionOptions::default());
                    t.get_single_literal_int_value().is_some()
                }),
            _ => false,
        };
        if attribute_class
            || class.all_parent_interfaces.iter().any(|name| {
                c.codebase
                    .get_class_like(name.as_bytes())
                    .is_some_and(|i| members::method(c, i, b"__construct").is_some())
            })
        {
            return;
        }
    }
    let declaring = super::missing_types::declaring(c, m.span()).unwrap_or(class);
    let signature = super::declared::with_classes(c, || super::declared::method(c, m, declaring, class));
    let contract = contract_parameters(&signature);
    if constructor {
        let subject = if c.in_anonymous_class() {
            "Constructor of an anonymous class".to_string()
        } else {
            format!("Constructor of class {}", c.display(class))
        };
        let unused = format!("{subject} has an unused parameter {{}}.");
        let flow = format!("{subject} has a parameter {{}} that only flows into values that are never used.");
        let exact = c.on(crate::rules::Switches::PRECISE_UNUSED_PARAMETER_LINE);
        let flow = c.toggled.unused_flow.then_some((flow.as_str(), "constructor.unusedParameterFlow"));
        unused_parameters(c, m.span(), &m.parameter_list, &contract, (&unused, "constructor.unusedParameter"), exact, flow);
    } else {
        let method = format!("{}::{}()", c.display(class), String::from_utf8_lossy(m.name.value));
        let unused = format!("Method {method} has an unused parameter {{}}.");
        let flow = format!("Method {method} has a parameter {{}} that only flows into values that are never used.");
        unused_parameters(
            c,
            m.span(),
            &m.parameter_list,
            &contract,
            (&unused, "method.unusedParameter"),
            true,
            Some((&flow, "method.unusedParameterFlow")),
        );
    }
}

/// Additional type queries for UnusedFunctionParametersCheck and UselessFunctionReturnValueRule.
pub(super) fn typed_variable(v: &Variable) -> Option<Span> {
    match v {
        Variable::Indirect(v) => Some(v.expression.span()),
        Variable::Nested(v) => Some(v.variable.span()),
        Variable::Direct(_) => None,
    }
}

pub(super) fn typed_arguments(
    call: &FunctionCall,
    names: &mago_names::ResolvedNames,
    level: u8,
) -> Vec<Span> {
    let Expression::Identifier(id) = call.function else {
        return Vec::new();
    };
    if (level >= 1 && node_name(names, id) == b"compact")
        || (level >= 4
            && [b"print_r".as_slice(), b"var_export", b"highlight_string"]
                .iter()
                .any(|name| {
                    names.get(&id.span()).eq_ignore_ascii_case(name)
                        || id.value().eq_ignore_ascii_case(name)
                }))
    {
        return call
            .argument_list
            .arguments
            .iter()
            .map(|a| a.value().span())
            .collect();
    }
    Vec::new()
}
