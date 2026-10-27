//! NodeScopeResolver's statement results, as far as the rules read them: whether a statement list always terminates
//! (`StatementResult::isAlwaysTerminating()`), whether it yields, its break/continue exit points, and the end statements
//! of a function body (`ExecutionEndNode`). Inference retains resolved call termination separately from value
//! types: implicit `never` does not terminate, and a loop condition typed `true` always iterates.

use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use pun_phpstan::types::Tri;

use std::collections::HashMap;

use mago_codex::ttype::union::TUnion;

/// How many loops a `break`/`continue` leaves: `break;` (Bare), `break 3;` (Levels(3)), `break $n;` (Dynamic).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Num {
    Bare,
    Levels(i64),
    Dynamic,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Jump {
    pub is_break: bool,
    pub num: Num,
}

/// An `InternalStatementResult`.
#[derive(Clone, Default, Debug)]
pub struct Flow {
    pub always: bool,
    pub has_yield: bool,
    /// The break and continue exit points (returns and throws only matter through `always`).
    pub jumps: Vec<Jump>,
}

impl Flow {
    fn terminating() -> Flow {
        Flow {
            always: true,
            ..Flow::default()
        }
    }

    /// `filterOutLoopExitPoints()`.
    fn without_loop_exits(mut self) -> Flow {
        if self
            .jumps
            .iter()
            .any(|j| matches!(j.num, Num::Bare | Num::Dynamic | Num::Levels(1)))
        {
            self.always = false;
        }
        self
    }

    /// `getExitPointsByType()`: the jumps of this loop.
    fn this_loop(&self, is_break: bool) -> usize {
        self.jumps
            .iter()
            .filter(|j| j.is_break == is_break && !matches!(j.num, Num::Levels(n) if n != 1))
            .count()
    }

    /// `getExitPointsForOuterLoop()`.
    fn outer_loop(&self) -> Vec<Jump> {
        self.jumps
            .iter()
            .filter_map(|j| match j.num {
                Num::Levels(n) if n > 1 => Some(Jump {
                    is_break: j.is_break,
                    num: if n == 2 { Num::Bare } else { Num::Levels(n - 1) },
                }),
                _ => None,
            })
            .collect()
    }
}

/// The statements php-parser has for a body: a block's statements, or the one statement.
pub fn body<'s, 'arena>(s: &'s Statement<'arena>) -> Vec<&'s Statement<'arena>> {
    match s {
        Statement::Block(b) => list(b.statements.as_slice()),
        s => list(std::slice::from_ref(s)),
    }
}

/// The statements php-parser keeps: no opening or closing tags, no empty statements.
pub fn list<'s, 'arena>(s: &'s [Statement<'arena>]) -> Vec<&'s Statement<'arena>> {
    s.iter()
        .filter(|s| !matches!(s, Statement::OpeningTag(_) | Statement::ClosingTag(_) | Statement::Noop(_)))
        .collect()
}

/// The start of the node an end statement reports at.
pub struct End {
    pub at: Span,
    pub flow: Flow,
}

/// Statement results over Mago's types of the analysed file (none when only the syntax is known).
pub struct Results<'t> {
    pub types: &'t HashMap<Span, TUnion>,
    pub call_terminations: Option<&'t HashMap<(u32, u32), bool>>,
    /// the analysed file's contents, for the comments php-parser turns into a trailing `Nop` statement
    pub contents: &'t [u8],
    /// the codebase, for an enum's cases (none when only the syntax is known)
    pub codebase: Option<&'t mago_codex::metadata::CodebaseMetadata>,
}

/// The `Nop` php-parser appends to a statement list when a comment comes right before its closing brace, on the
/// comment's last line. `close` is the brace.
pub fn nop(contents: &[u8], close: Span) -> Option<Span> {
    let mut p = close.start.offset as usize;
    while p > 0 && contents[p - 1].is_ascii_whitespace() {
        p -= 1;
    }
    if !contents[..p].ends_with(b"*/") && !line_comment(contents, p) {
        return None;
    }
    let pos = mago_span::Position::new(p as u32 - 1);
    Some(Span::new(close.file_id, pos, pos))
}

/// The line ending at `p` ends with a `//` or `#` comment.
// ponytail: the line is scanned for quotes only; a line inside a heredoc or multi-line string is not told apart
fn line_comment(contents: &[u8], p: usize) -> bool {
    let start = contents[..p].iter().rposition(|&b| b == b'\n').map_or(0, |i| i + 1);
    let line = &contents[start..p];
    let mut quote = None;
    let mut i = 0;
    while i < line.len() {
        let b = line[i];
        match quote {
            Some(_) if b == b'\\' => i += 1,
            Some(q) if b == q => quote = None,
            Some(_) => {}
            None if b == b'\'' || b == b'"' => quote = Some(b),
            None if line[i..].starts_with(b"//") || (b == b'#' && line.get(i + 1) != Some(&b'[')) => return true,
            None => {}
        }
        i += 1;
    }
    false
}

impl<'t> Results<'t> {
    fn type_of(&self, e: &Expression) -> Option<&'t TUnion> {
        self.types.get(&e.span())
    }

    fn call_terminates(&self, e: &Expression) -> bool {
        let span = e.span();
        self.call_terminations
            .and_then(|calls| calls.get(&(span.start.offset, span.end.offset)))
            .copied()
            .unwrap_or(false)
    }

    /// `processStmtNodes()` of a function-like body: its result and, when the last statement is reached, the
    /// ExecutionEndNodes (`parent` is the function-like, reported for an empty body).
    pub fn execution_ends<'arena>(
        &self,
        stmts: &[&Statement<'arena>],
        parent: Span,
        close: Option<Span>,
    ) -> (Flow, Vec<End>) {
        let nop = close.and_then(|c| nop(self.contents, c));
        let mut flow = Flow::default();
        let mut ends = Vec::new();
        for (i, s) in stmts.iter().enumerate() {
            if flow.always && !matches!(s, Statement::Function(_) | Statement::Class(_) | Statement::Interface(_) | Statement::Trait(_) | Statement::Enum(_) | Statement::Label(_)) {
                continue;
            }
            let (result, s_ends) = self.statement(s);
            flow.has_yield |= result.has_yield;
            if nop.is_none() && i == stmts.len() - 1 {
                if s_ends.is_empty() {
                    ends.push(End {
                        at: s.span(),
                        flow: Flow {
                            has_yield: flow.has_yield,
                            ..result.clone()
                        },
                    });
                } else {
                    ends.extend(s_ends.into_iter().map(|e| End {
                        at: e.at,
                        flow: Flow {
                            has_yield: flow.has_yield,
                            ..e.flow
                        },
                    }));
                }
            }
            flow.jumps.extend(result.jumps);
            flow.always |= result.always;
        }
        if let Some(at) = nop {
            if !flow.always {
                ends.push(End {
                    at,
                    flow: Flow {
                        always: false,
                        has_yield: flow.has_yield,
                        jumps: Vec::new(),
                    },
                });
            }
        } else if stmts.is_empty() {
            ends.push(End {
                at: parent,
                flow: flow.clone(),
            });
        }
        (flow, ends)
    }

    /// `processStmtNodes()` of a statement list.
    pub fn statements<'arena>(&self, stmts: &[&Statement<'arena>]) -> Flow {
        let mut flow = Flow::default();
        for s in stmts {
            if flow.always && !matches!(s, Statement::Function(_) | Statement::Class(_) | Statement::Interface(_) | Statement::Trait(_) | Statement::Enum(_) | Statement::Label(_)) {
                continue;
            }
            let result = self.statement(s).0;
            flow.has_yield |= result.has_yield;
            flow.jumps.extend(result.jumps);
            flow.always |= result.always;
        }
        flow
    }

    /// `processStmtNode()`: the statement's result and its end statements (an `if`'s branches).
    fn statement<'arena>(&self, s: &Statement<'arena>) -> (Flow, Vec<End>) {
        let flow = match s {
            Statement::Return(r) => Flow {
                always: true,
                has_yield: r.value.is_some_and(|v| yields(v)),
                jumps: Vec::new(),
            },
            Statement::Break(b) => jump(true, b.level),
            Statement::Continue(k) => jump(false, k.level),
            Statement::Goto(_) => Flow::terminating(),
            Statement::Expression(e) => Flow {
                always: self.early_terminating(e.expression) || self.terminates(e.expression),
                has_yield: yields(e.expression),
                jumps: Vec::new(),
            },
            Statement::Echo(e) => Flow {
                always: e.values.iter().any(|v| self.terminates(v)),
                has_yield: e.values.iter().any(|v| yields(v)),
                jumps: Vec::new(),
            },
            Statement::Block(b) => self.statements(&list(b.statements.as_slice())),
            Statement::Declare(d) => match &d.body {
                DeclareBody::Statement(s) => self.statements(&body(s)),
                DeclareBody::ColonDelimited(b) => self.statements(&list(b.statements.as_slice())),
            },
            Statement::If(i) => return self.r#if(i),
            Statement::Foreach(f) => self.foreach(f),
            Statement::While(w) => self.r#while(w),
            Statement::DoWhile(d) => self.do_while(d),
            Statement::For(f) => self.r#for(f),
            Statement::Switch(s) => self.switch(s),
            Statement::Try(t) => self.r#try(t),
            _ => Flow::default(),
        };
        (flow, Vec::new())
    }

    /// The cases cover every value of the subject (its type narrowed by them is `never`): enum cases and literal
    /// scalars are removed one by one.
    fn exhaustive<'arena>(&self, s: &Switch<'arena>, cases: &[SwitchCase<'arena>]) -> bool {
        let Some(codebase) = self.codebase else {
            return false;
        };
        let Some(subject) = self.type_of(s.expression) else {
            return false;
        };
        let mut left = Vec::new();
        for a in subject.types.iter() {
            match values(codebase, a) {
                Some(v) => left.extend(v),
                None => return false,
            }
        }
        for case in cases {
            if let SwitchCase::Expression(c) = case
                && let Some(t) = self.type_of(c.expression)
                && let [a] = t.types.as_ref()
                && let Some(v) = values(codebase, a)
                && let [v] = v.as_slice()
            {
                left.retain(|l| l != v);
            }
        }
        left.is_empty()
    }

    /// `toBoolean()->isTrue()` of a condition, by Mago's type of it.
    pub fn condition<'arena>(&self, e: &Expression<'arena>) -> Tri {
        if let Some(t) = constant_truth(e) {
            return Tri::from(t);
        }
        match self.type_of(e) {
            Some(t) if t.is_always_truthy() => Tri::Yes,
            Some(t) if t.is_always_falsy() => Tri::No,
            _ => Tri::Maybe,
        }
    }

    fn r#if<'arena>(&self, i: &If<'arena>) -> (Flow, Vec<End>) {
        // (condition, keyword, statements)
        let close = |s: &Statement| match s {
            Statement::Block(b) => nop(self.contents, b.right_brace),
            _ => None,
        };
        let mut branches: Vec<(Option<&Expression<'arena>>, Span, Vec<&Statement<'arena>>, Option<Span>)> = Vec::new();
        match &i.body {
            IfBody::Statement(b) => {
                branches.push((Some(i.condition), i.r#if.span(), body(b.statement), close(b.statement)));
                for e in b.else_if_clauses.iter() {
                    branches.push((Some(e.condition), e.elseif.span(), body(e.statement), close(e.statement)));
                }
                if let Some(e) = &b.else_clause {
                    branches.push((None, e.r#else.span(), body(e.statement), close(e.statement)));
                }
            }
            IfBody::ColonDelimited(b) => {
                branches.push((Some(i.condition), i.r#if.span(), list(b.statements.as_slice()), None));
                for e in b.else_if_clauses.iter() {
                    branches.push((Some(e.condition), e.elseif.span(), list(e.statements.as_slice()), None));
                }
                if let Some(e) = &b.else_clause {
                    branches.push((None, e.r#else.span(), list(e.statements.as_slice()), None));
                }
            }
        }
        let mut flow = Flow {
            always: true,
            has_yield: yields(i.condition),
            jumps: Vec::new(),
        };
        let mut ends = Vec::new();
        let mut decided = false; // `$ifAlwaysTrue || $lastElseIfConditionIsTrue`
        let mut has_else = false;
        for (n, (cond, keyword, stmts, nop)) in branches.into_iter().enumerate() {
            let truth = cond.map(|c| self.condition(c));
            let branch = self.statements(&stmts);
            let considered = match truth {
                // the `if` branch: unless its condition is always false
                Some(t) if n == 0 => !t.no(),
                Some(t) => !decided && !t.no(),
                None => {
                    has_else = true;
                    !decided
                }
            };
            if considered {
                flow.jumps.extend(branch.jumps.iter().copied());
                flow.always = if n == 0 { branch.always } else { flow.always && branch.always };
                flow.has_yield |= branch.has_yield;
                ends.push(End {
                    at: nop.unwrap_or_else(|| stmts.last().map_or(keyword, |s| s.span())),
                    flow: branch,
                });
            }
            if truth.is_some_and(Tri::yes) {
                decided = true;
            }
        }
        if !has_else && !decided {
            flow.always = false;
            ends.push(End {
                at: i.span(),
                flow: flow.clone(),
            });
        }
        (flow, ends)
    }

    fn foreach<'arena>(&self, f: &Foreach<'arena>) -> Flow {
        let stmts = match &f.body {
            ForeachBody::Statement(s) => body(s),
            ForeachBody::ColonDelimited(b) => list(b.statements.as_slice()),
        };
        let inner = self.statements(&stmts).without_loop_exits();
        let at_least_once = match self.type_of(f.expression) {
            Some(t) if !t.types.is_empty() => t.types.iter().all(non_empty_array),
            _ => matches!(unparen(f.expression), Expression::Array(a) if !a.elements.is_empty())
                || matches!(unparen(f.expression), Expression::LegacyArray(a) if !a.elements.is_empty()),
        };
        Flow {
            always: at_least_once && inner.always,
            has_yield: inner.has_yield || yields(f.expression),
            jumps: inner.outer_loop(),
        }
    }

    fn r#while<'arena>(&self, w: &While<'arena>) -> Flow {
        let stmts = match &w.body {
            WhileBody::Statement(s) => body(s),
            WhileBody::ColonDelimited(b) => list(b.statements.as_slice()),
        };
        let inner = self.statements(&stmts).without_loop_exits();
        let truth = self.condition(w.condition);
        let always = if truth.yes() {
            inner.this_loop(true) == 0
        } else {
            false
        };
        Flow {
            always,
            has_yield: inner.has_yield || yields(w.condition),
            jumps: inner.outer_loop(),
        }
    }

    fn do_while<'arena>(&self, d: &DoWhile<'arena>) -> Flow {
        let inner = self.statements(&body(d.statement)).without_loop_exits();
        let always = if self.condition(d.condition).yes() {
            inner.this_loop(true) == 0
        } else {
            inner.always
        };
        Flow {
            always,
            has_yield: inner.has_yield || yields(d.condition),
            jumps: inner.outer_loop(),
        }
    }

    fn r#for<'arena>(&self, f: &For<'arena>) -> Flow {
        let stmts = match &f.body {
            ForBody::Statement(s) => body(s),
            ForBody::ColonDelimited(b) => list(b.statements.as_slice()),
        };
        let inner = self.statements(&stmts).without_loop_exits();
        let truth = f.conditions.iter().last().map_or(Tri::Yes, |c| self.condition(c));
        let always = if truth.yes() {
            inner.this_loop(true) == 0
        } else {
            false
        };
        let has_yield = f
            .initializations
            .iter()
            .chain(f.conditions.iter())
            .chain(f.increments.iter())
            .any(|e| yields(e));
        Flow {
            always,
            has_yield: inner.has_yield || has_yield,
            jumps: inner.outer_loop(),
        }
    }

    fn switch<'arena>(&self, s: &Switch<'arena>) -> Flow {
        let cases = match &s.body {
            SwitchBody::BraceDelimited(b) => b.cases.as_slice(),
            SwitchBody::ColonDelimited(b) => b.cases.as_slice(),
        };
        let mut always = true;
        let mut has_default = false;
        let mut has_yield = yields(s.expression);
        let mut jumps = Vec::new();
        let mut last: Option<(bool, bool)> = None; // (branch always, final always) of a falling-through branch
        for case in cases {
            let stmts = match case {
                SwitchCase::Expression(c) => {
                    has_yield |= yields(c.expression);
                    list(c.statements.as_slice())
                }
                SwitchCase::Default(d) => {
                    has_default = true;
                    list(d.statements.as_slice())
                }
            };
            let branch = self.statements(&stmts);
            let fin = branch.clone().without_loop_exits();
            has_yield |= fin.has_yield;
            if branch.this_loop(true) > 0 {
                always = false;
            }
            jumps.extend(fin.outer_loop());
            if branch.always {
                always = always && fin.always;
                last = None;
            } else {
                last = Some((branch.always, fin.always));
            }
        }
        if !has_default && !self.exhaustive(s, cases) {
            always = false;
        }
        if let Some((_, fin)) = last {
            always = always && fin;
        }
        Flow {
            always,
            has_yield,
            jumps,
        }
    }

    fn r#try<'arena>(&self, t: &Try<'arena>) -> Flow {
        let inner = self.statements(&list(t.block.statements.as_slice()));
        let mut always = inner.always;
        let mut has_yield = inner.has_yield;
        let mut jumps = inner.jumps;
        for c in t.catch_clauses.iter() {
            let branch = self.statements(&list(c.block.statements.as_slice()));
            always = always && branch.always;
            has_yield |= branch.has_yield;
            jumps.extend(branch.jumps);
        }
        if let Some(f) = &t.finally_clause {
            let finally = self.statements(&list(f.block.statements.as_slice()));
            always = always || finally.always;
            has_yield |= finally.has_yield;
            jumps.extend(finally.jumps);
        }
        Flow {
            always,
            has_yield,
            jumps,
        }
    }

    /// An expression statement also terminates when its value is explicit never.
    fn early_terminating<'arena>(&self, e: &Expression<'arena>) -> bool {
        matches!(
            e,
            Expression::Throw(_) | Expression::Construct(Construct::Exit(_) | Construct::Die(_))
        ) || self.type_of(e).is_some_and(|t| t.is_explicit_never())
    }

    /// `ExpressionResult::isAlwaysTerminating()`: the expression always ends the execution where it is evaluated.
    pub fn terminates<'arena>(&self, e: &Expression<'arena>) -> bool {
        match e {
            Expression::Throw(_) | Expression::Construct(Construct::Exit(_) | Construct::Die(_)) => true,
            Expression::Parenthesized(p) => self.terminates(p.expression),
            Expression::Binary(b)
                if matches!(
                    b.operator,
                    BinaryOperator::And(_)
                        | BinaryOperator::Or(_)
                        | BinaryOperator::LowAnd(_)
                        | BinaryOperator::LowOr(_)
                        | BinaryOperator::NullCoalesce(_)
                ) =>
            {
                self.terminates(b.lhs)
            }
            Expression::Conditional(c) => self.terminates(c.condition),
            Expression::Match(m) => self.terminates(m.expression),
            Expression::Assignment(a) if matches!(a.operator, AssignmentOperator::Coalesce(_)) => false,
            Expression::Closure(_)
            | Expression::ArrowFunction(_)
            | Expression::AnonymousClass(_)
            | Expression::Call(Call::NullSafeMethod(_)) => false,
            Expression::Call(_) => self.call_terminates(e) || self.children_terminate(Node::Expression(e)),
            _ => self.children_terminate(Node::Expression(e)),
        }
    }

    /// (a nested function or class is an expression that [`Self::terminates`] stops at, so no statement is reached)
    fn children_terminate<'arena>(&self, n: Node<'_, 'arena>) -> bool {
        n.children().into_iter().any(|c| match c {
            Node::Expression(e) => self.terminates(e),
            c => self.children_terminate(c),
        })
    }
}

fn jump(is_break: bool, level: Option<&Expression>) -> Flow {
    let num = match level.map(unparen) {
        None => Num::Bare,
        Some(Expression::Literal(Literal::Integer(i))) => i.value.map_or(Num::Dynamic, |v| Num::Levels(v as i64)),
        Some(_) => Num::Dynamic,
    };
    Flow {
        always: true,
        has_yield: false,
        jumps: vec![Jump { is_break, num }],
    }
}

pub fn unparen<'s, 'arena>(mut e: &'s Expression<'arena>) -> &'s Expression<'arena> {
    while let Expression::Parenthesized(p) = e {
        e = p.expression;
    }
    e
}

/// A literal condition's truthiness (`while (true)`, `for (;;1)`).
fn constant_truth(e: &Expression) -> Option<bool> {
    match unparen(e) {
        Expression::Literal(Literal::True(_)) => Some(true),
        Expression::Literal(Literal::False(_) | Literal::Null(_)) => Some(false),
        Expression::Literal(Literal::Integer(i)) => i.value.map(|v| v != 0),
        Expression::ConstantAccess(c) => match c.name.value().to_ascii_lowercase().as_slice() {
            b"true" => Some(true),
            b"false" | b"null" => Some(false),
            _ => None,
        },
        _ => None,
    }
}

/// The single values of a type: enum cases, literal scalars (`None` for any other type).
fn values(codebase: &mago_codex::metadata::CodebaseMetadata, a: &mago_codex::ttype::atomic::TAtomic) -> Option<Vec<String>> {
    use mago_codex::ttype::atomic::TAtomic;
    use mago_codex::ttype::atomic::object::TObject;
    use mago_codex::ttype::atomic::scalar::TScalar;
    use mago_codex::ttype::atomic::scalar::int::TInteger;
    use mago_codex::ttype::atomic::scalar::string::TStringLiteral;
    let cases = |name: &[u8]| {
        let k = codebase.get_class_like(name)?;
        (k.kind == mago_codex::symbol::SymbolKind::Enum).then(|| {
            k.enum_cases
                .keys()
                .map(|c| format!("{}::{}", k.name.to_ascii_lowercase(), c))
                .collect()
        })
    };
    Some(match a {
        TAtomic::Object(TObject::Enum(e)) => match &e.case {
            Some(c) => vec![format!("{}::{}", e.name.to_ascii_lowercase(), c)],
            None => cases(e.name.as_bytes())?,
        },
        TAtomic::Object(TObject::Named(n)) => cases(n.name.as_bytes())?,
        TAtomic::Scalar(TScalar::String(s)) => match &s.literal {
            Some(TStringLiteral::Value(v)) => vec![format!("s:{v}")],
            _ => return None,
        },
        TAtomic::Scalar(TScalar::Integer(TInteger::Literal(i))) => vec![format!("i:{i}")],
        TAtomic::Scalar(TScalar::Bool(b)) => match b.value {
            Some(v) => vec![format!("b:{v}")],
            None => vec!["b:true".to_string(), "b:false".to_string()],
        },
        TAtomic::Null => vec!["null".to_string()],
        _ => return None,
    })
}

fn non_empty_array(a: &mago_codex::ttype::atomic::TAtomic) -> bool {
    use mago_codex::ttype::atomic::TAtomic;
    use mago_codex::ttype::atomic::array::TArray;
    match a {
        TAtomic::Array(TArray::List(l)) => l.non_empty || l.known_elements.as_ref().is_some_and(|k| k.values().any(|(optional, _)| !optional)),
        TAtomic::Array(TArray::Keyed(k)) => k.non_empty || k.known_items.as_ref().is_some_and(|k| k.values().any(|(optional, _)| !optional)),
        _ => false,
    }
}

/// The expression contains a `yield` (outside nested functions and classes).
pub fn yields(e: &Expression) -> bool {
    match e {
        Expression::Yield(_) => true,
        Expression::Closure(_) | Expression::ArrowFunction(_) | Expression::AnonymousClass(_) => false,
        e => node_yields(Node::Expression(e)),
    }
}

/// (a nested function or class is an expression that [`yields`] stops at)
fn node_yields(n: Node) -> bool {
    n.children().into_iter().any(|c| match c {
        Node::Expression(e) => yields(e),
        c => node_yields(c),
    })
}
