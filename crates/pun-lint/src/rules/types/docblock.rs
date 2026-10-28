//! What Rector 2.6.1's docblock type rules (rules/TypeDeclarationDocblocks and the docblock rules of
//! rules/TypeDeclaration) share: a function or method as a `ClassMethod|Function_`, `ReturnNodeFinder`, and the
//! report whose fix writes the changed doc comment through [`crate::rules::phpdoc_writer`].

use mago_syntax::cst::{
    Block, FunctionLikeParameterList, FunctionLikeReturnTypeHint, Hint, Method, MethodBody, Node, Return,
};
use pun_phpstan::types::Type;
use pun_problem::{Edit, Fix, Safety, Span};

use super::silent_void::Place;
use super::{class_scope, has_only_return_with_expr, returns_scoped};
use crate::Report;
use crate::rules::phpdoc_writer::{DocTarget, PhpDocInfo};
use crate::syntax::node_span;

/// A `ClassMethod` or `Function_`.
pub(crate) struct FunctionLike<'ast, 'arena> {
    /// The name.
    pub name: Span,
    pub name_text: &'arena [u8],
    /// php-parser's start of the node (its first attribute or modifier).
    pub start: usize,
    /// The body; `None` for an abstract method.
    pub body: Option<&'ast Block<'arena>>,
    pub parameters: &'ast FunctionLikeParameterList<'arena>,
    pub return_type: Option<&'ast FunctionLikeReturnTypeHint<'arena>>,
    /// The method, when it is one.
    pub method: Option<&'ast Method<'arena>>,
}

impl<'ast, 'arena> FunctionLike<'ast, 'arena> {
    pub fn of(node: Node<'ast, 'arena>) -> Option<FunctionLike<'ast, 'arena>> {
        match node {
            Node::Function(f) => Some(FunctionLike {
                name: node_span(&f.name),
                name_text: f.name.value,
                start: node_span(f).start,
                body: Some(&f.body),
                parameters: &f.parameter_list,
                return_type: f.return_type_hint.as_ref(),
                method: None,
            }),
            Node::Method(m) => Some(FunctionLike {
                name: node_span(&m.name),
                name_text: m.name.value,
                start: node_span(m).start,
                body: match &m.body {
                    MethodBody::Concrete(body) => Some(body),
                    MethodBody::Abstract(_) => None,
                },
                parameters: &m.parameter_list,
                return_type: m.return_type_hint.as_ref(),
                method: Some(m),
            }),
            _ => None,
        }
    }

    /// `$node->returnType instanceof Node && ! $this->isName($node->returnType, 'array')`: a return type other than
    /// `array`.
    pub fn has_non_array_return_type(&self) -> bool {
        self.return_type.is_some_and(|r| !is_array_hint(&r.hint))
    }

    /// Whether the return type is `array`.
    pub fn returns_array(&self) -> bool {
        self.return_type.is_some_and(|r| is_array_hint(&r.hint))
    }

    /// Whether it is a private method.
    pub fn is_private(&self) -> bool {
        self.method.is_some_and(|m| m.modifiers.iter().any(|m| m.is_private()))
    }

    /// `ReturnNodeFinder::findOnlyReturnWithExpr()`: the one `return` of the body, when it has a value and the end of
    /// the body cannot be reached.
    pub fn only_return_with_expr(
        &self,
        place: &Place<'_, '_, '_>,
    ) -> Option<&'ast Return<'arena>> {
        let body = self.body?;
        let returns = returns_scoped(body);
        if !has_only_return_with_expr(place, body, &returns) || returns.len() != 1 {
            return None;
        }
        Some(returns[0])
    }

    /// The doc comment target of the node.
    pub fn target(&self, source: &str) -> DocTarget {
        DocTarget::of(source, self.start)
    }
}

/// `$this->isName($typeNode, 'array')`: the hint is `array` (case-insensitively).
pub(crate) fn is_array_hint(hint: &Hint<'_>) -> bool {
    matches!(hint, Hint::Array(_))
}

/// The class `ancestors` lead to, for `self` in a doc type.
pub(crate) fn class_name(ancestors: &[Node<'_, '_>]) -> Option<String> {
    class_scope(ancestors).and_then(|s| s.name)
}

/// The report of a rule that changed `info`, the doc comment of `target`.
pub(crate) fn doc_report(
    span: Span,
    message: &str,
    source: &str,
    target: &DocTarget,
    info: &PhpDocInfo,
) -> Report {
    let edit: Edit = target.edit(source, &info.print());
    Report {
        span,
        message: message.to_string(),
        help: None,
        fix: Some(Fix {
            safety: Safety::Unsafe,
            edits: vec![edit],
        }),
    }
}

/// `Type::getReferencedClasses()`.
pub(crate) fn referenced_classes(t: &Type) -> Vec<String> {
    match t {
        Type::Object(name, args) => {
            let mut out = vec![name.clone()];
            out.extend(args.iter().flat_map(referenced_classes));
            out
        }
        Type::EnumCase(name, _) => vec![name.clone()],
        Type::ClassString(Some(inner)) => referenced_classes(inner),
        Type::Union(types) | Type::Benevolent(types) | Type::Intersection(types) => {
            types.iter().flat_map(referenced_classes).collect()
        }
        Type::Array(key, item) | Type::Iterable(key, item) => {
            let mut out = referenced_classes(key);
            out.extend(referenced_classes(item));
            out
        }
        Type::ConstArray(a) => a.keys.iter().chain(a.values.iter()).flat_map(referenced_classes).collect(),
        _ => vec![],
    }
}
