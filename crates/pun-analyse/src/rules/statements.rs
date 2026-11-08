//! Keyword, cast and context checks that need no types: MagicConstantContextRule, DeclareStrictTypesRule,
//! GotoUndefinedLabelRule, ConstructorReturnTypeRule, DeprecatedCastRule, UnsetCastRule, VoidCastRule,
//! BacktickRule, NoncapturingCatchRule, ThrowExpressionRule, InstantiationCallableRule and the PHP version branch of
//! the first-class callable rules.

use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;

use super::Checker;

/// What a scope boundary on [`Checker::frames`] is.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum FrameKind {
    /// The file's top level.
    Top,
    /// A class-like body: a fresh scope with no function.
    Class,
    /// A function, method or property hook (`$scope->getFunction()`).
    Named,
    /// A closure or arrow function (`$scope->isInAnonymousFunction()`).
    Anonymous,
    /// A parameter: analysed in the scope around its function-like.
    Param,
}

/// A scope boundary, with the goto labels and gotos directly in it (GotoLabelVisitor's scope stack).
pub(super) struct Frame {
    pub kind: FrameKind,
    labels: Vec<(Vec<u8>, Span)>,
    gotos: Vec<(Vec<u8>, Span)>,
    /// A function-like's variables (WrongVariableNameInVarTagRule's `hasVariableType()`).
    pub defined: Option<Box<super::var_names::Defined>>,
}

impl Frame {
    pub(super) fn new(kind: FrameKind) -> Frame {
        Frame {
            kind,
            labels: Vec::new(),
            gotos: Vec::new(),
            defined: None,
        }
    }
}

impl Checker<'_, '_> {
    pub(super) fn push_frame(&mut self, kind: FrameKind) {
        self.frames.push(Frame::new(kind));
    }

    /// Leaves a scope boundary: GotoUndefinedLabelRule reports its gotos to labels it does not hold.
    pub(super) fn pop_frame(&mut self) {
        let frame = self.frames.pop().expect("pushed");
        for (label, at) in &frame.gotos {
            if frame.labels.iter().any(|(l, _)| l == label) {
                continue;
            }
            self.report_non_ignorable(
                at,
                "goto.labelUndefined",
                format!(
                    "Goto to undefined label '{}'.",
                    String::from_utf8_lossy(label)
                ),
                None,
            );
        }
        // UnusedLabelRule (level 4, `featureToggles.unusedLabel`)
        if self.on(crate::rules::Switches::UNUSED_LABEL) {
            for (label, at) in &frame.labels {
                if !frame.gotos.iter().any(|(g, _)| g == label) {
                    let message = format!("Label '{}' is unused.", String::from_utf8_lossy(label));
                    self.report(*at, "label.unused", message, None);
                }
            }
        }
    }

    /// The innermost function-like a node is analysed in, up to the enclosing class-like: a parameter belongs to the
    /// scope around its function-like.
    fn function_frames(&self) -> impl Iterator<Item = FrameKind> + '_ {
        let mut skip = 0;
        self.frames
            .iter()
            .rev()
            .map(|f| f.kind)
            .take_while(|k| *k != FrameKind::Class)
            .filter(move |k| match k {
                FrameKind::Param => {
                    skip += 1;
                    false
                }
                _ if skip > 0 => {
                    skip -= 1;
                    false
                }
                _ => true,
            })
    }

    /// `$scope->getFunction() !== null`.
    pub(super) fn in_named_function(&self) -> bool {
        self.function_frames().any(|k| k == FrameKind::Named)
    }

    /// `$scope->getFunctionName() !== null || $scope->isInAnonymousFunction()`.
    fn in_function_like(&self) -> bool {
        self.function_frames()
            .any(|k| matches!(k, FrameKind::Named | FrameKind::Anonymous))
    }

    pub(super) fn label(&mut self, l: &Label) {
        let frame = self.frames.last_mut().expect("top frame");
        frame.labels.push((l.name.value.to_vec(), l.span()));
    }

    pub(super) fn goto(&mut self, g: &Goto) {
        let frame = self.frames.last_mut().expect("top frame");
        frame.gotos.push((g.label.value.to_vec(), g.span()));
    }
}

/// MagicConstantContextRule.
pub(super) fn magic_constant(c: &mut Checker, m: &MagicConstant) {
    let (name, identifier, message) = match m {
        MagicConstant::Class(_) if c.classes.is_empty() => {
            ("__CLASS__", "magicConstant.outOfClass", "outside a class")
        }
        MagicConstant::Trait(_) if !c.walk.contains(super::Walk::IN_TARGET) => {
            ("__TRAIT__", "magicConstant.outOfTrait", "outside a trait")
        }
        MagicConstant::Method(_) | MagicConstant::Function(_)
            if !c.in_function_like() && !c.magic_defaults.contains(&m.span()) =>
        {
            let name = match m {
                MagicConstant::Method(_) => "__METHOD__",
                _ => "__FUNCTION__",
            };
            (name, "magicConstant.outOfFunction", "outside a function")
        }
        MagicConstant::Namespace(_) if c.namespaces.last().is_none_or(|n| n.is_none()) => (
            "__NAMESPACE__",
            "magicConstant.outOfNamespace",
            "in global namespace",
        ),
        _ => return,
    };
    c.report(
        m,
        identifier,
        format!("Magic constant {name} is always empty {message}."),
        None,
    );
}

/// MagicConstantParamDefaultVisitor: a parameter's default that is a magic constant.
pub(super) fn parameter_default(c: &mut Checker, p: &FunctionLikeParameter) {
    if let Some(d) = &p.default_value
        && let Expression::MagicConstant(m) = d.value
    {
        c.magic_defaults.insert(m.span());
    }
}

/// DeclarePositionVisitor: whether each `declare` is the file's first statement (a shebang line before it is not a
/// statement, nor are the tags).
pub(super) fn statement_position(c: &mut Checker, s: &Statement) {
    match s {
        Statement::OpeningTag(_) | Statement::ClosingTag(_) => {}
        Statement::Inline(i)
            if c.walk.contains(super::Walk::FIRST_STATEMENT) && i.value.starts_with(b"#!") => {}
        // PHP's `?>` token takes the newline after it, so php-parser has no InlineHTML for it
        Statement::Inline(i)
            if matches!(i.value, b"\n" | b"\r\n")
                && c.file.contents[..i.span.start.offset as usize].ends_with(b"?>") => {}
        Statement::Declare(d) => {
            if !c.walk.contains(super::Walk::FIRST_STATEMENT) {
                c.declares_not_first.push(d.span());
            }
            c.walk.remove(super::Walk::FIRST_STATEMENT);
        }
        _ => c.walk.remove(super::Walk::FIRST_STATEMENT),
    }
}

/// DeclareStrictTypesRule.
pub(super) fn declare(c: &mut Checker, d: &Declare) {
    let Some(item) = d.items.iter().find(|i| i.name.value == b"strict_types") else {
        return;
    };
    if !matches!(item.value, Expression::Literal(Literal::Integer(i)) if matches!(i.value, Some(0 | 1)))
    {
        let value = print_expr(c, item.value);
        c.report_non_ignorable(
            d,
            "declareStrictTypes.value",
            format!("Declare strict_types must have 0 or 1 as its value, {value} given."),
            None,
        );
        return;
    }
    if c.declares_not_first.contains(&d.span()) {
        c.report_non_ignorable(
            d,
            "declareStrictTypes.notFirst",
            "Declare strict_types must be the very first statement.".to_string(),
            None,
        );
    }
}

/// ExprPrinter for a declare value: a constant prints by its resolved name (`\true`), anything else as written.
// assistant's call: php-parser's printer reprints literals in their written form, so the source text stands in for it
fn print_expr(c: &Checker, e: &Expression) -> String {
    let written = || c.written(e.span());
    match e {
        Expression::Literal(Literal::True(_) | Literal::False(_) | Literal::Null(_)) => {
            format!("\\{}", written())
        }
        Expression::ConstantAccess(a) => match &a.name {
            Identifier::FullyQualified(_) => written(),
            Identifier::Local(_) if c.namespaces.last().is_some_and(|n| n.is_some()) => written(),
            _ => format!("\\{}", c.resolved(&a.name)),
        },
        _ => written(),
    }
}

/// ConstructorReturnTypeRule (`InClassMethodNode`).
pub(super) fn constructor_return_type(c: &mut Checker, m: &Method) {
    let (Some(_), Some(class)) = (&m.return_type_hint, c.class_scope()) else {
        return;
    };
    let mut name = m.name.value.to_ascii_lowercase();
    if c.walk.contains(super::Walk::IN_TARGET) {
        // processNodesForTraitUse: a method the class aliases is analysed under the alias, remembering its name
        let alias = class
            .trait_alias_map
            .iter()
            .find(|(_, original)| original.as_bytes().eq_ignore_ascii_case(&name))
            .map(|(alias, _)| alias.as_bytes().to_ascii_lowercase());
        if let Some(alias) = alias {
            if name == b"__construct" {
                let t = c.in_trait().unwrap_or_default().to_string();
                let display = c
                    .codebase
                    .get_class_like(t.as_bytes())
                    .map_or_else(|| t.clone(), |k| c.display(k));
                c.report_non_ignorable(
                    m,
                    "constructor.returnType",
                    format!("Original constructor of trait {display} has a return type."),
                    None,
                );
                return;
            }
            name = alias;
        }
        // (a trait method the class declares itself is not walked: Checker::shadowed())
    }
    if name != b"__construct" {
        return;
    }
    let display = c.display(class);
    c.report_non_ignorable(
        m,
        "constructor.returnType",
        format!("Constructor of class {display} has a return type."),
        None,
    );
}

/// DeprecatedCastRule, UnsetCastRule and VoidCastRule.
pub(super) fn cast(c: &mut Checker, u: &UnaryPrefix) {
    let deprecated = match u.operator {
        UnaryPrefixOperator::IntegerCast(..) => Some(("integer", "int")),
        UnaryPrefixOperator::BooleanCast(..) => Some(("boolean", "bool")),
        UnaryPrefixOperator::DoubleCast(..) => Some(("double", "float")),
        UnaryPrefixOperator::BinaryCast(..) => Some(("binary", "string")),
        _ => None,
    };
    if let Some((written, standard)) = deprecated {
        // PhpVersion::deprecatesNonStandardCasts()
        if c.surely_at_least(80500) {
            c.report(
                u,
                "cast.deprecated",
                format!(
                    "Non-standard ({written}) cast is deprecated in PHP 8.5. Use ({standard}) instead."
                ),
                None,
            );
        }
    } else if matches!(u.operator, UnaryPrefixOperator::UnsetCast(..)) {
        // PhpVersion::supportsUnsetCast()
        if !c.surely_below(80000) {
            c.report_non_ignorable(
                u,
                "cast.unset",
                "The (unset) cast is no longer supported in PHP 8.0 and later.".to_string(),
                None,
            );
        }
    } else if matches!(u.operator, UnaryPrefixOperator::VoidCast(..)) {
        // PhpVersion::supportsVoidCast()
        if !c.surely_at_least(80500) {
            c.report_non_ignorable(
                u,
                "cast.voidNotSupported",
                "The (void) cast is supported only on PHP 8.5 and later.".to_string(),
                None,
            );
        }
        // PHP's grammar has `(void) expr;` as a statement, so `(void) 5 |> f(...);` casts the whole pipe (Mago
        // parses the cast as the pipe's input)
        let start = u.span().start.offset;
        if !c.top_level.iter().any(|(s, _)| *s == start) {
            c.report_non_ignorable(
                u,
                "cast.void",
                "The (void) cast cannot be used within an expression.".to_string(),
                None,
            );
        }
    }
}

/// BacktickRule.
pub(super) fn backtick(c: &mut Checker, s: &ShellExecuteString) {
    // PhpVersion::deprecatesBacktickOperator()
    if c.php_version < 80500 {
        return;
    }
    let count = c.errors.len();
    c.report(
        s,
        "backtick.deprecated",
        "Backtick operator is deprecated in PHP 8.5. Use shell_exec() function call instead."
            .to_string(),
        None,
    );
    // its fix replaces the backticks with a shell_exec() call
    c.fixable_since(count);
}

/// NoncapturingCatchRule.
pub(super) fn noncapturing_catch(c: &mut Checker, clause: &TryCatchClause) {
    if clause.variable.is_some() || c.surely_at_least(80000) {
        return;
    }
    c.report_non_ignorable(
        clause,
        "catch.nonCapturingNotSupported",
        "Non-capturing catch is supported only on PHP 8.0 and later.".to_string(),
        None,
    );
}

/// StandaloneThrowExprVisitor: a `throw` that is an expression statement.
pub(super) fn standalone_throw(c: &mut Checker, s: &ExpressionStatement) {
    if let Expression::Throw(t) = super::types::flow::unparen(s.expression) {
        c.standalone_throws.insert(t.span());
    }
}

/// ThrowExpressionRule.
pub(super) fn throw_expression(c: &mut Checker, t: &Throw) {
    // PhpVersion::supportsThrowExpression()
    if c.surely_at_least(80000) || c.standalone_throws.contains(&t.span()) {
        return;
    }
    c.report_non_ignorable(
        t,
        "throw.notSupported",
        "Throw expression is supported only on PHP 8.0 and later.".to_string(),
        None,
    );
}

/// The PHP version branch of FunctionCallableRule, MethodCallableRule and StaticMethodCallableRule on `f(...)` at
/// `at`: whether it reported (the rest of the rule is then skipped).
pub(super) fn first_class_callable(c: &mut Checker, at: Span, args: &PartialArgumentList) -> bool {
    // PhpVersion::supportsFirstClassCallables(); `x |> f(...)` is the call `f(x)`
    if c.surely_at_least(80100)
        || c.piped.contains(&at)
        || !matches!(
            args.arguments.iter().collect::<Vec<_>>().as_slice(),
            [PartialArgument::VariadicPlaceholder(_)]
        )
    {
        return false;
    }
    c.report_non_ignorable(
        at,
        "callable.notSupported",
        "First-class callables are supported only on PHP 8.1 and later.".to_string(),
        None,
    );
    true
}

/// InstantiationCallableRule: `new X(...)`, which Mago parses as one unpacked argument that is a parse error. Whether it
/// reported (NodeScopeResolver then analyses no `new`).
pub(super) fn instantiation_callable(c: &mut Checker, new: &Instantiation) -> bool {
    let Some(list) = &new.argument_list else {
        return false;
    };
    if !matches!(
        list.arguments.iter().collect::<Vec<_>>().as_slice(),
        [Argument::Positional(PositionalArgument {
            ellipsis: Some(_),
            value: Expression::Error(_),
        })]
    ) {
        return false;
    }
    c.report_non_ignorable(
        new,
        "callable.notSupported",
        "Cannot create callable from the new operator.".to_string(),
        None,
    );
    true
}
