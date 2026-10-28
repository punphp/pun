//! What Rector 2.6.1's docblock type rules (rules/TypeDeclarationDocblocks and the docblock rules of
//! rules/TypeDeclaration) share: a function or method as a `ClassMethod|Function_`, `ReturnNodeFinder`, and the
//! report whose fix writes the changed doc comment through [`crate::rules::phpdoc_writer`].

use mago_syntax::cst::{
    Access, Assignment, AssignmentOperator, Block, ClassLikeMember, ClassLikeMemberSelector, Expression,
    FunctionLikeParameterList, FunctionLikeReturnTypeHint, Hint, Method, MethodBody, Node, Property, PropertyItem,
    Return, Sequence, Statement, Variable,
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

/// The report of a rule that changed `info`, the doc comment of `target`; `None` when the printed doc leaves the
/// source as it is (Rector rewrites a not useful `mixed[]` tag with the same `mixed[]`).
pub(crate) fn doc_report(
    span: Span,
    message: &str,
    source: &str,
    target: &DocTarget,
    info: &PhpDocInfo,
) -> Option<Report> {
    report(span, message, source, vec![target.edit(source, &info.print())])
}

/// The report of a rule that changed the doc comments `changes` (a target and its changed doc each); `None` when
/// none of them changes the source.
pub(crate) fn docs_report(
    span: Span,
    message: &str,
    source: &str,
    changes: &[(DocTarget, PhpDocInfo)],
) -> Option<Report> {
    let edits = changes
        .iter()
        .map(|(target, info)| target.edit(source, &info.print()))
        .collect();
    report(span, message, source, edits)
}

/// The report with the `edits` that change the source, if any does.
fn report(span: Span, message: &str, source: &str, mut edits: Vec<Edit>) -> Option<Report> {
    edits.retain(|edit| source[edit.span.start..edit.span.end] != edit.text);
    (!edits.is_empty()).then(|| Report {
        span,
        message: message.to_string(),
        help: None,
        fix: Some(Fix {
            safety: Safety::Unsafe,
            edits,
        }),
    })
}

/// `Type::getReferencedClasses()`.
pub(crate) fn referenced_classes(t: &Type) -> Vec<String> {
    match t {
        Type::Object(name, args) => {
            let mut out = vec![name.clone()];
            out.extend(args.iter().flat_map(referenced_classes));
            out
        }
        Type::EnumCase(name, _) | Type::This(name) => vec![name.clone()],
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

/// A class's members (`Class_`, an anonymous class too), whether it is anonymous, and where its report points (its
/// name, or `class` of an anonymous class).
pub(crate) fn class_members<'ast, 'arena>(
    node: Node<'ast, 'arena>,
) -> Option<(&'ast Sequence<'arena, ClassLikeMember<'arena>>, bool, Span)> {
    match node {
        Node::Class(c) => Some((&c.members, false, node_span(&c.name))),
        Node::AnonymousClass(c) => Some((&c.members, true, node_span(&c.class))),
        _ => None,
    }
}

/// `Class_::getMethods()`.
pub(crate) fn methods<'ast, 'arena>(
    members: &'ast Sequence<'arena, ClassLikeMember<'arena>>,
) -> impl Iterator<Item = &'ast Method<'arena>> {
    members.iter().filter_map(|m| match m {
        ClassLikeMember::Method(m) => Some(m),
        _ => None,
    })
}

/// `Class_::getMethod()`: the method of that name, case-insensitively.
pub(crate) fn method<'ast, 'arena>(
    members: &'ast Sequence<'arena, ClassLikeMember<'arena>>,
    name: &str,
) -> Option<&'ast Method<'arena>> {
    methods(members).find(|m| m.name.value.eq_ignore_ascii_case(name.as_bytes()))
}

/// A `Property` statement as Rector reads it.
pub(crate) struct PropertyView<'ast, 'arena> {
    pub start: usize,
    pub hint: Option<&'ast Hint<'arena>>,
    /// The names of its items, without `$`.
    pub names: Vec<&'arena str>,
    /// The default of its first item.
    pub default: Option<&'ast Expression<'arena>>,
    pub is_private: bool,
}

/// `Class_::getProperties()`.
pub(crate) fn properties<'ast, 'arena>(
    members: &'ast Sequence<'arena, ClassLikeMember<'arena>>,
) -> Vec<PropertyView<'ast, 'arena>> {
    let name = |item: &'ast PropertyItem<'arena>| {
        let v = match item {
            PropertyItem::Abstract(i) => &i.variable,
            PropertyItem::Concrete(i) => &i.variable,
        };
        std::str::from_utf8(&v.name[1..]).unwrap_or("")
    };
    let default = |item: &'ast PropertyItem<'arena>| match item {
        PropertyItem::Concrete(i) => Some(i.value),
        PropertyItem::Abstract(_) => None,
    };
    members
        .iter()
        .filter_map(|m| match m {
            ClassLikeMember::Property(p) => Some(p),
            _ => None,
        })
        .map(|p| {
            let (modifiers, hint, items): (_, _, Vec<&'ast PropertyItem<'arena>>) = match p {
                Property::Plain(p) => (&p.modifiers, p.hint.as_ref(), p.items.iter().collect()),
                Property::Hooked(p) => (&p.modifiers, p.hint.as_ref(), vec![&p.item]),
            };
            PropertyView {
                start: node_span(p).start,
                hint,
                names: items.iter().map(|i| name(i)).collect(),
                default: items.first().and_then(|i| default(i)),
                is_private: modifiers.iter().any(|m| m.is_private()),
            }
        })
        .collect()
}

/// `$this->isName($expr, 'this')` of a direct variable.
pub(crate) fn is_this(expr: &Expression<'_>) -> bool {
    variable_name(expr) == Some("this")
}

/// The name of a direct variable, without `$`.
pub(crate) fn variable_name<'a>(expr: &Expression<'a>) -> Option<&'a str> {
    match expr {
        Expression::Variable(Variable::Direct(v)) => std::str::from_utf8(&v.name[1..]).ok(),
        _ => None,
    }
}

/// `$this->name` of a property fetch on `$this`: the name.
pub(crate) fn this_property<'a>(expr: &Expression<'a>) -> Option<&'a str> {
    match expr {
        Expression::Access(Access::Property(p)) if is_this(p.object) => match &p.property {
            ClassLikeMemberSelector::Identifier(id) => std::str::from_utf8(id.value).ok(),
            _ => None,
        },
        _ => None,
    }
}

/// The statements of a method's body (`null` for an abstract method).
pub(crate) fn method_statements<'ast, 'arena>(m: &'ast Method<'arena>) -> Option<Vec<&'ast Statement<'arena>>> {
    match &m.body {
        MethodBody::Concrete(b) => Some(b.statements.iter().collect()),
        MethodBody::Abstract(_) => None,
    }
}

/// The property `$this->x` the sole `return` statement of a method returns.
pub(crate) fn returned_this_property<'arena>(m: &Method<'arena>) -> Option<&'arena str> {
    match method_statements(m)?.as_slice() {
        [Statement::Return(r)] => this_property(r.value?),
        _ => None,
    }
}

/// `BetterNodeFinder::findInstancesOfScoped()` over `node`'s children: each node `found` takes, not looking into
/// classes, functions and closures (methods and arrow functions are looked into).
pub(crate) fn find_scoped<'ast, 'arena>(node: Node<'ast, 'arena>, found: &mut dyn FnMut(Node<'ast, 'arena>)) {
    node.visit_children(|child| {
        found(child);
        match child {
            Node::Function(_) | Node::Closure(_) | Node::Class(_) => {}
            Node::AnonymousClass(class) => {
                if let Some(args) = &class.argument_list {
                    find_scoped(Node::PartialArgumentList(args), found);
                }
            }
            _ => find_scoped(child, found),
        }
    });
}

/// The simple assignments (`=`) under `node`, scoped.
pub(crate) fn assignments<'ast, 'arena>(node: Node<'ast, 'arena>) -> Vec<&'ast Assignment<'arena>> {
    let mut out = Vec::new();
    find_scoped(node, &mut |n| {
        if let Node::Assignment(a) = n
            && matches!(a.operator, AssignmentOperator::Assign(_))
        {
            out.push(a);
        }
    });
    out
}
