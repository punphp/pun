//! `lint/phpunit/add-mock-object-type`, ported from rector-phpunit's `ExpectsParamToMockObjectRector`
//! (rules/PHPUnit110/Rector/ClassMethod).
//!
//! In a test class, a private method's parameter that the method calls `expects()` on (outside closures) gets the
//! type `\PHPUnit\Framework\MockObject\MockObject` in place of its own, unless that type names `MockObject` already
//! or a `@param` tag of the method names the parameter.

use mago_syntax::cst::{
    Expression, FunctionLikeParameter, Hint, MethodBody, Modifier, Node, NodeKind, Variable,
};
use pun_problem::{Edit, Safety, Span};

use super::annotations::Annotated;
use super::no_bare_mock_assign::scoped;
use super::phpdoc_type::typed_tag;
use super::{AnyCall, is_in_test_class, report, resolve};
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

/// `PHPUnitClassName::MOCK_OBJECT`.
const MOCK_OBJECT: &str = "PHPUnit\\Framework\\MockObject\\MockObject";

pub struct AddMockObjectType;

impl Rule for AddMockObjectType {
    fn id(&self) -> &'static str {
        "lint/phpunit/add-mock-object-type"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Method])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Method(method) = node else { return };
        if !method
            .modifiers
            .iter()
            .any(|m| matches!(m, Modifier::Private(_)))
        {
            return;
        }
        let MethodBody::Concrete(body) = &method.body else {
            return;
        };
        if method.parameter_list.parameters.is_empty()
            || !is_in_test_class(ctx, ancestors, Some(method))
        {
            return;
        }
        let doc = Annotated::of(ctx.source, node, ancestors).and_then(|a| a.doc);
        let roots: Vec<Node<'_, '_>> = body.statements.iter().map(Node::Statement).collect();
        let mut edits = Vec::new();
        for param in method.parameter_list.parameters.iter() {
            if param
                .hint
                .as_ref()
                .is_some_and(|h| has_mock_object_type(ancestors, h))
            {
                continue;
            }
            let name = &param.variable.name;
            // avoid contradicting the docblock type, that would have to be changed as well
            if let Some(doc) = &doc {
                match typed_tag(doc, "@param", Some(&String::from_utf8_lossy(name))) {
                    Ok(None) => {}
                    _ => continue,
                }
            }
            if !is_expects_called_on_variable(&roots, &name[1..]) {
                continue;
            }
            edits.push(type_edit(param, &format!("\\{MOCK_OBJECT}")));
        }
        if edits.is_empty() {
            return;
        }
        out.push(report(
            node_span(&method.name),
            "Type the parameter that gets expects() as a MockObject.",
            Safety::Safe,
            edits,
        ));
    }
}

/// The edit that gives `param` the type `ty`, in place of its own.
pub(super) fn type_edit(param: &FunctionLikeParameter<'_>, ty: &str) -> Edit {
    match &param.hint {
        Some(hint) => Edit::new(node_span(hint), ty),
        None => {
            let at = [param.ampersand, param.ellipsis]
                .into_iter()
                .flatten()
                .map(|s| s.start.offset as usize)
                .chain([node_span(&param.variable).start])
                .min()
                .unwrap();
            Edit::new(Span::new(at, at), format!("{ty} "))
        }
    }
}

/// `hasMockObjectType()`: the type names `MockObject`, alone, nullable, or in a union or an intersection.
fn has_mock_object_type(scope: &[Node<'_, '_>], hint: &Hint<'_>) -> bool {
    match hint {
        Hint::Identifier(id) => resolve(scope, id).eq_ignore_ascii_case(MOCK_OBJECT),
        Hint::Nullable(n) => has_mock_object_type(scope, n.hint),
        Hint::Union(u) => {
            has_mock_object_type(scope, u.left) || has_mock_object_type(scope, u.right)
        }
        Hint::Intersection(i) => {
            has_mock_object_type(scope, i.left) || has_mock_object_type(scope, i.right)
        }
        _ => false,
    }
}

/// `isExpectsCalledOnVariable()`: `$name->expects(...)` outside closures.
fn is_expects_called_on_variable(roots: &[Node<'_, '_>], name: &[u8]) -> bool {
    let mut found = false;
    scoped(roots, &mut |node| {
        if let Node::MethodCall(call) = node
            && matches!(unparenthesize(call.object), Expression::Variable(Variable::Direct(v)) if v.name[1..].eq_ignore_ascii_case(name))
            && AnyCall::of(node).is_some_and(|c| c.is_name(&["expects"]))
        {
            found = true;
        }
    });
    found
}
