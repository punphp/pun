//! `lint/pest/prefer-chained-expect`, ported from pest-plugin-rector's `ChainExpectCallsRector`: consecutive
//! `expect()` statements become one chain, the matchers of the same value added to it on lines of their own and
//! another value joined with `->and($value)` (unless `merge-different-variables` is off).

use mago_syntax::cst::{Expression, MethodCall, Node, NodeKind, PropertyAccess};
use pun_problem::Safety;

use super::{
    Item, Link, STATEMENT_LISTS, chain_links, expect_argument, has_comments, identifier,
    is_expect_chain, kept_text, method_call, print_args, print_member, report, rewrite_list,
    statement_expression, statement_list,
};
use crate::syntax::{node_span, same_expression, unparenthesize};
use crate::{Context, Report, Rule};

/// `merge_different_variables`, on by default: `separate_variables` is its opposite.
#[derive(Default)]
pub struct PreferChainedExpect {
    separate_variables: bool,
}

/// An expression of the chain being built: an old method call kept whole, or a node Rector makes.
#[derive(Clone)]
enum Chain<'a> {
    Old(&'a MethodCall<'a>),
    /// `new MethodCall($var, $call->name, $call->args)`.
    Call {
        var: Box<Chain<'a>>,
        call: &'a MethodCall<'a>,
        newline: bool,
    },
    /// `new MethodCall($var, 'and', [new Arg($value)])`.
    And {
        var: Box<Chain<'a>>,
        value: &'a Expression<'a>,
        newline: bool,
    },
    /// `new PropertyFetch($var, $fetch->name)`.
    Property {
        var: Box<Chain<'a>>,
        fetch: &'a PropertyAccess<'a>,
    },
}

/// A statement of the list: its old index, and the chain it holds now when Rector changed it.
struct Stmt<'a> {
    index: usize,
    call: &'a MethodCall<'a>,
    chain: Option<Chain<'a>>,
}

impl Rule for PreferChainedExpect {
    fn id(&self) -> &'static str {
        "lint/pest/prefer-chained-expect"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(STATEMENT_LISTS)
    }

    /// `configure`: `merge-different-variables` (the plugin's `merge_different_variables`), a boolean.
    fn configure(&mut self, options: &serde_json::Value) -> Result<(), String> {
        match options {
            serde_json::Value::Null => Ok(()),
            serde_json::Value::Object(map) => {
                for (key, value) in map {
                    match (key.as_str(), value.as_bool()) {
                        ("merge-different-variables", Some(merge)) => {
                            self.separate_variables = !merge;
                        }
                        _ => {
                            return Err(format!(
                                "unknown option `{key}` or a value of the wrong type"
                            ));
                        }
                    }
                }
                Ok(())
            }
            _ => Err("options must be a table".into()),
        }
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let list = statement_list(ctx, node);
        // php-parser's statements, with what refactor() asks of them: an expression statement whose expression is a
        // method call. The others only stop a run.
        let mut stmts: Vec<Option<Stmt<'_>>> = list
            .stmts
            .iter()
            .enumerate()
            .map(|(index, s)| {
                let call = statement_expression(s).and_then(method_call)?;
                Some(Stmt {
                    index,
                    call,
                    chain: None,
                })
            })
            .collect();
        // Where each entry of `stmts` stands in the old list, for the statements that are not method calls.
        let mut order: Vec<usize> = (0..list.stmts.len()).collect();
        let starts: Vec<usize> = list.stmts.iter().map(|s| node_span(*s).start).collect();
        let commented = |s: &Stmt<'_>| has_comments(ctx, starts[s.index]);
        let merge = !self.separate_variables;
        let mut changed = false;
        loop {
            let mut changed_in_pass = false;
            for key in 0..stmts.len() {
                let (Some(Some(current)), Some(Some(next))) = (stmts.get(key), stmts.get(key + 1))
                else {
                    continue;
                };
                if !is_expect_chain(current.call) || !is_expect_chain(next.call) {
                    continue;
                }
                let (Some(first), Some(next_arg)) =
                    (expect_argument(current.call), expect_argument(next.call))
                else {
                    continue;
                };
                if commented(current) || commented(next) {
                    continue;
                }
                if same_expression(ctx.source, first, next_arg) {
                    let has_and = names(current).iter().any(|n| *n == Some(&b"and"[..]));
                    if has_and && !merge {
                        continue;
                    }
                    if !has_and {
                        // mergeSameVariable
                        let second = stmts.remove(key + 1).unwrap();
                        order.remove(key + 1);
                        let current = stmts[key].as_mut().unwrap();
                        let base = current.chain.take().unwrap_or(Chain::Old(current.call));
                        let mut chain = rebuild(base, &chain_links(second.call));
                        newlines(&mut chain);
                        current.chain = Some(chain);
                        changed_in_pass = true;
                        break;
                    }
                }
                if merge {
                    merge_different(ctx, &mut stmts, &mut order, key, &commented);
                    changed_in_pass = true;
                    break;
                }
            }
            changed |= changed_in_pass;
            if !changed_in_pass {
                break;
            }
        }
        if !changed {
            return;
        }
        let mut items: Vec<Item<'_>> = Vec::new();
        for (entry, index) in stmts.iter().zip(&order) {
            match entry
                .as_ref()
                .and_then(|s| s.chain.as_ref().map(|c| (s.call, c)))
            {
                Some((call, chain)) => {
                    let span = node_span(list.stmts[*index]);
                    let indent = super::indent_at(ctx, span.start);
                    let expr = node_span(call);
                    let text = format!(
                        "{}{}{}",
                        &ctx.source[span.start..expr.start],
                        print(ctx, chain, indent),
                        &ctx.source[expr.end..span.end]
                    );
                    items.push(Item::Changed(*index, text));
                }
                None => items.push(Item::Old(*index)),
            }
        }
        if list.nop {
            items.push(Item::Old(list.stmts.len()));
        }
        let Some(edits) = rewrite_list(ctx, node, ancestors, &list, &items) else {
            return;
        };
        out.push(report(
            node_span(list.stmts[0]),
            "Chain consecutive `expect()` calls.".to_string(),
            Safety::Safe,
            edits,
        ));
    }
}

/// `mergeDifferentVariableChains`: the statements after `key` on the value of the next one, their methods chained
/// after `->and(value)` on the statement at `key`.
fn merge_different<'a>(
    ctx: &Context<'_>,
    stmts: &mut Vec<Option<Stmt<'a>>>,
    order: &mut Vec<usize>,
    key: usize,
    commented: &dyn Fn(&Stmt<'_>) -> bool,
) {
    let target = expect_argument(stmts[key + 1].as_ref().unwrap().call).unwrap();
    let mut methods = Vec::new();
    while let Some(Some(current)) = stmts.get(key + 1) {
        if commented(current) || !is_expect_chain(current.call) {
            break;
        }
        if !expect_argument(current.call).is_some_and(|a| same_expression(ctx.source, target, a)) {
            break;
        }
        methods.extend(chain_links(current.call));
        stmts.remove(key + 1);
        order.remove(key + 1);
    }
    let current = stmts[key].as_mut().unwrap();
    let base = current.chain.take().unwrap_or(Chain::Old(current.call));
    let and = Chain::And {
        var: Box::new(base),
        value: target,
        newline: false,
    };
    let mut chain = rebuild(and, &methods);
    newlines(&mut chain);
    current.chain = Some(chain);
}

/// `rebuildMethodChain`: `methods` written again on `base`.
fn rebuild<'a>(base: Chain<'a>, methods: &[Link<'a>]) -> Chain<'a> {
    methods.iter().fold(base, |var, link| match link {
        Link::Method(call) => Chain::Call {
            var: Box::new(var),
            call,
            newline: false,
        },
        Link::Property(fetch) => Chain::Property {
            var: Box::new(var),
            fetch,
        },
    })
}

/// The names `collectChainMethods` gives for a statement's chain, as `getName` reads them (none for a dynamic name).
fn names<'a>(statement: &Stmt<'a>) -> Vec<Option<&'a [u8]>> {
    fn walk<'a>(chain: &Chain<'a>, out: &mut Vec<Option<&'a [u8]>>) {
        match chain {
            Chain::Old(call) => out.extend(chain_links(call).iter().map(|l| link_name(l))),
            Chain::Call { var, call, .. } => {
                walk(var, out);
                out.push(identifier(&call.method));
            }
            Chain::And { var, .. } => {
                walk(var, out);
                out.push(Some(b"and"));
            }
            Chain::Property { var, fetch } => {
                walk(var, out);
                out.push(identifier(&fetch.property));
            }
        }
    }
    let mut out = Vec::new();
    match &statement.chain {
        Some(chain) => walk(chain, &mut out),
        None => walk(&Chain::Old(statement.call), &mut out),
    }
    out
}

fn link_name<'a>(link: &Link<'a>) -> Option<&'a [u8]> {
    match link {
        Link::Method(call) => identifier(&call.method),
        Link::Property(fetch) => identifier(&fetch.property),
    }
}

/// `applyNewlineAttributes`: down the chain, each new method call on another method call goes on a line of its own,
/// but not one right after `->and()` (the `->and()` call does) or after a property fetch. Old calls keep their text.
fn newlines(current: &mut Chain<'_>) {
    let (var, flag) = match current {
        Chain::Call { var, newline, .. } | Chain::And { var, newline, .. } => (var, newline),
        Chain::Property { .. } | Chain::Old(_) => return,
    };
    match &mut **var {
        // Old code below keeps its text: its flags show nowhere.
        Chain::Old(call) => {
            if !is_old_and(call) {
                *flag = true;
            }
        }
        Chain::Property { var, .. } => newlines(var),
        Chain::And { newline, var, .. } => {
            *newline = true;
            newlines(var);
        }
        Chain::Call {
            call,
            newline,
            var: below,
        } => {
            if is_old_and(call) {
                *newline = true;
                newlines(below);
            } else {
                *flag = true;
                newlines(var);
            }
        }
    }
}

/// An old method call named `and` (`isName`, any case).
fn is_old_and(call: &MethodCall<'_>) -> bool {
    super::is_name(&call.method, "and")
}

/// The chain printed by Rector's `pExpr_MethodCall` at `indent`: a call with the newline flag on another call goes on
/// a line of its own, one level deeper.
fn print(ctx: &Context<'_>, chain: &Chain<'_>, indent: usize) -> String {
    let call_like = |c: &Chain<'_>| !matches!(c, Chain::Property { .. });
    let link = |var: &Chain<'_>, newline: bool, name: String, args: String| {
        let sep = if newline && call_like(var) {
            format!("\n{}", " ".repeat(indent + 4))
        } else {
            String::new()
        };
        format!("{}{sep}->{name}({args})", print(ctx, var, indent))
    };
    match chain {
        Chain::Old(call) => kept_text(ctx, node_span(*call), indent),
        Chain::Call { var, call, newline } => link(
            var,
            *newline,
            print_member(ctx, &call.method, indent),
            print_args(ctx, &call.argument_list, indent),
        ),
        Chain::And {
            var,
            value,
            newline,
        } => link(
            var,
            *newline,
            "and".to_string(),
            kept_text(ctx, node_span(unparenthesize(value)), indent),
        ),
        Chain::Property { var, fetch } => format!(
            "{}->{}",
            print(ctx, var, indent),
            print_member(ctx, &fetch.property, indent)
        ),
    }
}
