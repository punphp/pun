//! Declaration-owned effects, separate from NoopExpressionNode's explicit-throw/call-result model.
//!
//! `None` means a producer is not ported, not an empty list. This deliberately bounded projection of
//! PHPStan's expression/statement results answers only list emptiness. In particular a throw point is
//! not a union type: an implicit or residual Never-typed point must not disappear through type reduction.

use std::collections::HashSet;

use mago_span::HasSpan;
use mago_syntax::cst::*;
use pun_phpstan::types::Tri;

use super::{Checker, declarations, declared, doc, members, reflection::Lookup, types::flow};

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
        referenced: false,
    }
    .block(b)
}

struct Producer<'c, 'code, 'names> {
    c: &'c Checker<'code, 'names>,
    /// Proven plain-array containers. An unknown local may instead invoke ArrayAccess methods.
    arrays: HashSet<Vec<u8>>,
    /// Offset unsets after aliases require a reference-aware scope transfer, not observed array types.
    referenced: bool,
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
            Statement::Expression(s) => {
                let mut effects = self.expression(s.expression)?;
                // ExpressionHandler promotes an explicit-never *statement* to an exit. NewHandler
                // itself only forwards argument termination: a nested never constructor is different.
                effects.terminates |= self
                    .c
                    .type_of(s.expression)
                    .is_some_and(|t| t.is_explicit_never());
                Some(effects)
            }
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
            Statement::Unset(s) => {
                let mut effects = Effects::default();
                for value in s.values.iter() {
                    effects.merge(self.unset(value)?);
                }
                // UnsetHandler forwards operand points but its statement is never always-terminating.
                effects.terminates = false;
                Some(effects)
            }
            Statement::If(i) => self.if_statement(i),
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
            // Loop scope joins, hooks, globals, static variables, etc. need their own producers.
            _ => None,
        }
    }

    /// IfHandler merges every condition's points, but only reachable bodies' points and termination.
    fn if_statement(&mut self, i: &If) -> Option<Effects> {
        let mut effects = Effects {
            terminates: true,
            ..Effects::default()
        };
        let mut continuing = None;
        let mut decided = false;
        for (condition, statements) in
            std::iter::once((i.condition, i.body.statements())).chain(i.body.else_if_clauses())
        {
            let result = self.expression(condition)?;
            effects.impure |= result.impure;
            effects.throws |= result.throws;
            let truth = self.condition(condition);
            if !decided && !truth.no() {
                self.branch(statements, &mut effects, &mut continuing)?;
            }
            decided |= truth.yes();
        }
        if !decided {
            // Without an else, the false path continues with the last condition's scope.
            self.branch(
                i.body.else_statements().unwrap_or_default(),
                &mut effects,
                &mut continuing,
            )?;
        }
        self.arrays = continuing.unwrap_or_default();
        Some(effects)
    }

    fn branch(
        &mut self,
        statements: &[Statement],
        effects: &mut Effects,
        continuing: &mut Option<HashSet<Vec<u8>>>,
    ) -> Option<()> {
        let mut branch = Self {
            c: self.c,
            arrays: self.arrays.clone(),
            referenced: self.referenced,
        };
        let result = branch.statements(statements)?;
        self.referenced |= branch.referenced;
        effects.impure |= result.impure;
        effects.throws |= result.throws;
        effects.terminates &= result.terminates;
        if !result.terminates {
            // Only facts shared by every continuing branch survive; a terminating branch adds none.
            if let Some(arrays) = continuing {
                arrays.retain(|name| branch.arrays.contains(name));
            } else {
                *continuing = Some(branch.arrays);
            }
        }
        Some(())
    }

    fn condition(&self, e: &Expression) -> Tri {
        // These results do not depend on level-specific type requests. Unsupported spreads already
        // return None from expression(), before literal-array truthiness is consulted.
        match e {
            Expression::Parenthesized(p) => self.condition(p.expression),
            Expression::Assignment(a) if a.operator.is_assign() => self.condition(a.rhs),
            Expression::UnaryPrefix(u) if matches!(u.operator, UnaryPrefixOperator::Not(_)) => {
                self.condition(u.operand).negate()
            }
            Expression::Array(a) => Tri::from(!a.elements.is_empty()),
            Expression::LegacyArray(a) => Tri::from(!a.elements.is_empty()),
            e => flow::Results {
                types: &self.c.types,
                call_terminations: Some(&self.c.call_terminations),
                contents: &self.c.file.contents,
                codebase: Some(self.c.codebase),
            }
            .condition(e),
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
            Expression::Assignment(a) if matches!(a.operator, AssignmentOperator::Coalesce(_)) => {
                // AssignOpHandler prepares the read target BEFORE evaluating the value. Only a
                // direct local or a single plain-array dimension has a complete producer here.
                let lhs_array =
                    self.array(a.lhs) || self.c.type_of(a.lhs).is_some_and(|t| t.is_array());
                let lhs_null = self.c.type_of(a.lhs).is_some_and(|t| t.is_null());
                let mut effects = match a.lhs {
                    Expression::Variable(Variable::Direct(_)) => self.expression(a.lhs)?,
                    Expression::ArrayAccess(offset) => {
                        let Expression::Variable(Variable::Direct(v)) = offset.array else {
                            return None;
                        };
                        if self.referenced
                            || (!self.array(offset.array)
                                && !self.c.type_of(offset.array).is_some_and(|t| t.is_array()))
                        {
                            return None;
                        }
                        let mut read = self.expression(offset.array)?;
                        self.arrays.insert(v.name.to_vec());
                        let dimension = self.expression(offset.index)?;
                        if self.referenced || !self.array(offset.array) {
                            return None; // The dimension may replace or escape the prepared container.
                        }
                        // AssignHandler::prepareTarget forwards dimension throw points, but neither
                        // its impure points nor its termination flag (unlike ArrayDimFetchHandler).
                        read.throws |= dimension.throws;
                        read
                    }
                    _ => return None,
                };
                let before = self.arrays.clone();
                // PHPStan retains the RHS's points even for a surely non-null target. Its exit is
                // unconditional only when the prepared read is definitely null.
                let mut value = self.expression(a.rhs)?;
                let array = self.array(a.rhs);
                value.terminates &= lhs_null;
                effects.merge(value);
                self.arrays.retain(|name| before.contains(name));
                if let Expression::Variable(Variable::Direct(v)) = a.lhs {
                    self.arrays.remove(v.name);
                    if !self.referenced && (lhs_array || (lhs_null && array)) {
                        self.arrays.insert(v.name.to_vec());
                    }
                }
                Some(effects)
            }
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
            // BinaryOpHandler adds no call/throw points for comparisons; it merges both operands.
            Expression::Binary(b) if b.operator.is_comparison() => {
                let mut effects = self.expression(b.lhs)?;
                effects.merge(self.expression(b.rhs)?);
                Some(effects)
            }
            Expression::UnaryPrefix(u) if matches!(u.operator, UnaryPrefixOperator::Not(_)) => {
                self.expression(u.operand)
            }
            Expression::UnaryPrefix(u) if u.operator.is_reference() => {
                let effects = self.expression(u.operand)?;
                self.arrays.clear(); // An alias can subsequently replace the entire container.
                self.referenced = true;
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
            Expression::ArrayAccess(a) => {
                // ArrayDimFetchHandler evaluates the dimension first. Plain arrays never invoke
                // offsetGet; unlike writes, a read can also use an already-observed array type.
                let mut effects = self.expression(a.index)?;
                if !self.array(a.array) && !self.c.type_of(a.array).is_some_and(|t| t.is_array()) {
                    return None;
                }
                effects.merge(self.expression(a.array)?);
                Some(effects)
            }
            Expression::Throw(t) => {
                let mut effects = self.expression(t.exception)?;
                effects.throws = true;
                effects.terminates = true;
                Some(effects)
            }
            Expression::Instantiation(new) => self.instantiation(new),
            Expression::Call(Call::Function(call)) => self.call(call),
            _ => None,
        }
    }

    /// NewHandler's named-class path: argument results and reflected constructor effects, not the
    /// constructor body's apparent purity. Legality and the stored object/never type are independent.
    fn instantiation(&mut self, new: &Instantiation) -> Option<Effects> {
        let Expression::Identifier(id) = new.class else {
            return None; // Dynamic, relative and anonymous targets need their own reflection producers.
        };
        let mut effects = Effects::default();
        for arg in new
            .argument_list
            .iter()
            .flat_map(|list| list.arguments.iter())
        {
            if matches!(arg, Argument::Positional(p) if p.ellipsis.is_some())
                || matches!(
                    arg.value(),
                    Expression::Closure(_) | Expression::ArrowFunction(_)
                )
            {
                return None; // Unpack normalization and escaping callbacks are not modeled here.
            }
            effects.merge(self.expression(arg.value())?);
        }
        let class = self.c.class_like(&self.c.resolved(id));
        if matches!(class, Lookup::NotFound) {
            // Both lists are nonempty even when InstantiationRule also reports class.notFound.
            effects.impure = true;
            effects.throws = true;
            self.arrays.clear();
        } else if let Some(constructor) = members::method(self.c, class.found()?, b"__construct") {
            let members::Method::User { declaring, f, .. } = constructor else {
                // Built-in reflection does not yet carry constructor throw types/extensions.
                return None;
            };
            self.c.file_contents(f.span.file_id)?;
            let raw = declared::doc_at(self.c, f.span)
                .map(|(raw, _)| raw)
                .unwrap_or_default();
            let (pure, unless_callable, unless_passed) =
                declared::method_purity(self.c, declaring, b"__construct");
            if unless_callable
                || unless_passed
                || f.parameters.iter().any(|p| p.flags.is_by_reference())
                || declarations::doc_tags(&raw)
                    .iter()
                    .any(|tag| tag == "@param-immediately-invoked-callable")
            {
                return None;
            }
            // PhpMethodReflection exempts constructors from the void-return-implies-impure rule.
            effects.impure |= pure != Some(true);
            if pure != Some(true) {
                self.arrays.clear();
            }
            let tags = doc::tags(&raw);
            let kind = if tags.iter().any(|(name, _)| name == "@phpstan-throws") {
                "@phpstan-throws"
            } else {
                "@throws"
            };
            let throws: Vec<_> = tags.iter().filter(|(name, _)| name == kind).collect();
            if throws.iter().any(|(_, value)| value.contains(&b'$')) {
                return None; // Conditional throw types require argument-sensitive reflection.
            }
            // No tag produces an implicit point; every non-void tag produces an explicit point,
            // including never. Do not reduce the list to the union of its types.
            effects.throws |= throws.is_empty()
                || !throws.iter().all(|(_, value)| {
                    value.split(u8::is_ascii_whitespace).find(|s| !s.is_empty())
                        == Some(b"void".as_slice())
                });
        }
        // A known named class without a constructor adds neither point, even if it is not instantiable.
        Some(effects)
    }

    fn array(&self, e: &Expression) -> bool {
        match e {
            Expression::Array(_) | Expression::LegacyArray(_) => true,
            Expression::Variable(Variable::Direct(v)) => self.arrays.contains(v.name),
            _ => false,
        }
    }

    /// UnsetHandler evaluates each operand, then removes the local or virtually assigns its new array.
    fn unset(&mut self, e: &Expression) -> Option<Effects> {
        match e {
            Expression::Variable(Variable::Direct(v)) => {
                self.arrays.remove(v.name);
                self.expression(e) // VariableHandler still produces superglobal access points.
            }
            Expression::ArrayAccess(a) => {
                let Expression::Variable(Variable::Direct(v)) = a.array else {
                    return None; // Nested/property roots need their own reads and virtual write-back.
                };
                // ArrayDimFetchHandler walks the dimension BEFORE reading the receiver. It may replace
                // the container or escape an alias, so neither proof may be taken before this walk.
                let mut effects = self.expression(a.index)?;
                if self.referenced
                    || (!self.array(a.array)
                        && !self.c.type_of(a.array).is_some_and(|t| t.is_array()))
                {
                    return None; // Unknown/ArrayAccess containers need offsetGet AND offsetUnset points.
                }
                effects.merge(self.expression(a.array)?);
                // Removing an offset preserves the container's array kind, unlike removing the local.
                self.arrays.insert(v.name.to_vec());
                Some(effects)
            }
            // Property unsets add an impure point; dynamic variables/static properties need producers.
            _ => None,
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
                self.referenced = true;
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
        } else if !throws.iter().all(|(_, value)| {
            value.split(u8::is_ascii_whitespace).find(|s| !s.is_empty()) == Some(b"void".as_slice())
        }) {
            return None;
        }
        self.arrays.clear();
        Some(effects)
    }
}
