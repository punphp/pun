//! Level 3's return statements: ReturnTypeRule (functions, methods), ClosureReturnTypeRule and
//! ArrowFunctionReturnTypeRule over FunctionReturnTypeCheck (`return.type`, `return.empty`, `return.void`,
//! `return.never`).

use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;
use pun_phpstan::types::{NeverKind, Type, recommended_level};

use super::super::{Checker, declared};
use super::missing_return::{declaring, generator_return, late, method_name};

pub const IDENTIFIERS: &[&str] = &["return.type", "return.empty", "return.void"];

/// A body's own `return` statements (not those of nested functions and classes) and whether it yields.
#[derive(Default)]
pub(super) struct Returns<'ast, 'arena> {
    pub(super) returns: Vec<&'ast Return<'arena>>,
    yields: Vec<&'ast Yield<'arena>>,
    /// The expressions that are statements.
    statements: Vec<Span>,
    pub(super) generator: bool,
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Returns<'ast, 'arena> {
    fn walk_in_statement_expression(&mut self, s: &'ast ExpressionStatement<'arena>, _: &mut ()) {
        self.statements.push(s.expression.span());
    }
    fn walk_in_return(&mut self, r: &'ast Return<'arena>, _: &mut ()) {
        self.returns.push(r);
    }
    fn walk_in_yield(&mut self, y: &'ast Yield<'arena>, _: &mut ()) {
        self.yields.push(y);
        self.generator = true;
    }
    fn walk_closure(&mut self, _: &'ast Closure<'arena>, _: &mut ()) {}
    fn walk_arrow_function(&mut self, _: &'ast ArrowFunction<'arena>, _: &mut ()) {}
    fn walk_function(&mut self, _: &'ast Function<'arena>, _: &mut ()) {}
    fn walk_class(&mut self, _: &'ast Class<'arena>, _: &mut ()) {}
    fn walk_interface(&mut self, _: &'ast Interface<'arena>, _: &mut ()) {}
    fn walk_trait(&mut self, _: &'ast Trait<'arena>, _: &mut ()) {}
    fn walk_enum(&mut self, _: &'ast Enum<'arena>, _: &mut ()) {}
    fn walk_anonymous_class(&mut self, _: &'ast AnonymousClass<'arena>, _: &mut ()) {}
}

pub(super) fn returns<'ast, 'arena>(b: &'ast Block<'arena>) -> Returns<'ast, 'arena> {
    let mut r = Returns::default();
    for s in b.statements.iter() {
        r.walk_statement(s, &mut ());
    }
    r
}

/// The returned values whose types the rules read, of the function-likes with a body.
pub(super) fn typed(b: &Block, spans: &mut Vec<Span>) {
    let r = returns(b);
    spans.extend(r.returns.iter().filter_map(|r| r.value).map(|v| v.span()));
    for y in &r.yields {
        super::yields::typed(y, spans);
    }
}

/// The yields of an arrow function's expression.
fn arrow_yields<'ast, 'arena>(e: &'ast Expression<'arena>) -> Vec<&'ast Yield<'arena>> {
    let mut r = Returns::default();
    r.walk_expression(e, &mut ());
    r.yields
}

/// [`typed()`] of an arrow function.
pub(super) fn typed_arrow(e: &Expression, spans: &mut Vec<Span>) {
    for y in arrow_yields(e) {
        super::yields::typed(y, spans);
    }
}

/// `declare(strict_types=1)` in the file.
// ponytail: found in the file's text, not its first statement
pub(super) fn strict_types(c: &Checker) -> bool {
    let text: Vec<u8> = c.file.contents.iter().filter(|b| !b.is_ascii_whitespace()).map(u8::to_ascii_lowercase).collect();
    text.windows(23).any(|w| w == b"declare(strict_types=1)")
}

/// `FunctionReturnTypeCheck::checkReturnType()` for one return of a function-like described by `of`.
fn check(c: &mut Checker, ret: &Type, value: Option<&Expression>, at: Span, of: &str, generator: bool) {
    if matches!(ret, Type::Never(NeverKind::Explicit)) {
        let message = format!("{of} should never return but return statement found.");
        c.report(at, "return.never", message, None);
        return;
    }
    let mut ret = ret.clone();
    if generator {
        let Some(r) = generator_return(&ret) else {
            return;
        };
        ret = r;
    }
    let void = ret.is_void();
    let Some(value) = value else {
        if !void.no() {
            return;
        }
        let described = declared::with_classes(c, || ret.describe(recommended_level(&ret, None)));
        let message = format!("{of} should return {described} but empty return statement found.");
        c.report(at, "return.empty", message, None);
        return;
    };
    let Some(mut value_type) = c.phpstan_type(value) else {
        return;
    };
    // assistant's call: Mago types `new static`, `new $class` (of `static::class`) and a call returning `static` as
    // the class itself, where PHPStan keeps `static`; a returned object of the class is taken as `static`
    if let (Type::Static(..), Type::Object(name, args, _)) = (&ret, &value_type) {
        value_type = Type::Static(name.clone(), args.clone());
    }
    let strict = strict_types(c);
    let levels = c.levels();
    let message = declared::with_classes(c, || {
        let level = recommended_level(&ret, Some(&value_type));
        if void.yes() {
            return Some((
                "return.void",
                format!(
                    "{of} with return type void returns {} but should not return anything.",
                    value_type.describe(level)
                ),
            ));
        }
        (!levels.accepts(&ret, &value_type, strict)).then(|| {
            (
                "return.type",
                format!("{of} should return {} but returns {}.", ret.describe(level), value_type.describe(level)),
            )
        })
    });
    if let Some((identifier, message)) = message {
        c.report(at, identifier, message, None);
    }
}

fn body(c: &mut Checker, b: &Block, ret: &Type, of: &str) {
    let r = returns(b);
    for ret_stmt in &r.returns {
        check(c, ret, ret_stmt.value, ret_stmt.span(), of, r.generator);
    }
    let strict = strict_types(c);
    super::yields::check(c, &r.yields, &r.statements, ret, strict);
}

pub(in super::super) fn function(c: &mut Checker, f: &Function) {
    if c.level < 3 {
        return;
    }
    let name = String::from_utf8_lossy(c.names.get(&f.name.span())).into_owned();
    let native = f.return_type_hint.is_some();
    let ret = declared::with_classes(c, || late(c, &declared::function(c, f).ret, None, !native));
    body(c, &f.body, &ret, &format!("Function {name}()"));
}

pub(in super::super) fn method(c: &mut Checker, m: &Method) {
    if c.level < 3 {
        return;
    }
    let MethodBody::Concrete(b) = &m.body else {
        return;
    };
    let (Some(context), Some(declaring)) = (c.class_scope(), declaring(c, m.span())) else { return };
    let native = m.return_type_hint.is_some();
    let ret = declared::with_classes(c, || late(c, &declared::method(c, m, declaring, context).ret, Some(context), !native));
    let of = format!("Method {}::{}()", c.display(context), method_name(c, m));
    body(c, b, &ret, &of);
}

/// A closure with a native return type (without one, the type is inferred from its returns).
// ponytail: a closure's PHPDoc `@return` is not read
pub(in super::super) fn closure(c: &mut Checker, f: &Closure) {
    if c.level < 3 {
        return;
    }
    let Some(hint) = &f.return_type_hint else {
        return;
    };
    let class = c.class_scope();
    let ret = declared::with_classes(c, || late(c, &declared::native(c, &hint.hint, class), class, false));
    body(c, &f.body, &ret, "Anonymous function");
}

pub(in super::super) fn arrow_function(c: &mut Checker, f: &ArrowFunction) {
    if c.level < 3 {
        return;
    }
    let Some(hint) = &f.return_type_hint else {
        return;
    };
    let class = c.class_scope();
    let ret = declared::with_classes(c, || late(c, &declared::native(c, &hint.hint, class), class, false));
    if matches!(ret, Type::Never(NeverKind::Explicit))
        && c.type_of(f.expression).is_some_and(|t| t.is_never())
    {
        return;
    }
    let generator = matches!(&ret, Type::Object(n, _, _) if n.eq_ignore_ascii_case("Generator"));
    check(c, &ret, Some(f.expression), f.expression.span(), "Anonymous function", generator);
    let strict = strict_types(c);
    super::yields::check(c, &arrow_yields(f.expression), &[], &ret, strict);
}

/// The hooks of property `name` (with its `$`): a `get` hook returns the property's type (an `=> expr` body is its
/// return), a `set` hook returns void.
pub(in super::super) fn hooks(c: &mut Checker, name: &[u8], hint: Option<&Hint>, at: Span, list: &PropertyHookList) {
    if c.level < 3 {
        return;
    }
    let Some(context) = c.class_scope() else { return };
    let display = c.display(context);
    let property = String::from_utf8_lossy(name).into_owned();
    for h in list.hooks.iter() {
        let PropertyHookBody::Concrete(body) = &h.body else {
            continue;
        };
        let get = h.name.value.eq_ignore_ascii_case(b"get");
        let ret = if get {
            declared::with_classes(c, || {
                late(c, &declared::property(c, hint, at, &name[1..], context), Some(context), hint.is_none())
            })
        } else {
            Type::Void
        };
        let of = format!("{} hook for property {display}::{property}", if get { "Get" } else { "Set" });
        match body {
            PropertyHookConcreteBody::Block(b) => self::body(c, b, &ret, &of),
            PropertyHookConcreteBody::Expression(e) if get => {
                check(c, &ret, Some(e.expression), e.expression.span(), &of, false);
            }
            PropertyHookConcreteBody::Expression(_) => {}
        }
    }
}
