//! `lint/pest/prefer-expect`, ported from pest-plugin-rector's `ConvertAssertToExpectRector`: `$this->assertX(...)`
//! becomes the `expect()` chain with Pest's matching matcher, such as `$this->assertSame(42, $value)` to
//! `expect($value)->toBe(42)`.

use mago_syntax::cst::{Expression, Node, NodeKind, Variable};
use pun_problem::{Edit, Safety};

use super::{argument_value, identifier, indent_at, kept_text, report};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct PreferExpect;

/// `SINGLE_ARG_ASSERTIONS`, `TWO_ARG_ASSERTIONS` and `THREE_ARG_ASSERTIONS`: the matcher, whether it is negated, and
/// how many arguments the assertion needs.
fn assertion(name: &[u8]) -> Option<(&'static str, bool, usize)> {
    Some(match name {
        b"assertTrue" => ("toBeTrue", false, 1),
        b"assertFalse" => ("toBeFalse", false, 1),
        b"assertNull" => ("toBeNull", false, 1),
        b"assertNotNull" => ("toBeNull", true, 1),
        b"assertEmpty" => ("toBeEmpty", false, 1),
        b"assertNotEmpty" => ("toBeEmpty", true, 1),
        b"assertIsArray" => ("toBeArray", false, 1),
        b"assertIsList" => ("toBeList", false, 1),
        b"assertIsBool" => ("toBeBool", false, 1),
        b"assertIsFloat" => ("toBeFloat", false, 1),
        b"assertIsInt" => ("toBeInt", false, 1),
        b"assertIsString" => ("toBeString", false, 1),
        b"assertIsNumeric" => ("toBeNumeric", false, 1),
        b"assertIsObject" => ("toBeObject", false, 1),
        b"assertIsCallable" => ("toBeCallable", false, 1),
        b"assertIsIterable" => ("toBeIterable", false, 1),
        b"assertIsScalar" => ("toBeScalar", false, 1),
        b"assertIsResource" => ("toBeResource", false, 1),
        b"assertNan" => ("toBeNan", false, 1),
        b"assertInfinite" => ("toBeInfinite", false, 1),
        b"assertFileExists" => ("toBeFile", false, 1),
        b"assertDirectoryExists" => ("toBeDirectory", false, 1),
        b"assertFileIsReadable" => ("toBeReadableFile", false, 1),
        b"assertFileIsWritable" => ("toBeWritableFile", false, 1),
        b"assertDirectoryIsReadable" => ("toBeReadableDirectory", false, 1),
        b"assertDirectoryIsWritable" => ("toBeWritableDirectory", false, 1),
        b"assertJson" => ("toBeJson", false, 1),
        b"assertIsNotArray" => ("toBeArray", true, 1),
        b"assertIsNotBool" => ("toBeBool", true, 1),
        b"assertIsNotFloat" => ("toBeFloat", true, 1),
        b"assertIsNotInt" => ("toBeInt", true, 1),
        b"assertIsNotString" => ("toBeString", true, 1),
        b"assertIsNotNumeric" => ("toBeNumeric", true, 1),
        b"assertIsNotObject" => ("toBeObject", true, 1),
        b"assertIsNotCallable" => ("toBeCallable", true, 1),
        b"assertIsNotIterable" => ("toBeIterable", true, 1),
        b"assertIsNotScalar" => ("toBeScalar", true, 1),
        b"assertIsNotResource" => ("toBeResource", true, 1),
        b"assertNotTrue" => ("toBeTrue", true, 1),
        b"assertNotFalse" => ("toBeFalse", true, 1),
        b"assertEquals" => ("toEqual", false, 2),
        b"assertEqualsCanonicalizing" => ("toEqualCanonicalizing", false, 2),
        b"assertNotEquals" => ("toEqual", true, 2),
        b"assertSame" => ("toBe", false, 2),
        b"assertNotSame" => ("toBe", true, 2),
        b"assertCount" => ("toHaveCount", false, 2),
        b"assertSameSize" => ("toHaveSameSize", false, 2),
        b"assertInstanceOf" => ("toBeInstanceOf", false, 2),
        b"assertNotInstanceOf" => ("toBeInstanceOf", true, 2),
        b"assertContains" => ("toContain", false, 2),
        b"assertContainsEquals" => ("toContainEqual", false, 2),
        b"assertNotContains" => ("toContain", true, 2),
        b"assertNotContainsEquals" => ("toContainEqual", true, 2),
        b"assertContainsOnlyInstancesOf" => ("toContainOnlyInstancesOf", false, 2),
        b"assertArrayHasKey" => ("toHaveKey", false, 2),
        b"assertArrayNotHasKey" => ("toHaveKey", true, 2),
        b"assertObjectHasProperty" => ("toHaveProperty", false, 2),
        b"assertObjectNotHasProperty" => ("toHaveProperty", true, 2),
        b"assertStringContainsString" => ("toContain", false, 2),
        b"assertStringNotContainsString" => ("toContain", true, 2),
        b"assertStringStartsWith" => ("toStartWith", false, 2),
        b"assertStringEndsWith" => ("toEndWith", false, 2),
        b"assertMatchesRegularExpression" => ("toMatch", false, 2),
        b"assertGreaterThan" => ("toBeGreaterThan", false, 2),
        b"assertGreaterThanOrEqual" => ("toBeGreaterThanOrEqual", false, 2),
        b"assertLessThan" => ("toBeLessThan", false, 2),
        b"assertLessThanOrEqual" => ("toBeLessThanOrEqual", false, 2),
        b"assertEqualsWithDelta" => ("toEqualWithDelta", false, 3),
        _ => return None,
    })
}

impl Rule for PreferExpect {
    fn id(&self) -> &'static str {
        "lint/pest/prefer-expect"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::MethodCall])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::MethodCall(call) = node else {
            return;
        };
        let Expression::Variable(Variable::Direct(this)) = unparenthesize(call.object) else {
            return;
        };
        if !this.name.eq_ignore_ascii_case(b"$this") {
            return;
        }
        let Some((matcher, negated, needed)) = identifier(&call.method).and_then(assertion) else {
            return;
        };
        let arguments = call.argument_list.arguments.as_slice();
        if arguments.len() < needed {
            return;
        }
        // The checked value is the only argument, or the second; the expected value and the third go to the matcher.
        let span = node_span(call);
        let indent = indent_at(ctx, span.start);
        let text = |i: usize| {
            kept_text(
                ctx,
                node_span(unparenthesize(argument_value(&arguments[i]))),
                indent,
            )
        };
        let (subject, args) = match needed {
            1 => (text(0), String::new()),
            2 => (text(1), text(0)),
            _ => (text(1), format!("{}, {}", text(0), text(2))),
        };
        let not = if negated { "->not" } else { "" };
        out.push(report(
            span,
            format!(
                "Use `expect()->{matcher}()` instead of `$this->{}()`.",
                String::from_utf8_lossy(identifier(&call.method).unwrap_or_default())
            ),
            Safety::Safe,
            vec![Edit::new(
                span,
                format!("expect({subject}){not}->{matcher}({args})"),
            )],
        ));
    }
}
