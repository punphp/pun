//! The walk behind `check`: a rule sees every node `Node::visit_children` yields, in the same order, with the same
//! ancestors. The expected walk comes from Mago's `visit_children`, which the engine does not use.

use std::collections::BTreeSet;
use std::path::Path;

use mago_allocator::LocalArena;
use mago_database::file::FileId;
use mago_span::HasSpan;
use mago_syntax::cst::Node;
use pun_lint::syntax::node_span;
use pun_lint::{Context, Linter, Report, Rule};
use pun_problem::Span;

/// Reports every node, with its kind and its ancestors' kinds as the message.
struct Record;

impl Rule for Record {
    fn id(&self) -> &'static str {
        "lint/test/record"
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        _: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let path: Vec<Node> = ancestors.iter().copied().chain([node]).collect();
        out.push(Report {
            span: node_span(&node),
            message: kinds(&path),
            help: None,
            fix: None,
        });
    }
}

fn kinds(path: &[Node<'_, '_>]) -> String {
    let names: Vec<String> = path.iter().map(|n| format!("{:?}", n.kind())).collect();
    names.join(" > ")
}

/// The same reports from a walk over `visit_children`.
fn expected<'ast, 'arena>(
    node: Node<'ast, 'arena>,
    path: &mut Vec<Node<'ast, 'arena>>,
    out: &mut Vec<(Span, String)>,
) {
    path.push(node);
    out.push((node_span(&node.span()), kinds(path)));
    node.visit_children(|child| expected(child, path, out));
    path.pop();
}

#[test]
fn check_walks_the_nodes_of_visit_children_with_their_ancestors() {
    let linter = Linter::new(vec![Box::new(Record)], 80400);
    let mut seen = BTreeSet::new();
    for name in ["every-node.php", "braced-namespace.php", "short-tag.php"] {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/walk")
            .join(name);
        let source = std::fs::read_to_string(&path).unwrap();
        let got: Vec<(Span, String)> = linter
            .check(&path, &source)
            .unwrap()
            .into_iter()
            .map(|p| (p.span, p.message))
            .collect();

        let arena = LocalArena::new();
        let program =
            mago_syntax::parser::parse_file_content(&arena, FileId::zero(), source.as_bytes());
        let mut want = Vec::new();
        expected(Node::Program(program), &mut Vec::new(), &mut want);
        // `check` sorts its problems by start offset, keeping the walk's order at the same offset.
        want.sort_by_key(|(span, _)| span.start);
        assert_eq!(got, want, "{name}");
        seen.extend(
            want.iter()
                .map(|(_, path)| path.rsplit(" > ").next().unwrap().to_string()),
        );
    }
    // The files hold every node type `visit_children` yields: Mago's 227 `Node` variants less the 4 that only a
    // syntax error makes (`Error`, `MissingTerminator` and the two missing selectors) and the 2 `TraitUseAdaptation`
    // cases, whose children it yields directly.
    assert_eq!(seen.len(), 221, "{seen:?}");
}
