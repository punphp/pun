//! `lint/refactor/rename-cast`, ported from Rector 2.6.1's `RenameCastRector` (rules/Renaming/Rector/Cast).
//!
//! Options: `items = [{ from-cast-expr-class = "PhpParser\\Node\\Expr\\Cast\\Double", from-cast-kind = 3,
//! to-cast-kind = 2 }]` (`RenameCast`, with php-parser's `KIND_*` numbers). A cast of that class whose `kind` (what
//! php-parser's parser reads from the written cast) is the old one is printed anew with the new kind: php-parser's
//! printer writes `(double)`, `(float)` or `(real)` by kind for a `Double`, and always `(int)`, `(bool)` and
//! `(string)` for the other three. The cast is printed anew (`pPrefixOp`): its operand keeps its own text, without the
//! parentheses and comments around it, in new parentheses only where precedence needs them. The casts without a
//! `kind` (`Array_`, `Object_`, `Unset_`, `Void_`) never match.

use mago_span::HasSpan;
use mago_syntax::cst::{Node, NodeKind, UnaryPrefixOperator};
use pun_problem::Edit;

use super::{assert_class, int, items, object, report, string};
use crate::rules::rector_code_quality::{operand, text};
use crate::syntax::node_span;
use crate::{Context, Report, Rule};

/// php-parser's cast classes, by the `Cast` subclass name `RenameCast` takes.
const CASTS: &[&str] = &[
    "Array_", "Bool_", "Double", "Int_", "Object_", "String_", "Unset_", "Void_",
];

#[derive(Default)]
pub struct RenameCast {
    /// (cast class short name, from kind, to kind)
    pub renames: Vec<(String, i64, i64)>,
}

/// The php-parser class and `kind` of a Mago cast operator, `None` for any other prefix operator.
fn class_and_kind(op: &UnaryPrefixOperator<'_>) -> Option<(&'static str, Option<i64>)> {
    Some(match op {
        UnaryPrefixOperator::ArrayCast(..) => ("Array_", None),
        UnaryPrefixOperator::BoolCast(..) => ("Bool_", Some(1)),
        UnaryPrefixOperator::BooleanCast(..) => ("Bool_", Some(2)),
        UnaryPrefixOperator::DoubleCast(..) => ("Double", Some(1)),
        UnaryPrefixOperator::FloatCast(..) => ("Double", Some(2)),
        UnaryPrefixOperator::RealCast(..) => ("Double", Some(3)),
        UnaryPrefixOperator::IntCast(..) => ("Int_", Some(1)),
        UnaryPrefixOperator::IntegerCast(..) => ("Int_", Some(2)),
        UnaryPrefixOperator::ObjectCast(..) => ("Object_", None),
        UnaryPrefixOperator::StringCast(..) => ("String_", Some(1)),
        UnaryPrefixOperator::BinaryCast(..) => ("String_", Some(2)),
        UnaryPrefixOperator::UnsetCast(..) => ("Unset_", None),
        UnaryPrefixOperator::VoidCast(..) => ("Void_", None),
        _ => return None,
    })
}

/// The cast as php-parser's printer writes it.
fn printed(class: &str, kind: i64) -> &'static str {
    match class {
        "Double" => match kind {
            1 => "(double)",
            2 => "(float)",
            // `assert($kind === Cast\Double::KIND_REAL)`
            _ => "(real)",
        },
        "Int_" => "(int)",
        "Bool_" => "(bool)",
        _ => "(string)",
    }
}

impl Rule for RenameCast {
    fn id(&self) -> &'static str {
        "lint/refactor/rename-cast"
    }

    fn configure(&mut self, options: &serde_json::Value) -> Result<(), String> {
        const KEYS: &[&str] = &["from-cast-expr-class", "from-cast-kind", "to-cast-kind"];
        let mut renames = Vec::new();
        for item in items(options)? {
            let map = object(item, KEYS, KEYS)?;
            let class = string(map, "from-cast-expr-class")?;
            assert_class(&class)?;
            let short = class
                .trim_start_matches('\\')
                .strip_prefix("PhpParser\\Node\\Expr\\Cast\\")
                .filter(|s| CASTS.contains(s))
                .ok_or_else(|| {
                    format!("\"{class}\" is not a subclass of \"PhpParser\\Node\\Expr\\Cast\"")
                })?;
            renames.push((
                short.to_string(),
                int(map, "from-cast-kind")?,
                int(map, "to-cast-kind")?,
            ));
        }
        self.renames = renames;
        Ok(())
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::UnaryPrefix])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::UnaryPrefix(cast) = node else {
            return;
        };
        let Some((class, Some(kind))) = class_and_kind(&cast.operator) else {
            return;
        };
        let Some((_, _, to)) = self
            .renames
            .iter()
            .find(|(c, from, _)| c == class && *from == kind)
        else {
            return;
        };
        let new = printed(class, *to);
        // `pPrefixOp`: the operand at the cast's precedence (10).
        let code = format!("{new} {}", operand(ctx, cast.operand, 10, None));
        // A rename to the same cast (`Int_` from one kind to the other prints `(int)` either way) would only respace
        // it, and again in every fix round.
        if code == text(ctx, cast) {
            return;
        }
        out.push(report(
            node_span(cast),
            format!(
                "Rename the cast `{}` to `{new}`.",
                text(ctx, &cast.operator.span())
            ),
            vec![Edit::new(node_span(cast), code)],
        ));
    }
}
