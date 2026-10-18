//! `lint/classes/no-null-property-init`, ported from Rector 2.6.1's `RemoveNullPropertyInitializationRector`
//! (rules/DeadCode/Rector/PropertyProperty).
//!
//! `private $a = null;` becomes `private $a;`: an untyped property is `null` until it is set. A typed property
//! keeps `= null`, since without it the property is uninitialized. The default must be the constant `null` in any
//! case (`NULL`, `\null`).

use mago_syntax::cst::{Node, NodeKind, Property, PropertyItem};
use pun_problem::{Edit, Fix, Safety, Span};

use super::php_parser::is_constant;
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

pub struct NoNullPropertyInit;

impl Rule for NoNullPropertyInit {
    fn id(&self) -> &'static str {
        "lint/classes/no-null-property-init"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Property])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        _ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Property(prop) = node else { return };
        let (hint, items): (_, Vec<&PropertyItem>) = match prop {
            Property::Plain(p) => (&p.hint, p.items.iter().collect()),
            Property::Hooked(p) => (&p.hint, vec![&p.item]),
        };
        if hint.is_some() {
            return;
        }
        for item in items {
            let PropertyItem::Concrete(item) = item else {
                continue;
            };
            if !is_constant(item.value, "null") {
                continue;
            }
            let end = node_span(item.value).end;
            out.push(Report {
                span: node_span(item),
                message: "An untyped property is null already; remove `= null`.".to_string(),
                help: None,
                fix: Some(Fix {
                    safety: Safety::Safe,
                    edits: vec![Edit::new(Span::new(node_span(&item.variable).end, end), "")],
                }),
            });
        }
    }
}
