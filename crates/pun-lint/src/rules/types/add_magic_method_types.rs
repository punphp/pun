//! `lint/types/add-magic-method-return-type`, ported from Rector 2.6.1's `KnownMagicClassMethodTypeRector`
//! (rules/TypeDeclaration/Rector/ClassMethod): the parameters of `__call`, `__callStatic`, `__get`, `__set`, `__isset`
//! and `__unset` get their known types (`string` for the name, `array` for `__call`'s arguments, `mixed` for
//! `__set`'s value), and `__get` returns `mixed`, in a class whose parents do not declare the method.

use mago_syntax::cst::{FunctionLikeParameter, Modifier, Node, NodeKind};
use pun_problem::{Edit, Fix, Safety, Span};

use super::class_scope;
use super::facts::facts;
use super::never::has_parent_class_method;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct AddMagicMethodTypes;

/// `PhpVersionFeature::MIXED_TYPE`.
const MIXED_TYPE: u32 = 80000;

impl Rule for AddMagicMethodTypes {
    fn id(&self) -> &'static str {
        "lint/types/add-magic-method-return-type"
    }

    fn min_php_version(&self) -> u32 {
        super::SCALAR_TYPES
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
        let Node::Method(method) = node else {
            return;
        };
        // Rector visits classes, anonymous ones too, not traits, interfaces or enums.
        let in_class = ancestors.iter().rev().find_map(|a| match a {
            Node::Class(_) | Node::AnonymousClass(_) => Some(true),
            Node::Interface(_) | Node::Trait(_) | Node::Enum(_) => Some(false),
            _ => None,
        });
        if in_class != Some(true) {
            return;
        }
        let name = method.name.value.to_ascii_lowercase();
        let (first, second): (Option<&str>, Option<&str>) = match name.as_slice() {
            b"__call" | b"__callstatic" => (Some("string"), Some("array")),
            b"__set" => (
                Some("string"),
                (ctx.php_version >= MIXED_TYPE).then_some("mixed"),
            ),
            b"__get" | b"__isset" | b"__unset" => (Some("string"), None),
            _ => return,
        };
        let private = method
            .modifiers
            .iter()
            .any(|m| matches!(m, Modifier::Private(_)));
        if !private
            && class_scope(ancestors)
                .is_some_and(|scope| has_parent_class_method(&facts(ctx), &scope, &name))
        {
            return;
        }
        let parameters: Vec<&FunctionLikeParameter<'_>> =
            method.parameter_list.parameters.iter().collect();
        let mut edits = Vec::new();
        for (index, r#type) in [(0, first), (1, second)] {
            if let (Some(r#type), Some(parameter)) = (r#type, parameters.get(index))
                && parameter.hint.is_none()
            {
                let at = parameter
                    .ampersand
                    .or(parameter.ellipsis)
                    .map_or(node_span(&parameter.variable).start, |s| {
                        s.start.offset as usize
                    });
                edits.push(Edit::new(Span::new(at, at), format!("{type} ")));
            }
        }
        if name == b"__get" && ctx.php_version >= MIXED_TYPE && method.return_type_hint.is_none() {
            let at = node_span(&method.parameter_list).end;
            edits.push(Edit::new(Span::new(at, at), ": mixed".to_string()));
        }
        if edits.is_empty() {
            return;
        }
        out.push(Report {
            span: node_span(&method.name),
            message: format!(
                "Add the known types of `{}`.",
                String::from_utf8_lossy(method.name.value)
            ),
            help: None,
            fix: Some(Fix {
                safety: Safety::Unsafe,
                edits,
            }),
        });
    }
}
