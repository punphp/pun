//! The `analyse/types/*` rules: PHPStan's type-checking rules of levels 0 to 9, and what they share (statement
//! results in [`flow`]).

mod arguments;
pub(super) mod callables;
mod castable;
mod access;
mod casts;
mod comparison;
mod convert;
pub mod flow;
mod level;
mod inc_dec;
mod missing_return;
mod param_default;
mod property_default;
mod read_only_write;
mod receivers;
pub(super) mod regex;
mod returns;
mod values;
mod dim_fetch;
mod yields;
mod void_match;
mod constant_value;
pub(super) mod properties;
mod offsets;
mod binary_op;
mod too_wide;
pub(super) mod dynamic_names;
mod nullsafe;
mod throws_void;
pub(super) mod throw_points;
mod throws;
pub(super) mod param_out;

use std::collections::HashMap;

use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;

pub(super) use arguments::{arguments, declared_type, is_late};
pub(super) use castable::function_call as castable_call;
pub(super) use casts::{echo, interpolated, print, unary};
pub(super) use comparison::binary;
pub(super) use param_default::{
    anonymous as anonymous_defaults, function as function_defaults, method as method_defaults,
};
pub(super) use inc_dec::{postfix, prefix};
pub(super) use missing_return::{closure, function, get_hooks, method};
pub(super) use read_only_write::assignment;
pub(super) use access::{property_fetch, static_property_fetch};
pub(super) use offsets::accessible as offset_accessible;
pub(super) use offsets::offset_set as offset_types;
pub(super) use properties::{native_unset_property, with_args};
pub(super) use receivers::{check_method, check_static, forwarded_static_call, method_call, static_call};
pub(super) use regex::function_call;
pub(super) use callables::{call as callable_call, call_user_func, pipe as callable_pipe, pipe_by_ref};
pub(super) use returns::{
    arrow_function as arrow_function_returns, closure as closure_returns, function as function_returns,
    hooks as hook_returns, method as method_returns,
};
pub(super) use void_match::void_match;
pub(super) use values::{array, array_access, clone, foreach, throw};
pub(super) use dim_fetch::{array_item, destructuring, dim_fetch};
pub(super) use dim_fetch::watched as dim_fetches;
pub(super) use yields::program as yields_outside;
pub(super) use property_default::property as property_default;
pub(super) use constant_value::{class_constant, define_value, global_constant};
pub(super) use offsets::assignment as offset_assignment;
pub(super) use binary_op::{assignment as assign_operation, binary as binary_operation, pipe as pipe_operation};
pub(super) use throws_void::{
    catches as dead_catches, function as throws_void_function, hooks as throws_void_hooks,
    method as throws_void_method,
};
pub(super) use nullsafe::{method as nullsafe_method, property as nullsafe_property};
pub(super) use throws::{
    covariance as throws_covariance, dead_catches as unthrown_catches, function as throws_function, hooks as throws_hooks, method as throws_method,
};
pub(super) use too_wide::{
    arrow_function as too_wide_arrow_function, closure as too_wide_closure, function as too_wide_function,
    method as too_wide_method, properties as too_wide_properties,
};
pub(super) use param_out::{function as param_out_function, method as param_out_method, self_out};
pub(super) use properties::{assignment as property_assignment, increment as property_increment, unset as property_unset};

/// The identifiers of the rules above level 0 (level-0 ones are in [`super::IDENTIFIERS`]).
pub const IDENTIFIERS: &[&[&str]] = &[callables::IDENTIFIERS, regex::IDENTIFIERS, arguments::IDENTIFIERS, castable::IDENTIFIERS, receivers::IDENTIFIERS, access::IDENTIFIERS, casts::IDENTIFIERS, comparison::IDENTIFIERS, param_default::IDENTIFIERS, void_match::IDENTIFIERS, returns::IDENTIFIERS, values::IDENTIFIERS, dim_fetch::IDENTIFIERS, yields::IDENTIFIERS, property_default::IDENTIFIERS, properties::IDENTIFIERS, constant_value::IDENTIFIERS, offsets::IDENTIFIERS, binary_op::IDENTIFIERS, too_wide::IDENTIFIERS, dynamic_names::IDENTIFIERS, nullsafe::IDENTIFIERS, throws_void::IDENTIFIERS, inc_dec::IDENTIFIERS, throws::IDENTIFIERS, param_out::IDENTIFIERS];

/// The expressions whose type the statement results read, in the function-likes whose missing return could be
/// reported: those with a non-void return type whose body, by syntax alone, does not always terminate (types only
/// ever make a body terminate: a `never` call, a condition always true, a non-empty array iterated).
pub(super) fn typed_expressions(program: &Program, contents: &[u8], comments: &super::Comments, level: u8, check_arguments: bool, check_this_only: bool) -> Vec<Span> {
    let mut w = Wanted {
        contents,
        comments,
        level,
        check_arguments,
        check_this_only,
        spans: Vec::new(),
    };
    w.walk_program(program, &mut ());
    w.spans
}

struct Wanted<'c> {
    contents: &'c [u8],
    comments: &'c super::Comments,
    level: u8,
    check_arguments: bool,
    check_this_only: bool,
    spans: Vec<Span>,
}

impl Wanted<'_> {
    fn function_like(&mut self, span: Span, ret: Option<&FunctionLikeReturnTypeHint>, b: &Block) {
        let stmts = b.statements.as_slice();
        let returns = match ret {
            Some(r) => !matches!(r.hint, Hint::Void(_)),
            None => self.comments.doc_before(self.contents, span.start.offset as usize)
                .is_some_and(|(d, _)| d.windows(7).any(|w| w == b"return ")),
        };
        if !returns {
            return;
        }
        let stmts = flow::list(stmts);
        let empty = HashMap::new();
        let results = flow::Results {
            types: &empty,
            call_terminations: None,
            contents: self.contents,
            codebase: None,
        };
        let (_, ends) = results.execution_ends(&stmts, span, Some(b.right_brace));
        if ends.iter().all(|e| e.flow.always) {
            return;
        }
        let mut c = Collect(Vec::new());
        for s in &stmts {
            c.walk_statement(s, &mut ());
        }
        self.spans.extend(c.0);
    }
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Wanted<'_> {
    fn walk_in_property(&mut self, p: &'ast Property<'arena>, _: &mut ()) {
        if self.level >= 3 {
            property_default::typed(p, &mut self.spans);
        }
    }
    fn walk_in_expression(&mut self, e: &'ast Expression<'arena>, _: &mut ()) {
        values::typed(e, self.level, &mut self.spans);
        dynamic_names::typed(e, &mut self.spans);
        inc_dec::typed(e, &mut self.spans);
        if self.level >= 2 {
            properties::typed(e, &mut self.spans);
            binary_op::typed(e, &mut self.spans);
            // InstantiationRule's `new.nonObject` (checkThisOnly below level 2)
            if let Expression::Instantiation(new) = e
                && !matches!(
                    new.class,
                    Expression::Identifier(_) | Expression::Static(_) | Expression::Self_(_) | Expression::Parent(_)
                )
            {
                self.spans.push(flow::unparen(new.class).span());
            }
        }
        if self.level >= 3 {
            offsets::typed(e, &mut self.spans);
        }
        if self.level >= 4 {
            nullsafe::typed(e, &mut self.spans);
        }
    }
    fn walk_in_class_like_constant(&mut self, k: &'ast ClassLikeConstant<'arena>, _: &mut ()) {
        if self.level >= 2 {
            constant_value::typed(k, &mut self.spans);
        }
    }
    fn walk_in_constant(&mut self, k: &'ast Constant<'arena>, _: &mut ()) {
        if self.level >= 2 {
            constant_value::typed_global(k, &mut self.spans);
        }
    }
    fn walk_in_foreach(&mut self, f: &'ast Foreach<'arena>, _: &mut ()) {
        if self.level >= 3 {
            self.spans.push(f.expression.span());
        }
    }
    fn walk_in_pipe(&mut self, p: &'ast Pipe<'arena>, _: &mut ()) {
        callables::typed_pipe(p, &mut self.spans);
    }
    fn walk_in_function_call(&mut self, call: &'ast FunctionCall<'arena>, _: &mut ()) {
        regex::typed(call, &mut self.spans);
        // Reference argument scopes need the actual dynamic callee even below CallCallablesRule's level.
        callables::typed(call, &mut self.spans);
    }
    fn walk_in_method_call(&mut self, call: &'ast MethodCall<'arena>, _: &mut ()) {
        if !self.check_this_only || super::bind_scope::has_inline_closure(&call.argument_list) {
            receivers::typed(call.object, &mut self.spans);
        }
    }
    fn walk_in_null_safe_method_call(&mut self, call: &'ast NullSafeMethodCall<'arena>, _: &mut ()) {
        if !self.check_this_only || super::bind_scope::has_inline_closure(&call.argument_list) {
            receivers::typed(call.object, &mut self.spans);
        }
    }
    fn walk_in_property_access(&mut self, p: &'ast PropertyAccess<'arena>, _: &mut ()) {
        if !self.check_this_only {
            receivers::typed(p.object, &mut self.spans);
        }
    }
    fn walk_in_null_safe_property_access(&mut self, p: &'ast NullSafePropertyAccess<'arena>, _: &mut ()) {
        if !self.check_this_only || super::Checker::is_this_expression(p.object) {
            self.spans.push(p.object.span());
        }
    }
    fn walk_in_static_property_access(&mut self, p: &'ast StaticPropertyAccess<'arena>, _: &mut ()) {
        if !self.check_this_only || super::Checker::is_this_expression(p.class) {
            access::typed_class(p.class, &mut self.spans);
        }
    }
    fn walk_in_static_method_call(&mut self, call: &'ast StaticMethodCall<'arena>, _: &mut ()) {
        if !self.check_this_only || super::bind_scope::has_inline_closure(&call.argument_list) {
            receivers::typed_class(call.class, &mut self.spans);
        }
    }
    fn walk_in_argument_list(&mut self, l: &'ast ArgumentList<'arena>, _: &mut ()) {
        for a in l.arguments.iter() {
            // Overload selection reads operands even when argument diagnostics are disabled.
            self.spans.push(a.value().span());
            if self.check_arguments {
                match a.value() {
                    Expression::Array(arr) => self.spans.extend(arr.elements.iter().map(super::params::element)),
                    Expression::LegacyArray(arr) => self.spans.extend(arr.elements.iter().map(super::params::element)),
                    _ => {}
                }
            }
        }
    }
    fn walk_in_partial_argument_list(&mut self, l: &'ast PartialArgumentList<'arena>, _: &mut ()) {
        if self.check_arguments {
            for a in l.arguments.iter() {
                match a {
                    PartialArgument::Positional(p) => self.spans.push(p.value.span()),
                    PartialArgument::Named(n) => self.spans.push(n.value.span()),
                    _ => {}
                }
            }
        }
    }
    fn walk_in_echo(&mut self, e: &'ast Echo<'arena>, _: &mut ()) {
        if self.level >= 2 {
            self.spans.extend(e.values.iter().map(|v| v.span()));
        }
    }
    fn walk_in_print_construct(&mut self, p: &'ast PrintConstruct<'arena>, _: &mut ()) {
        if self.level >= 2 {
            self.spans.push(p.value.span());
        }
    }
    fn walk_in_interpolated_string(&mut self, s: &'ast InterpolatedString<'arena>, _: &mut ()) {
        if self.level >= 2 {
            self.spans.extend(s.parts.iter().filter_map(casts::part).map(|e| e.span()));
        }
    }
    fn walk_in_document_string(&mut self, s: &'ast DocumentString<'arena>, _: &mut ()) {
        if self.level >= 2 {
            self.spans.extend(s.parts.iter().filter_map(casts::part).map(|e| e.span()));
        }
    }
    fn walk_in_function_like_parameter_list(&mut self, l: &'ast FunctionLikeParameterList<'arena>, _: &mut ()) {
        if self.level >= 2 {
            param_default::typed(l, &mut self.spans);
        }
    }
    fn walk_in_arrow_function(&mut self, f: &'ast ArrowFunction<'arena>, _: &mut ()) {
        if self.level >= 3 && f.return_type_hint.is_some() {
            self.spans.push(f.expression.span());
            returns::typed_arrow(f.expression, &mut self.spans);
        }
    }
    fn walk_in_match(&mut self, m: &'ast Match<'arena>, _: &mut ()) {
        if self.level >= 2 {
            void_match::typed(m, &mut self.spans);
        }
    }
    fn walk_in_binary(&mut self, b: &'ast Binary<'arena>, _: &mut ()) {
        if self.level >= 2 && comparison::operator(&b.operator).is_some() {
            self.spans.extend([b.lhs.span(), b.rhs.span()]);
        }
    }
    fn walk_in_unary_prefix(&mut self, u: &'ast UnaryPrefix<'arena>, _: &mut ()) {
        if self.level >= 2 && casts::operation(&u.operator).is_some() {
            self.spans.push(u.operand.span());
        }
    }
    fn walk_in_function(&mut self, f: &'ast Function<'arena>, _: &mut ()) {
        if self.level >= 3 {
            returns::typed(&f.body, &mut self.spans);
        }
        self.function_like(f.span(), f.return_type_hint.as_ref(), &f.body);
    }
    fn walk_in_method(&mut self, m: &'ast Method<'arena>, _: &mut ()) {
        if let MethodBody::Concrete(b) = &m.body {
            if self.level >= 3 {
                returns::typed(b, &mut self.spans);
            }
            self.function_like(m.span(), m.return_type_hint.as_ref(), b);
        }
    }
    fn walk_in_closure(&mut self, f: &'ast Closure<'arena>, _: &mut ()) {
        if self.level >= 3 && f.return_type_hint.is_some() {
            returns::typed(&f.body, &mut self.spans);
        }
        if f.return_type_hint.is_some() {
            self.function_like(f.span(), f.return_type_hint.as_ref(), &f.body);
        }
    }
    fn walk_in_property_hook(&mut self, h: &'ast PropertyHook<'arena>, _: &mut ()) {
        if self.level >= 3 {
            match &h.body {
                PropertyHookBody::Concrete(PropertyHookConcreteBody::Block(b)) => returns::typed(b, &mut self.spans),
                PropertyHookBody::Concrete(PropertyHookConcreteBody::Expression(e)) => {
                    self.spans.push(e.expression.span());
                }
                PropertyHookBody::Abstract(_) => {}
            }
        }
        if let PropertyHookBody::Concrete(PropertyHookConcreteBody::Block(b)) = &h.body
            && h.name.value.eq_ignore_ascii_case(b"get")
        {
            let stmts = flow::list(b.statements.as_slice());
            let empty = HashMap::new();
            let results = flow::Results {
                types: &empty,
                call_terminations: None,
                contents: self.contents,
                codebase: None,
            };
            let (_, ends) = results.execution_ends(&stmts, h.span(), Some(b.right_brace));
            if ends.iter().all(|e| e.flow.always) {
                return;
            }
            let mut c = Collect(Vec::new());
            for s in &stmts {
                c.walk_statement(s, &mut ());
            }
            self.spans.extend(c.0);
        }
    }
}

/// Calls and matches (a `never` type ends the execution), conditions and iterated expressions.
struct Collect(Vec<Span>);

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Collect {
    fn walk_in_call(&mut self, c: &'ast Call<'arena>, _: &mut ()) {
        self.0.push(c.span());
    }
    fn walk_in_match(&mut self, m: &'ast Match<'arena>, _: &mut ()) {
        self.0.push(m.span());
    }
    fn walk_in_if(&mut self, i: &'ast If<'arena>, _: &mut ()) {
        self.0.push(i.condition.span());
        match &i.body {
            IfBody::Statement(b) => self.0.extend(b.else_if_clauses.iter().map(|e| e.condition.span())),
            IfBody::ColonDelimited(b) => self.0.extend(b.else_if_clauses.iter().map(|e| e.condition.span())),
        }
    }
    fn walk_in_while(&mut self, w: &'ast While<'arena>, _: &mut ()) {
        self.0.push(w.condition.span());
    }
    fn walk_in_do_while(&mut self, d: &'ast DoWhile<'arena>, _: &mut ()) {
        self.0.push(d.condition.span());
    }
    fn walk_in_for(&mut self, f: &'ast For<'arena>, _: &mut ()) {
        self.0.extend(f.conditions.iter().last().map(|c| c.span()));
    }
    fn walk_in_foreach(&mut self, f: &'ast Foreach<'arena>, _: &mut ()) {
        self.0.push(f.expression.span());
    }
    fn walk_in_switch(&mut self, s: &'ast Switch<'arena>, _: &mut ()) {
        self.0.push(s.expression.span());
    }
    fn walk_in_switch_expression_case(&mut self, c: &'ast SwitchExpressionCase<'arena>, _: &mut ()) {
        self.0.push(c.expression.span());
    }
}
