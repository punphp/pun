//! `lint/dead-code/no-duplicate-case-body`, ported from Rector 2.8.4's `RemoveDuplicatedCaseInSwitchRector`
//! (rules/DeadCode/Rector/Switch_).
//!
//! Cases of a `switch` whose bodies are the same code ending the case (a `break` or `return` among its statements)
//! are grouped: the later ones move up under the first, which keeps the body, and their own bodies go. Two bodies
//! that start with `break` only count as the same when their comments are the same too.

use mago_syntax::cst::*;
use pun_problem::{Edit, Fix, Safety, Span};

use super::rector_stmts::{
    List, first_comment, holder_list, line_indent, previous_token_end, printed, reindent,
    same_with_comments,
};
use crate::syntax::{node_span, same_code};
use crate::{Context, Report, Rule};

pub struct NoDuplicateCaseBody;

impl Rule for NoDuplicateCaseBody {
    fn id(&self) -> &'static str {
        "lint/dead-code/no-duplicate-case-body"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Switch])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        _: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Switch(switch) = node else { return };
        let (cases, open) = match &switch.body {
            SwitchBody::BraceDelimited(b) => (b.cases.as_slice(), b.left_brace.end.offset as usize),
            SwitchBody::ColonDelimited(b) => (b.cases.as_slice(), b.colon.end.offset as usize),
        };
        if cases.len() < 2 {
            return;
        }
        let source = ctx.source;
        let lists: Vec<List<'_>> = cases
            .iter()
            .map(|c| holder_list(source, case_node(c), &[]).unwrap())
            .collect();
        // Which case's statements each case holds.
        let mut body: Vec<Option<usize>> = (0..cases.len())
            .map(|i| (!lists[i].is_empty()).then_some(i))
            .collect();
        let mut result: Vec<usize> = Vec::new();
        let mut processed: Vec<usize> = Vec::new();
        let mut changed = false;
        for outer in 0..cases.len() {
            if processed.contains(&outer) {
                continue;
            }
            processed.push(outer);
            let Some(outer_body) = body[outer] else {
                result.push(outer);
                continue;
            };
            let mut without_stmts: Vec<usize> = Vec::new();
            let mut equal_cases: Vec<usize> = Vec::new();
            for (inner, slot) in body.iter_mut().enumerate() {
                if processed.contains(&inner) {
                    continue;
                }
                let Some(inner_body) = *slot else {
                    without_stmts.push(inner);
                    continue;
                };
                if are_equal_and_with_break(ctx, &lists[outer_body], &lists[inner_body]) {
                    for &w in &without_stmts {
                        equal_cases.push(w);
                        processed.push(w);
                    }
                    *slot = None;
                    equal_cases.push(inner);
                    processed.push(inner);
                }
                without_stmts.clear();
            }
            if equal_cases.is_empty() {
                result.push(outer);
                continue;
            }
            changed = true;
            body[*equal_cases.last().unwrap()] = body[outer];
            body[outer] = None;
            result.push(outer);
            result.extend(equal_cases);
        }
        if !changed {
            return;
        }
        // The cases: where each one's comments start, and the text php-parser prints for it without them.
        let mut starts = Vec::new();
        let mut previous = open;
        for case in cases {
            let span = node_span(case);
            let before = previous_token_end(ctx, previous, span.start);
            starts.push(first_comment(source, before, span.start).unwrap_or(span.start));
            previous = span.end;
        }
        let texts: Vec<String> = (0..cases.len())
            .map(|i| {
                let span = node_span(&cases[i]);
                // php-parser's printer keeps the text of a case it moves; it prints anew one whose statements changed.
                if body[i] == (!lists[i].is_empty()).then_some(i) {
                    return source[span.start..span.end].to_string();
                }
                let head_end = match &cases[i] {
                    SwitchCase::Expression(c) => node_span(&c.separator).end,
                    SwitchCase::Default(c) => node_span(&c.separator).end,
                };
                let mut text = source[span.start..head_end].to_string();
                if text.ends_with(';') {
                    text.replace_range(text.len() - 1.., ":");
                }
                if let Some(from) = body[i] {
                    let stmts = printed(ctx, &lists[from]);
                    let indent = line_indent(
                        source,
                        lists[from]
                            .stmts
                            .first()
                            .map_or(stmts.start, |s| node_span(*s).start),
                    );
                    text.push('\n');
                    text.push_str(indent);
                    text.push_str(&reindent(source, stmts, indent, indent));
                }
                text
            })
            .collect();
        let comments = |i: usize| {
            let start = node_span(&cases[i]).start;
            (starts[i] < start).then(|| source[starts[i]..start].trim_end())
        };
        // `pArray` on the diff of the old order and the new one. The first case stays first, so the diff starts with
        // a keep: nothing is added before it or removed at the start.
        let nl = format!("\n{}", line_indent(source, node_span(&cases[0]).start));
        let mut text = String::new();
        let mut pos = open;
        for elem in coalesce_replacements(diff(cases.len(), &result)) {
            match elem {
                Elem::Keep(old, new) | Elem::Replace(old, new) => {
                    let item_start = node_span(&cases[old]).start;
                    let comment_start = starts[old];
                    text.push_str(&source[pos..comment_start]);
                    if old == new {
                        text.push_str(&source[comment_start..item_start]);
                    } else if let Some(c) = comments(new) {
                        text.push_str(c);
                        text.push_str(&nl);
                    }
                    text.push_str(&texts[new]);
                    pos = node_span(&cases[old]).end;
                }
                Elem::Add(new) => {
                    text.push_str(&nl);
                    if let Some(c) = comments(new) {
                        text.push_str(c);
                        text.push_str(&nl);
                    }
                    text.push_str(&texts[new]);
                }
                Elem::Remove(old) => pos = node_span(&cases[old]).end,
            }
        }
        let span = Span::new(open, pos);
        out.push(Report {
            span,
            message: "These cases run the same code; group them.".to_string(),
            help: None,
            fix: Some(Fix {
                safety: Safety::Safe,
                edits: vec![Edit::new(span, text)],
            }),
        });
    }
}

fn case_node<'a>(case: &'a SwitchCase<'a>) -> Node<'a, 'a> {
    match case {
        SwitchCase::Expression(c) => Node::SwitchExpressionCase(c),
        SwitchCase::Default(c) => Node::SwitchDefaultCase(c),
    }
}

/// `areSwitchStmtsEqualsAndWithBreak`.
fn are_equal_and_with_break(ctx: &crate::Context<'_>, current: &List<'_>, next: &List<'_>) -> bool {
    let source = ctx.source;
    let is_break = |l: &List<'_>| matches!(l.stmts.first(), Some(Statement::Break(_)));
    if is_break(current) && is_break(next) {
        return same_with_comments(ctx, current, next);
    }
    if !same_code(source, printed(ctx, current), printed(ctx, next)) {
        return false;
    }
    current
        .stmts
        .iter()
        .any(|s| matches!(s, Statement::Break(_) | Statement::Return(_)))
}

/// An element of php-parser's `Differ` on lists of case numbers.
#[derive(Clone, Copy)]
enum Elem {
    Keep(usize, usize),
    Remove(usize),
    Add(usize),
    Replace(usize, usize),
}

/// php-parser's `Differ::diff` (Myers) from the cases `0..n` to the order `new`.
fn diff(n: usize, new: &[usize]) -> Vec<Elem> {
    let m = new.len();
    let max = (n + m) as isize;
    let at = |k: isize| (k + max + 1) as usize;
    let mut v = vec![0usize; 2 * max as usize + 3];
    let mut trace: Vec<Vec<usize>> = Vec::new();
    let (mut x, mut y) = (0, 0);
    'outer: for d in 0..=max {
        trace.push(v.clone());
        let mut k = -d;
        while k <= d {
            x = if k == -d || (k != d && v[at(k - 1)] < v[at(k + 1)]) {
                v[at(k + 1)]
            } else {
                v[at(k - 1)] + 1
            };
            y = (x as isize - k) as usize;
            while x < n && y < m && x == new[y] {
                x += 1;
                y += 1;
            }
            v[at(k)] = x;
            if x >= n && y >= m {
                break 'outer;
            }
            k += 2;
        }
    }
    let mut out = Vec::new();
    for d in (0..trace.len() as isize).rev() {
        let v = &trace[d as usize];
        let k = x as isize - y as isize;
        let prev_k = if k == -d || (k != d && v[at(k - 1)] < v[at(k + 1)]) {
            k + 1
        } else {
            k - 1
        };
        let prev_x = v[at(prev_k)];
        // At `d` 0 it is -1, below the start.
        let prev_y = prev_x as isize - prev_k;
        while x > prev_x && y as isize > prev_y {
            out.push(Elem::Keep(x - 1, new[y - 1]));
            x -= 1;
            y -= 1;
        }
        if d == 0 {
            break;
        }
        while x > prev_x {
            out.push(Elem::Remove(x - 1));
            x -= 1;
        }
        while y as isize > prev_y {
            out.push(Elem::Add(new[y - 1]));
            y -= 1;
        }
    }
    out.reverse();
    out
}

/// `Differ::coalesceReplacements`: a run of removals followed by as many additions is a run of replacements.
fn coalesce_replacements(diff: Vec<Elem>) -> Vec<Elem> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < diff.len() {
        if !matches!(diff[i], Elem::Remove(_)) {
            out.push(diff[i]);
            i += 1;
            continue;
        }
        let mut j = i;
        while j < diff.len() && matches!(diff[j], Elem::Remove(_)) {
            j += 1;
        }
        let mut k = j;
        while k < diff.len() && matches!(diff[k], Elem::Add(_)) {
            k += 1;
        }
        if j - i == k - j {
            for n in 0..j - i {
                if let (Elem::Remove(old), Elem::Add(new)) = (diff[i + n], diff[j + n]) {
                    out.push(Elem::Replace(old, new));
                }
            }
        } else {
            out.extend_from_slice(&diff[i..k]);
        }
        i = k;
    }
    out
}
