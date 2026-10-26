//! Declaration-owned effects, separate from NoopExpressionNode's explicit-throw/call-result model.
//!
//! `None` means a producer is not ported, not an empty list. This deliberately bounded projection of
//! PHPStan's expression/statement results answers only list emptiness. In particular a throw point is
//! not a union type: an implicit or residual Never-typed point must not disappear through type reduction.

use std::collections::HashSet;

use mago_span::HasSpan;
use mago_syntax::cst::*;

use super::{Checker, declarations, declared, doc};

/// CallToFunctionStatementWithoutSideEffectsRule::PHPSTAN_TESTING_FUNCTIONS.
pub(super) const TESTING_FUNCTIONS: &[&str] = &[
    "PHPStan\\dumpNativeType",
    "PHPStan\\dumpType",
    "PHPStan\\dumpPhpDocType",
    "PHPStan\\debugScope",
    "PHPStan\\Testing\\assertType",
    "PHPStan\\Testing\\assertNativeType",
    "PHPStan\\Testing\\assertSuperType",
    "PHPStan\\Testing\\assertVariableCertainty",
];

/// ResolvedPhpDocBlock::isPure(): absence is unspecified; pure wins over impure.
pub(super) fn purity(raw: &[u8]) -> Option<bool> {
    let tags = declarations::doc_tags(raw);
    if declarations::has_tag(
        &tags,
        &[
            "@pure",
            "@phan-pure",
            "@phan-side-effect-free",
            "@psalm-pure",
            "@phpstan-pure",
        ],
    ) {
        Some(true)
    } else if declarations::has_tag(&tags, &["@impure", "@phpstan-impure"]) {
        Some(false)
    } else {
        None
    }
}

/// Conditional purity needs argument/parameter reflection, which this producer does not yet resolve.
pub(super) fn conditional(raw: &[u8]) -> bool {
    declarations::doc_tags(raw).iter().any(|tag| {
        matches!(
            tag.as_str(),
            "@pure-unless-callable-is-impure"
                | "@phpstan-pure-unless-callable-is-impure"
                | "@pure-unless-parameter-passed"
                | "@phpstan-pure-unless-parameter-passed"
        )
    })
}

#[derive(Default)]
pub(super) struct Effects {
    impure: bool,
    throws: bool,
    terminates: bool,
}

impl Effects {
    fn merge(&mut self, other: Self) {
        self.impure |= other.impure;
        self.throws |= other.throws;
        self.terminates |= other.terminates;
    }

    pub(super) fn empty(&self) -> bool {
        !self.impure && !self.throws
    }
}

/// FuncCall names are resolved by the name resolver, including imported names.
pub(super) fn testing(c: &Checker, e: &Expression) -> bool {
    if let Expression::Parenthesized(p) = e {
        return testing(c, p.expression);
    }
    let Expression::Call(Call::Function(call)) = e else {
        return false;
    };
    let Expression::Identifier(id) = call.function else {
        return false;
    };
    TESTING_FUNCTIONS
        .iter()
        .any(|name| name.as_bytes() == c.names.get(&id.span()))
}

pub(super) fn body(c: &Checker, b: &Block) -> Option<Effects> {
    Producer {
        c,
        arrays: HashSet::new(),
    }
    .block(b)
}

struct Producer<'c, 'code, 'names> {
    c: &'c Checker<'code, 'names>,
    /// Only proven literal-array containers. An unknown local may instead invoke ArrayAccess methods.
    arrays: HashSet<Vec<u8>>,
}

impl Producer<'_, '_, '_> {
    fn block(&mut self, b: &Block) -> Option<Effects> {
        self.statements(b.statements.as_slice())
    }

    fn statements(&mut self, statements: &[Statement]) -> Option<Effects> {
        let mut effects = Effects::default();
        for statement in statements {
            // StatementsHandler still visits declarations after an unconditional exit, not ordinary code.
            if effects.terminates && !matches!(statement, Statement::Function(_)) {
                continue;
            }
            effects.merge(self.statement(statement)?);
        }
        Some(effects)
    }

    fn statement(&mut self, s: &Statement) -> Option<Effects> {
        match s {
            Statement::Noop(_) | Statement::Function(_) => Some(Effects::default()),
            Statement::Block(b) => self.block(b),
            Statement::Expression(s) => self.expression(s.expression),
            Statement::Return(r) => {
                let mut effects = match r.value {
                    Some(value) => self.expression(value)?,
                    None => Effects::default(),
                };
                effects.terminates = true;
                Some(effects)
            }
            Statement::Break(b) => {
                let mut effects = match b.level {
                    Some(level) => self.expression(level)?,
                    None => Effects::default(),
                };
                effects.terminates = true;
                Some(effects)
            }
            Statement::Switch(s) => {
                // SwitchHandler merges subject/case effects and each branch result. Without narrowing, this
                // overapproximates reachable branches; only a known-empty result is consumed here.
                let mut effects = self.expression(s.expression)?;
                for case in s.body.cases() {
                    self.arrays.clear();
                    if let Some(value) = case.expression() {
                        effects.merge(self.expression(value)?);
                    }
                    effects.merge(self.statements(case.statements())?);
                }
                effects.terminates = false; // No all-branches exit proof yet; break only exits the switch.
                self.arrays.clear();
                Some(effects)
            }
            Statement::Try(t) => {
                let mut effects = self.block(&t.block)?;
                self.arrays.clear();
                if effects.throws {
                    for catch in t.catch_clauses.iter() {
                        // A narrower catch needs typed residual points (including PHPStan's fallback point).
                        // Never substitute a reduced throw-type union for that producer.
                        let Hint::Identifier(id) = &catch.hint else {
                            return None;
                        };
                        if !self
                            .c
                            .names
                            .get(&id.span())
                            .eq_ignore_ascii_case(b"Throwable")
                        {
                            return None;
                        }
                        effects.throws = false;
                        let caught = self.block(&catch.block)?;
                        effects.terminates &= caught.terminates;
                        effects.impure |= caught.impure;
                        effects.throws |= caught.throws;
                        break;
                    }
                }
                if let Some(finally) = &t.finally_clause {
                    effects.merge(self.block(&finally.block)?);
                }
                self.arrays.clear();
                Some(effects)
            }
            Statement::Echo(e) if e.values.iter().all(|e| matches!(e, Expression::Literal(_))) => {
                Some(Effects {
                    impure: true,
                    ..Effects::default()
                })
            }
            // Branch/loop scope joins, hooks, globals, static variables, etc. need their own producers.
            _ => None,
        }
    }

    fn expression(&mut self, e: &Expression) -> Option<Effects> {
        match e {
            Expression::Literal(_)
            | Expression::MagicConstant(_)
            | Expression::Closure(_)
            | Expression::ArrowFunction(_) => Some(Effects::default()),
            Expression::Parenthesized(p) => self.expression(p.expression),
            // ClassConstFetchHandler has no call/throw point for a literal class and constant name.
            Expression::Access(Access::ClassConstant(a))
                if matches!(a.class, Expression::Identifier(_))
                    && matches!(a.constant, ClassLikeConstantSelector::Identifier(_)) =>
            {
                Some(Effects::default())
            }
            Expression::Construct(Construct::Print(p))
                if matches!(p.value, Expression::Literal(_)) =>
            {
                Some(Effects {
                    impure: true,
                    ..Effects::default()
                })
            }
            Expression::Variable(Variable::Direct(v)) => Some(Effects {
                impure: matches!(
                    v.name,
                    b"$GLOBALS"
                        | b"$_SERVER"
                        | b"$_GET"
                        | b"$_POST"
                        | b"$_FILES"
                        | b"$_COOKIE"
                        | b"$_SESSION"
                        | b"$_REQUEST"
                        | b"$_ENV"
                ),
                ..Effects::default()
            }),
            Expression::Array(a) => self.elements(a.elements.iter(), false, false),
            Expression::LegacyArray(a) => self.elements(a.elements.iter(), false, false),
            Expression::Assignment(a) => {
                // Concat may call __toString(); division/modulo have additional throw producers.
                if !matches!(
                    a.operator,
                    AssignmentOperator::Assign(_) | AssignmentOperator::Addition(_)
                ) {
                    return None;
                }
                let mut effects = self.expression(a.rhs)?;
                let array = self.array(a.rhs);
                effects.merge(self.target(a.lhs, true)?);
                if let Expression::Variable(Variable::Direct(v)) = a.lhs {
                    if a.operator.is_assign() && array {
                        self.arrays.insert(v.name.to_vec());
                    } else {
                        self.arrays.remove(v.name);
                    }
                }
                Some(effects)
            }
            Expression::UnaryPrefix(u) if u.operator.is_reference() => {
                let effects = self.expression(u.operand)?;
                self.arrays.clear(); // An alias can subsequently replace the entire container.
                Some(effects)
            }
            Expression::UnaryPrefix(u)
                if matches!(
                    u.operator,
                    UnaryPrefixOperator::Plus(_) | UnaryPrefixOperator::Negation(_)
                ) && matches!(
                    u.operand,
                    Expression::Literal(Literal::Integer(_) | Literal::Float(_))
                ) =>
            {
                self.expression(u.operand)
            }
            Expression::UnaryPrefix(u)
                if matches!(
                    u.operator,
                    UnaryPrefixOperator::PreIncrement(_) | UnaryPrefixOperator::PreDecrement(_)
                ) =>
            {
                self.target(u.operand, false)
            }
            Expression::UnaryPostfix(u) => self.target(u.operand, false),
            Expression::ArrayAccess(a) if self.array(a.array) => {
                let mut effects = self.expression(a.array)?;
                effects.merge(self.expression(a.index)?);
                Some(effects)
            }
            Expression::Throw(t) => {
                let mut effects = self.expression(t.exception)?;
                effects.throws = true;
                effects.terminates = true;
                Some(effects)
            }
            Expression::Call(Call::Function(call)) => self.call(call),
            _ => None,
        }
    }

    fn array(&self, e: &Expression) -> bool {
        match e {
            Expression::Array(_) | Expression::LegacyArray(_) => true,
            Expression::Variable(Variable::Direct(v)) => self.arrays.contains(v.name),
            _ => false,
        }
    }

    /// AssignHandler::prepareTarget()/processArrayDestructuring(). A root [] target is fixed up recursively;
    /// explicit list() does NOT fix up literal arrays in its items. Literal-array children are reads.
    fn target(&mut self, e: &Expression, fix_arrays: bool) -> Option<Effects> {
        match e {
            Expression::Variable(Variable::Direct(v)) => {
                self.arrays.remove(v.name);
                self.expression(e)
            }
            Expression::List(l) => self.elements(l.elements.iter(), true, false),
            Expression::Array(a) if fix_arrays => self.elements(a.elements.iter(), true, true),
            Expression::LegacyArray(a) if fix_arrays => {
                self.elements(a.elements.iter(), true, true)
            }
            Expression::UnaryPrefix(u) if u.operator.is_reference() => {
                let effects = self.target(u.operand, fix_arrays)?;
                self.arrays.clear();
                Some(effects)
            }
            Expression::ArrayAccess(a)
                if self.array(a.array) && matches!(a.index, Expression::Literal(_)) =>
            {
                Some(Effects::default())
            }
            Expression::ArrayAppend(a) if self.array(a.array) => Some(Effects::default()),
            // Property writes, ArrayAccess, property hooks and dynamic names remain unknown here.
            Expression::Access(_) | Expression::ArrayAccess(_) | Expression::ArrayAppend(_) => None,
            e => self.expression(e),
        }
    }

    fn elements<'a, 'arena: 'a>(
        &mut self,
        items: impl Iterator<Item = &'a ArrayElement<'arena>>,
        targets: bool,
        fix_arrays: bool,
    ) -> Option<Effects> {
        let mut effects = Effects::default();
        for item in items {
            let value = match item {
                ArrayElement::KeyValue(kv) => {
                    effects.merge(self.expression(kv.key)?);
                    kv.value
                }
                ArrayElement::Value(v) => v.value,
                ArrayElement::Missing(_) => continue,
                ArrayElement::Variadic(_) => return None,
            };
            effects.merge(if targets {
                self.target(value, fix_arrays)?
            } else {
                self.expression(value)?
            });
        }
        Some(effects)
    }

    fn call(&mut self, call: &FunctionCall) -> Option<Effects> {
        let Expression::Identifier(id) = call.function else {
            return None;
        };
        let mut effects = Effects::default();
        for arg in call.argument_list.arguments.iter() {
            if matches!(arg, Argument::Positional(p) if p.ellipsis.is_some()) {
                return None;
            }
            // Callable invocation policies need their own producer, even for an explicitly pure callee.
            if matches!(
                arg.value(),
                Expression::Closure(_) | Expression::ArrowFunction(_)
            ) {
                return None;
            }
            effects.merge(self.expression(arg.value())?);
        }
        let name = self.c.names.get(&id.span());
        if TESTING_FUNCTIONS.iter().any(|n| n.as_bytes() == name) {
            return Some(effects);
        }
        let fallback = match id {
            Identifier::Local(l) if !self.c.names.is_imported(&l.span) => Some(l.value),
            _ => None,
        };
        let f = std::iter::once(name)
            .chain(fallback)
            .find_map(|name| self.c.codebase.get_function(name))?;
        if f.flags.is_built_in() {
            return None;
        }
        // Source availability, not an absent PURE bit, establishes an unannotated user declaration.
        self.c.file_contents(f.span.file_id)?;
        let raw = declared::doc_at(self.c, f.span)
            .map(|(raw, _)| raw)
            .unwrap_or_default();
        // Parameterized calls additionally need reference writes and immediately-invoked callable policies,
        // including a callable passed through a local variable rather than a closure literal.
        if conditional(&raw) || !f.parameters.is_empty() {
            return None;
        }
        let ret = &f.return_type_metadata.as_ref()?.type_union;
        if ret.is_never() {
            return None;
        }
        effects.impure |= ret.is_void() || purity(&raw) != Some(true);
        // PhpDocNodeResolver::resolveThrowsTags(): phpstan-throws takes precedence, all tags of that name merge.
        let tags = doc::tags(&raw);
        let kind = if tags.iter().any(|(name, _)| name == "@phpstan-throws") {
            "@phpstan-throws"
        } else {
            "@throws"
        };
        let throws: Vec<_> = tags.iter().filter(|(name, _)| name == kind).collect();
        if throws.is_empty() {
            effects.throws = true; // FuncCallHandler's default implicit throw point, even on a pure call.
        } else if !throws
            .iter()
            .all(|(_, value)| value.split_whitespace().next() == Some("void"))
        {
            return None;
        }
        self.arrays.clear();
        Some(effects)
    }
}
