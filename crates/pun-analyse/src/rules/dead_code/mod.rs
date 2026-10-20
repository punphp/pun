//! PHPStan's dead-code rules. Expected diagnostics come from the pinned PHPStan rule fixtures.

mod conditions;
mod finally;
mod function_discard;
mod no_discard;
mod noop;
mod unreachable;
mod unused_private;

pub(super) use conditions::{
    Facts as ConditionFacts, binary, boolean_not, do_while, facts as condition_facts, if_statement,
    ternary, while_loop,
};
pub(super) use unreachable::find as unreachable_statements;

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

pub(super) fn flow_types(program: &Program, level: u8) -> Vec<Span> {
    if level < 4 {
        return Vec::new();
    }
    let mut spans = finally::typed(program);
    spans.extend(unused_private::typed(program));
    spans.extend(conditions::typed(program));
    spans
}

pub(super) use unused_private::class as unused_private;

pub(super) fn statement(c: &mut Checker, s: &ExpressionStatement) {
    no_discard::statement(c, s);
    function_discard::statement(c, s);
    noop::statement(c, s);
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
fn node_name<'arena>(names: &ResolvedNames<'arena>, id: &Identifier<'arena>) -> &'arena [u8] {
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

/// UnusedClosureUsesRule, with UnusedFunctionParametersCheck's lexical variable walk.
pub(super) fn closure(c: &mut Checker, closure: &Closure) {
    if c.level < 1 {
        return;
    }
    let Some(uses) = &closure.use_clause else {
        return;
    };
    let used = used_variables(c, &closure.body);
    for capture in uses.variables.iter() {
        if !used.contains(capture.variable.name) && !used.contains(b"*".as_slice()) {
            let at = if c.precise_unused_parameter_line {
                capture.variable.span()
            } else {
                closure.span()
            };
            c.report(
                at,
                "closure.unusedUse",
                format!(
                    "Anonymous function has an unused use {}.",
                    String::from_utf8_lossy(capture.variable.name)
                ),
                None,
            );
        }
    }
}

/// UnusedConstructorParametersRule. Attribute constructors, promoted properties and interface contracts are exempt.
pub(super) fn constructor(c: &mut Checker, method: &Method, name: &str) {
    if c.level < 1 || !name.eq_ignore_ascii_case("__construct") {
        return;
    }
    let MethodBody::Concrete(body) = &method.body else {
        return;
    };
    let class = c.class_scope().expect("a method has class metadata");
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
    let subject = if c.in_anonymous_class() {
        "an anonymous class".into()
    } else {
        format!("class {}", c.display(class))
    };
    let used = used_variables(c, body);
    for param in method.parameter_list.parameters.iter() {
        if param.modifiers.is_empty()
            && !used.contains(param.variable.name)
            && !used.contains(b"*".as_slice())
        {
            let at = if c.precise_unused_parameter_line {
                param.variable.span()
            } else {
                method.span()
            };
            c.report(
                at,
                "constructor.unusedParameter",
                format!(
                    "Constructor of {subject} has an unused parameter {}.",
                    String::from_utf8_lossy(param.variable.name)
                ),
                None,
            );
        }
    }
}

fn used_variables(c: &Checker, body: &Block) -> HashSet<Vec<u8>> {
    let mut used = UsedVariables {
        c,
        names: HashSet::new(),
    };
    used.walk_block(body, &mut ());
    used.names
}

/// The upstream walk counts writes as uses, descends into arrow functions and named declarations, but only
/// visits the `use` list of a nested closure. `*` means a dynamic operation may read every defined variable.
struct UsedVariables<'c, 'a, 'arena> {
    c: &'c Checker<'a, 'arena>,
    names: HashSet<Vec<u8>>,
}

impl UsedVariables<'_, '_, '_> {
    fn strings(&mut self, span: Span, unknown_reads_all: bool) {
        let strings: Vec<_> = self
            .c
            .types
            .get(&span)
            .into_iter()
            .flat_map(|t| t.types.iter())
            .filter_map(|t| t.get_literal_string_value())
            .collect();
        if strings.is_empty() && unknown_reads_all {
            self.names.insert(b"*".to_vec());
        }
        for name in strings {
            let mut variable = b"$".to_vec();
            variable.extend_from_slice(name);
            self.names.insert(variable);
        }
    }
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for UsedVariables<'_, '_, '_> {
    fn walk_in_direct_variable(&mut self, v: &'ast DirectVariable<'arena>, _: &mut ()) {
        self.names.insert(v.name.to_vec());
    }
    fn walk_in_indirect_variable(&mut self, v: &'ast IndirectVariable<'arena>, _: &mut ()) {
        self.strings(v.expression.span(), true);
    }
    fn walk_in_nested_variable(&mut self, v: &'ast NestedVariable<'arena>, _: &mut ()) {
        self.strings(v.variable.span(), true);
    }
    fn walk_closure(&mut self, f: &'ast Closure<'arena>, _: &mut ()) {
        if let Some(uses) = &f.use_clause {
            for capture in uses.variables.iter() {
                self.names.insert(capture.variable.name.to_vec());
            }
        }
    }
    fn walk_in_construct(&mut self, c: &'ast Construct<'arena>, _: &mut ()) {
        if matches!(
            c,
            Construct::Include(_)
                | Construct::IncludeOnce(_)
                | Construct::Require(_)
                | Construct::RequireOnce(_)
                | Construct::Eval(_)
        ) {
            self.names.insert(b"*".to_vec());
        }
    }
    fn walk_in_function_call(&mut self, call: &'ast FunctionCall<'arena>, _: &mut ()) {
        let Expression::Identifier(id) = call.function else {
            return;
        };
        let fallback = match id {
            Identifier::Local(l) if !self.c.names.is_imported(&l.span) => Some(l.value),
            _ => None,
        };
        let name = std::iter::once(self.c.names.get(&id.span()))
            .chain(fallback)
            .find_map(|n| self.c.function(n))
            .map(|(n, _)| n);
        if matches!(name.as_deref(), Some("func_get_args" | "get_defined_vars")) {
            self.names.insert(b"*".to_vec());
        }
        if node_name(self.c.names, id) == b"compact" {
            for arg in call.argument_list.arguments.iter() {
                self.strings(arg.value().span(), false);
            }
        }
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
