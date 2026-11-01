//! phpstan-strict-rules (rules.neon, each rule on with its `strictRules.*` switch): ClosureUsesThisRule,
//! RequireParentConstructCallRule, DynamicCallOnStaticMethodsRule and DynamicCallOnStaticMethodsCallableRule.
//! UselessCastRule and MatchingTypeInSwitchCaseConditionRule. WrongCaseOfInheritedMethodRule is in overriding.rs,
//! next to the ancestry it reads.

use std::collections::HashSet;

use mago_codex::metadata::CodebaseMetadata;
use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_syntax::walker::MutWalker;

use pun_phpstan::types::{Level, Type, is_super_type_of};

use super::{Checker, declared, members};

/// The expressions whose types the rules read: the variables a non-static closure imports (ClosureUsesThisRule),
/// from level 2 the object of a call to a method name some class declares static, a cast and its operand
/// (UselessCastRule), and a switch's condition and case conditions.
pub fn typed_expressions(project: &super::Checks, codebase: &CodebaseMetadata, program: &Program) -> Vec<Span> {
    let strict = project.strict;
    if !strict.closure_uses_this
        && !(strict.dynamic_call_on_static_method && project.level >= 2)
        && !strict.useless_cast
        && !strict.switch_conditions_matching_type
    {
        return Vec::new();
    }
    let statics: HashSet<Vec<u8>> = if strict.dynamic_call_on_static_method && project.level >= 2 {
        codebase
            .function_likes
            .iter()
            .filter(|(_, f)| f.method_metadata.as_ref().is_some_and(|m| m.is_static))
            .map(|((_, method), _)| method.as_bytes().to_ascii_lowercase())
            .collect()
    } else {
        HashSet::new()
    };
    let mut typed = Typed {
        closures: strict.closure_uses_this,
        statics,
        casts: strict.useless_cast,
        switches: strict.switch_conditions_matching_type,
        spans: Vec::new(),
    };
    typed.walk_program(program, &mut ());
    typed.spans
}

struct Typed {
    closures: bool,
    statics: HashSet<Vec<u8>>,
    casts: bool,
    switches: bool,
    spans: Vec<Span>,
}

impl Typed {
    fn call(&mut self, object: &Expression, method: &ClassLikeMemberSelector) {
        if let ClassLikeMemberSelector::Identifier(id) = method
            && self.statics.contains(&id.value.to_ascii_lowercase())
        {
            self.spans.push(object.span());
        }
    }
}

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for Typed {
    fn walk_in_closure(&mut self, f: &'ast Closure<'arena>, _: &mut ()) {
        if self.closures
            && f.r#static.is_none()
            && let Some(uses) = &f.use_clause
        {
            self.spans.extend(uses.variables.iter().map(|u| u.variable.span()));
        }
    }
    fn walk_in_method_call(&mut self, call: &'ast MethodCall<'arena>, _: &mut ()) {
        self.call(call.object, &call.method);
    }
    fn walk_in_null_safe_method_call(&mut self, call: &'ast NullSafeMethodCall<'arena>, _: &mut ()) {
        self.call(call.object, &call.method);
    }
    fn walk_in_method_partial_application(&mut self, m: &'ast MethodPartialApplication<'arena>, _: &mut ()) {
        self.call(m.object, &m.method);
    }
    fn walk_in_unary_prefix(&mut self, u: &'ast UnaryPrefix<'arena>, _: &mut ()) {
        if self.casts && u.operator.is_cast() {
            self.spans.extend([u.span(), u.operand.span()]);
        }
    }
    fn walk_in_switch(&mut self, s: &'ast Switch<'arena>, _: &mut ()) {
        if self.switches {
            self.spans.push(s.expression.span());
            self.spans.extend(s.body.cases().iter().filter_map(SwitchCase::expression).map(HasSpan::span));
        }
    }
}

const TIP: &str = "Because the type is coming from a PHPDoc, you can turn off this check by setting treatPhpDocTypesAsCertain: false in your %configurationFile%.";

/// UselessCastRule: a cast whose (generalized) type already holds its operand's type.
pub fn cast(c: &mut Checker, u: &UnaryPrefix) {
    if !c.strict.useless_cast
        || !u.operator.is_cast()
        || matches!(u.operator, UnaryPrefixOperator::VoidCast(..) | UnaryPrefixOperator::UnsetCast(..))
    {
        return;
    }
    let Some(cast) = c.phpstan_type_at(u.span()).filter(|t| !matches!(t, Type::Error)) else {
        return;
    };
    let cast = cast.generalize();
    // ponytail: Mago tracks native types only in fully supported functions; otherwise a parameter is its declaration
    // and a literal or constant has no PHPDoc input, and anything else has no native type known (it gets the tip)
    let native = || {
        c.phpstan_type_in(u.operand, &c.native_types).or_else(|| match u.operand {
            Expression::Literal(_) | Expression::Array(_) | Expression::ConstantAccess(_) => c.phpstan_type(u.operand),
            Expression::Variable(_) => super::dead_code::sort::native(c, u.operand),
            _ => None,
        })
    };
    let operand = if c.treat_php_doc_types_as_certain { c.phpstan_type(u.operand) } else { native() };
    let Some(operand) = operand else {
        return;
    };
    let holds = |t: &Type| declared::with_classes(c, || is_super_type_of(&cast, t).yes());
    if !holds(&operand) {
        return;
    }
    let tip = (c.treat_php_doc_types_as_certain && !native().is_some_and(|n| holds(&n))).then_some(TIP);
    let message = declared::with_classes(c, || {
        format!(
            "Casting to {} something that's already {}.",
            cast.describe(Level::TypeOnly),
            operand.describe(Level::TypeOnly)
        )
    });
    c.report(u, "cast.useless", message, tip);
}

/// MatchingTypeInSwitchCaseConditionRule: a case whose type the switch condition's type can never be.
pub fn switch(c: &mut Checker, s: &Switch) {
    if !c.strict.switch_conditions_matching_type {
        return;
    }
    let Some(condition) = c.phpstan_type(s.expression) else {
        return;
    };
    for case in s.body.cases() {
        let SwitchCase::Expression(case) = case else {
            continue;
        };
        let Some(t) = c.phpstan_type(case.expression) else {
            continue;
        };
        if !declared::with_classes(c, || is_super_type_of(&condition, &t).no()) {
            continue;
        }
        let message = declared::with_classes(c, || {
            format!(
                "Switch condition type ({}) does not match case condition {} ({}).",
                condition.describe(Level::Value),
                super::arrays::written(c, case.expression),
                t.describe(Level::TypeOnly)
            )
        });
        c.report(case, "switch.type", message, None);
    }
}

/// ClosureUsesThisRule: a variable holding `$this` imported into a non-static closure. A closure inside the
/// arguments of `Closure::bind()` is skipped (`isInClosureBind()`).
pub fn closure(c: &mut Checker, f: &Closure) {
    if !c.strict.closure_uses_this || f.r#static.is_some() || c.closure_bind > 0 {
        return;
    }
    let Some(uses) = &f.use_clause else {
        return;
    };
    for u in uses.variables.iter() {
        let is_this = c.types.get(&u.variable.span()).is_some_and(|t| {
            matches!(t.types.as_ref(), [TAtomic::Object(TObject::Named(o))] if o.is_this)
        });
        if is_this {
            let name = String::from_utf8_lossy(&u.variable.name[1..]);
            let message = format!(
                "Anonymous function uses $this assigned to variable ${name}. Use $this directly in the function body."
            );
            c.report(u, "closure.useThis", message, None);
        }
    }
}

/// RequireParentConstructCallRule on method `m` of the class being walked.
pub fn constructor(c: &mut Checker, m: &Method<'_>) {
    if !c.strict.require_parent_constructor_call || c.in_trait().is_some() || m.name.value != b"__construct" {
        return;
    }
    let MethodBody::Concrete(body) = &m.body else {
        return;
    };
    let k = c.class_scope().expect("a method is walked inside its class-like");
    if k.kind.is_interface() || calls_parent_construct(body.statements.as_slice()) {
        return;
    }
    let mut parent = k.direct_parent_class.and_then(|p| c.codebase.get_class_like(p.as_bytes()));
    // a circular hierarchy (which PHP refuses) ends
    let mut depth = 0;
    while let Some(p) = parent
        && depth <= 64
    {
        depth += 1;
        let own = |f: &members::Method| {
            f.declaring_name().eq_ignore_ascii_case(&String::from_utf8_lossy(p.name.as_bytes()))
        };
        let constructor = members::method(c, p, b"__construct")
            .filter(|f| own(f) && !f.is_abstract() && !f.visibility().is_private() && !is_deprecated(f));
        // a PHP 4 constructor, looked up by the class's full name (so only in the global namespace)
        let named = members::method(c, p, p.original_name.as_bytes()).filter(|f| own(f) && !f.is_abstract());
        if constructor.is_some() || named.is_some() {
            let message = format!(
                "{}::__construct() does not call parent constructor from {}.",
                c.class_name(k),
                c.class_name(p)
            );
            c.report(m, "constructor.missingParentCall", message, None);
            return;
        }
        parent = p.direct_parent_class.and_then(|n| c.codebase.get_class_like(n.as_bytes()));
    }
}

fn is_deprecated(m: &members::Method) -> bool {
    match m {
        members::Method::User { f, .. } => f.flags.is_deprecated(),
        members::Method::Builtin { f, .. } => f.deprecated,
    }
}

/// `callsParentConstruct()`: a `parent::__construct()` statement (`@` ignored), here or in the statements of a nested
/// node that has a statement list (php-parser's `stmts`: an `if` body, loops, `try`, blocks, declarations).
fn calls_parent_construct(statements: &[Statement]) -> bool {
    statements.iter().any(calls_in_statement)
}

fn calls_in_statement(s: &Statement) -> bool {
    let one = |s: &Statement| calls_in_statement(s);
    match s {
        Statement::Expression(e) => calls_in_expression(e.expression),
        Statement::Block(b) => calls_parent_construct(b.statements.as_slice()),
        Statement::If(i) => match &i.body {
            IfBody::Statement(b) => one(b.statement),
            IfBody::ColonDelimited(b) => calls_parent_construct(b.statements.as_slice()),
        },
        Statement::While(w) => match &w.body {
            WhileBody::Statement(s) => one(s),
            WhileBody::ColonDelimited(b) => calls_parent_construct(b.statements.as_slice()),
        },
        Statement::DoWhile(d) => one(d.statement),
        Statement::For(f) => match &f.body {
            ForBody::Statement(s) => one(s),
            ForBody::ColonDelimited(b) => calls_parent_construct(b.statements.as_slice()),
        },
        Statement::Foreach(f) => match &f.body {
            ForeachBody::Statement(s) => one(s),
            ForeachBody::ColonDelimited(b) => calls_parent_construct(b.statements.as_slice()),
        },
        Statement::Try(t) => calls_parent_construct(t.block.statements.as_slice()),
        Statement::Declare(d) => match &d.body {
            DeclareBody::Statement(s) => one(s),
            DeclareBody::ColonDelimited(b) => calls_parent_construct(b.statements.as_slice()),
        },
        Statement::Function(f) => calls_parent_construct(f.body.statements.as_slice()),
        _ => false,
    }
}

fn calls_in_expression(mut e: &Expression) -> bool {
    if let Expression::UnaryPrefix(u) = e
        && matches!(u.operator, UnaryPrefixOperator::ErrorControl(_))
    {
        e = u.operand;
    }
    match e {
        Expression::Call(Call::StaticMethod(call)) => {
            matches!(call.class, Expression::Parent(p) if p.value == b"parent")
                && matches!(&call.method, ClassLikeMemberSelector::Identifier(id) if id.value == b"__construct")
        }
        Expression::Closure(f) => calls_parent_construct(f.body.statements.as_slice()),
        _ => false,
    }
}

/// DynamicCallOnStaticMethodsRule (`$expr->name()`, `$expr?->name()`) and DynamicCallOnStaticMethodsCallableRule
/// (`$expr->name(...)`). RuleLevelHelper::findTypeToCheck() checks only `$this` below level 2; from there the
/// object's type drops `null` (below level 8) and the union members without the method (below level 7).
pub fn dynamic_call(c: &mut Checker, span: Span, object: &Expression, method: &ClassLikeMemberSelector) {
    if !c.strict.dynamic_call_on_static_method {
        return;
    }
    let ClassLikeMemberSelector::Identifier(id) = method else {
        return;
    };
    let is_this = matches!(object, Expression::Variable(Variable::Direct(v)) if v.name == b"$this");
    let classes: Vec<Option<&ClassLikeMetadata>> = if is_this {
        if !c.this_defined() {
            return;
        }
        vec![c.class_scope()]
    } else if c.level >= 2 {
        // the object of every method call is typed from level 2
        c.type_of(object)
            .into_iter()
            .flat_map(|t| t.types.iter())
            .filter(|a| c.level >= 8 || !matches!(a, TAtomic::Null))
            .map(|a| match a {
                TAtomic::Object(TObject::Named(o)) => c.codebase.get_class_like(o.name.as_bytes()),
                TAtomic::Object(TObject::Enum(o)) => c.codebase.get_class_like(o.name.as_bytes()),
                _ => None,
            })
            .collect()
    } else {
        return;
    };
    let found: Vec<Option<members::Method>> = classes
        .iter()
        .map(|k| k.and_then(|k| members::method(c, k, id.value)))
        .collect();
    let with: Vec<&members::Method> = found.iter().flatten().collect();
    let Some(m) = with.first() else {
        return;
    };
    if (c.level < 7 || with.len() == found.len()) && m.is_static() {
        let message = format!("Dynamic call to static method {}::{}().", m.declaring_display(c), m.name());
        c.report(span, "staticMethod.dynamicCall", message, None);
    }
}
