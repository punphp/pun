//! VariableFlowBuilder and the expression/statement fragment composition used by its handlers.

use std::rc::Rc;

use foldhash::HashSet;
use mago_allocator::Arena;
use mago_codex::ttype::atomic::{TAtomic, array::TArray, scalar::TScalar};
use mago_codex::ttype::union::TUnion;
use mago_span::{HasSpan, Span};
use mago_syntax::cst::*;
use mago_word::{Word, word};

use crate::artifacts::{AnalysisArtifacts, VariableThrowKind};
use crate::context::Context;
use crate::context::block::BlockContext;

use super::{Flow, Offset, State, ThrowPoint, VariableFlow as F, VariableWrite, resolver};

fn empty() -> Flow {
    Rc::new(F::Empty)
}
fn sequence(flows: impl IntoIterator<Item = Flow>) -> Flow {
    F::sequence(flows)
}

pub(crate) fn expression<A: Arena>(
    expr: &Expression<'_>,
    context: &mut Context<'_, '_, A>,
    block_context: &BlockContext<'_>,
    artifacts: &AnalysisArtifacts,
) {
    if !context.settings.track_variable_flow || context.raw_callback_gather_depth != 0 {
        return;
    }
    let function_name = if let Expression::Call(Call::Function(call)) = expr
        && let Expression::Identifier(identifier) = call.function
    {
        Some(
            context
                .resolve_function_name(identifier)
                .to_ascii_lowercase(),
        )
    } else {
        None
    };
    // VariableFlowBuilder::targetWrite uses the post-assignment scope: releasing an object with
    // a destructor is observable, so copies feeding that scope guard are used through the alias.
    let scope_guard = if let Expression::Assignment(assignment) = expr
        && let Expression::Variable(Variable::Direct(variable)) = assignment.lhs
        && block_context
            .locals
            .get(&word(variable.name))
            .is_some_and(|ty| {
                ty.types.iter().any(|atomic| {
                    atomic.get_all_object_names().iter().any(|name| {
                        context
                            .codebase
                            .method_exists(name.as_bytes(), b"__destruct")
                    })
                })
            }) {
        Rc::new(F::Escape(word(variable.name)))
    } else {
        empty()
    };
    let state = &mut context.variable_flow;
    // The concat analyzer flattens a chain and analyzes only its operands. Give the nested
    // concatenations their fragments, innermost first, before the outer one composes them.
    if let Expression::Binary(binary) = expr
        && matches!(binary.operator, BinaryOperator::StringConcat(_))
    {
        let mut nested = Vec::new();
        let mut stack = vec![binary.lhs, binary.rhs];
        while let Some(operand) = stack.pop() {
            if let Expression::Binary(inner) = operand
                && matches!(inner.operator, BinaryOperator::StringConcat(_))
            {
                nested.push(operand);
                stack.extend([inner.lhs, inner.rhs]);
            }
        }
        for operand in nested.into_iter().rev() {
            let flow = state.expression(operand, artifacts, None);
            state.expressions.insert(operand.span(), flow);
        }
    }
    let flow = sequence([
        state.expression(expr, artifacts, function_name.as_deref()),
        scope_guard,
    ]);
    let throws = artifacts
        .variable_throw_scopes
        .iter()
        .filter(|((span, _), _)| *span == expr.span() && !matches!(expr, Expression::Match(_)))
        .flat_map(|((span, kind), thrown)| {
            thrown.keys().map(move |exception| {
                Rc::new(F::Throw {
                    point: ThrowPoint(*span, *kind, *exception),
                    can_continue: *kind != VariableThrowKind::Expression,
                })
            })
        });
    let flow = sequence(std::iter::once(flow).chain(throws));
    let flow = if artifacts
        .call_terminations
        .get(&(expr.span().start.offset, expr.span().end.offset))
        == Some(&true)
    {
        sequence([flow, Rc::new(F::Stop)])
    } else {
        flow
    };
    state.expressions.insert(expr.span(), flow);
}

pub(crate) fn statement<A: Arena>(
    stmt: &Statement<'_>,
    context: &mut Context<'_, '_, A>,
    artifacts: &AnalysisArtifacts,
) {
    if !context.settings.track_variable_flow || context.raw_callback_gather_depth != 0 {
        return;
    }
    let flow = context.variable_flow.statement(stmt, artifacts);
    context.variable_flow.statements.insert(stmt.span(), flow);
}

pub(crate) fn reference_argument<A: Arena>(
    expr: &Expression<'_>,
    context: &mut Context<'_, '_, A>,
) {
    if !context.settings.track_variable_flow || context.raw_callback_gather_depth != 0 {
        return;
    }
    let state = &mut context.variable_flow;
    let flow = state
        .expressions
        .get(&expr.span())
        .cloned()
        .unwrap_or_else(empty);
    state
        .expressions
        .insert(expr.span(), sequence([flow, State::escape_root(expr)]));
}

impl State {
    /// Completed expression/statement results are authoritative. Structural CST wrappers only compose
    /// those results; absent (unreachable or unanalysed) children do not manufacture variable reads.
    fn child(&self, node: Node<'_, '_>) -> Flow {
        match node {
            Node::Expression(expr) => {
                let raw = self
                    .expressions
                    .get(&expr.span())
                    .cloned()
                    .unwrap_or_else(empty);
                let write = match expr {
                    Expression::Assignment(assignment) => Some(assignment.lhs.span()),
                    Expression::UnaryPrefix(unary)
                        if unary.operator.is_increment_or_decrement() =>
                    {
                        Some(unary.operand.span())
                    }
                    Expression::UnaryPostfix(unary) => Some(unary.operand.span()),
                    _ => None,
                };
                sequence(std::iter::once(raw).chain(write.map(|write| {
                    Rc::new(F::Inputs {
                        write,
                        target: None,
                    })
                })))
            }
            Node::Statement(stmt) => self
                .statements
                .get(&stmt.span())
                .cloned()
                .unwrap_or_else(empty),
            // `$this->$name`: the analyzer reads a selector variable without analyzing it.
            Node::ClassLikeMemberSelector(ClassLikeMemberSelector::Variable(Variable::Direct(
                variable,
            ))) => self
                .expressions
                .get(&variable.span())
                .cloned()
                .unwrap_or_else(|| F::read(word(variable.name), None)),
            Node::Variable(variable) => self
                .expressions
                .get(&variable.span())
                .cloned()
                .unwrap_or_else(|| self.children(node)),
            _ => sequence(node.children().into_iter().map(|child| self.child(child))),
        }
    }

    fn children(&self, node: Node<'_, '_>) -> Flow {
        sequence(node.children().into_iter().map(|child| self.child(child)))
    }

    fn body(&self, statements: &[Statement<'_>]) -> Flow {
        sequence(
            statements
                .iter()
                .map(|stmt| self.child(Node::Statement(stmt))),
        )
    }

    fn value(
        &self,
        expr: &Expression<'_>,
        target: Option<Span>,
        direct: bool,
        artifacts: &AnalysisArtifacts,
    ) -> Flow {
        if !self.expressions.contains_key(&expr.span()) {
            return empty();
        }
        let raw = || {
            self.expressions
                .get(&expr.span())
                .cloned()
                .unwrap_or_else(empty)
        };
        match expr {
            Expression::Variable(Variable::Direct(variable)) if target.is_some() => {
                F::read(word(variable.name), target)
            }
            Expression::Parenthesized(parenthesized) => {
                self.value(parenthesized.expression, target, direct, artifacts)
            }
            Expression::ArrayAccess(access) => self.array_read(access, target, artifacts),
            Expression::Binary(binary) if target.is_some() && !short_circuit(&binary.operator) => {
                sequence([
                    self.value(binary.lhs, target, false, artifacts),
                    self.value(binary.rhs, target, false, artifacts),
                ])
            }
            Expression::UnaryPrefix(unary)
                if !unary.operator.is_increment_or_decrement()
                    && !matches!(unary.operator, UnaryPrefixOperator::Reference(_)) =>
            {
                self.value(unary.operand, target, false, artifacts)
            }
            Expression::Conditional(conditional) => sequence([
                self.child(Node::Expression(conditional.condition)),
                Rc::new(F::Choice(vec![
                    conditional
                        .then
                        .map_or_else(empty, |expr| self.value(expr, target, false, artifacts)),
                    self.value(conditional.r#else, target, false, artifacts),
                ])),
            ]),
            Expression::Match(expression) => self.match_flow(expression, target, artifacts),
            Expression::Assignment(assignment) => sequence([
                raw(),
                Rc::new(F::Inputs {
                    write: assignment.lhs.span(),
                    target,
                }),
            ]),
            Expression::UnaryPrefix(unary) if unary.operator.is_increment_or_decrement() => {
                sequence([
                    raw(),
                    Rc::new(F::Inputs {
                        write: unary.operand.span(),
                        target,
                    }),
                ])
            }
            Expression::UnaryPostfix(unary) => sequence([
                raw(),
                Rc::new(F::Inputs {
                    write: unary.operand.span(),
                    target,
                }),
            ]),
            Expression::Array(array) if target.is_some() => self.array_value(
                array.elements.as_slice(),
                target.unwrap(),
                direct,
                artifacts,
            ),
            Expression::LegacyArray(array) if target.is_some() => self.array_value(
                array.elements.as_slice(),
                target.unwrap(),
                direct,
                artifacts,
            ),
            _ => raw(),
        }
    }

    fn array_value(
        &self,
        elements: &[ArrayElement<'_>],
        target: Span,
        direct: bool,
        artifacts: &AnalysisArtifacts,
    ) -> Flow {
        let parent = self
            .sites
            .get(&target)
            .filter(|write| direct && !write.offset_write);
        let mut next_index: Option<i64> = Some(0);
        let mut flows = Vec::new();
        for (item, value) in elements
            .iter()
            .filter_map(|item| item.get_value().map(|value| (item, value)))
        {
            if let Some(key) = item.get_key() {
                flows.push(self.value(key, Some(target), false, artifacts));
            }
            let offset = if item.is_variadic() {
                next_index = None;
                None
            } else if let Some(key) = item.get_key() {
                let offset = offset(key, artifacts);
                match &offset {
                    Some(Offset::Integer(index)) => {
                        next_index = next_index.map(|next| next.max(index.saturating_add(1)))
                    }
                    None => next_index = None,
                    _ => {}
                }
                offset
            } else {
                let offset = next_index.map(Offset::Integer);
                next_index = next_index.and_then(|index| index.checked_add(1));
                offset
            };
            let destination = if let Some(parent) = parent {
                flows.push(Rc::new(F::Define(VariableWrite {
                    node: item.span(),
                    name: parent.name,
                    offset,
                    offset_write: true,
                    replaces_offset: false,
                    parent: Some(target),
                })));
                item.span()
            } else {
                target
            };
            flows.push(self.value(value, Some(destination), false, artifacts));
        }
        sequence(flows)
    }

    /// ArrayDimFetchHandler evaluates the dimension first, then the container, then the selected
    /// offset. Both pure operands keep the destination write's value-flow context.
    fn array_read(
        &self,
        access: &ArrayAccess<'_>,
        target: Option<Span>,
        artifacts: &AnalysisArtifacts,
    ) -> Flow {
        let dimension = self.value(access.index, target, false, artifacts);
        let (container, selected) =
            if let Expression::Variable(Variable::Direct(variable)) = access.array {
                (
                    Rc::new(F::Read {
                        name: word(variable.name),
                        target,
                        container: true,
                        offset: None,
                    }),
                    Rc::new(F::Read {
                        name: word(variable.name),
                        target,
                        container: false,
                        offset: offset(access.index, artifacts),
                    }),
                )
            } else {
                (self.value(access.array, target, false, artifacts), empty())
            };
        sequence([dimension, container, selected])
    }

    fn match_flow(
        &self,
        expression: &Match<'_>,
        target: Option<Span>,
        artifacts: &AnalysisArtifacts,
    ) -> Flow {
        let mut otherwise = artifacts
            .variable_throw_scopes
            .get(&(expression.span(), VariableThrowKind::Expression))
            .and_then(|throws| {
                throws
                    .keys()
                    .find(|exception| exception.as_bytes() == b"UnhandledMatchError")
            })
            .map_or_else(
                || Rc::new(F::Stop),
                |exception| {
                    Rc::new(F::Throw {
                        point: ThrowPoint(
                            expression.span(),
                            VariableThrowKind::Expression,
                            *exception,
                        ),
                        can_continue: false,
                    })
                },
            );
        for arm in &expression.arms {
            if let MatchArm::Default(arm) = arm {
                otherwise = self.value(arm.expression, target, false, artifacts);
            }
        }
        for arm in expression.arms.iter().rev() {
            let MatchArm::Expression(arm) = arm else {
                continue;
            };
            for condition in arm.conditions.iter().rev() {
                otherwise = sequence([
                    self.child(Node::Expression(condition)),
                    Rc::new(F::Choice(vec![
                        self.value(arm.expression, target, false, artifacts),
                        otherwise,
                    ])),
                ]);
            }
        }
        sequence([
            self.child(Node::Expression(expression.expression)),
            otherwise,
        ])
    }

    fn expression(
        &mut self,
        expr: &Expression<'_>,
        artifacts: &AnalysisArtifacts,
        function_name: Option<&[u8]>,
    ) -> Flow {
        match expr {
            Expression::Variable(variable) => {
                let ty = match variable {
                    Variable::Direct(variable) => return F::read(word(variable.name), None),
                    Variable::Nested(variable) => artifacts.get_expression_type(variable.variable),
                    Variable::Indirect(variable) => {
                        artifacts.get_expression_type(variable.expression)
                    }
                };
                let names: Vec<_> = ty
                    .into_iter()
                    .flat_map(|ty| ty.types.iter())
                    .filter_map(|atomic| atomic.get_literal_string_value())
                    .collect();
                let read = if names.is_empty() {
                    Rc::new(F::ReadAll)
                } else {
                    sequence(
                        names
                            .into_iter()
                            .map(|name| F::read(word(&[b"$".as_slice(), name].concat()), None)),
                    )
                };
                sequence([self.children(Node::Expression(expr)), read])
            }
            Expression::Assignment(assignment) => {
                let site = self.write_site(assignment.lhs, artifacts);
                if let Some(site) = &site {
                    self.sites.insert(site.node, site.clone());
                }
                let target = site.as_ref().map(|site| site.node);
                let simple = assignment.operator.is_assign();
                let coalesce = matches!(assignment.operator, AssignmentOperator::Coalesce(_));
                let rhs = self.value(assignment.rhs, target, simple, artifacts);
                let flow = sequence([
                    if coalesce {
                        empty()
                    } else {
                        self.target_read(
                            assignment.lhs,
                            !simple,
                            if simple { None } else { target },
                            artifacts,
                        )
                    },
                    rhs,
                    self.target_write(assignment.lhs, false, artifacts),
                    if matches!(assignment.rhs, Expression::UnaryPrefix(unary) if unary.operator.is_reference())
                    {
                        Self::escape_root(assignment.lhs)
                    } else {
                        empty()
                    },
                ]);
                if coalesce {
                    sequence([
                        self.target_read(assignment.lhs, true, None, artifacts),
                        Rc::new(F::Choice(vec![empty(), flow])),
                    ])
                } else {
                    flow
                }
            }
            Expression::UnaryPrefix(unary)
                if matches!(unary.operator, UnaryPrefixOperator::Reference(_)) =>
            {
                sequence([
                    self.child(Node::Expression(unary.operand)),
                    Self::escape_root(unary.operand),
                ])
            }
            Expression::UnaryPrefix(unary) if unary.operator.is_increment_or_decrement() => {
                sequence([
                    self.target_read(unary.operand, true, Some(unary.operand.span()), artifacts),
                    self.target_write(unary.operand, false, artifacts),
                ])
            }
            Expression::UnaryPostfix(unary) => sequence([
                self.target_read(unary.operand, true, Some(unary.operand.span()), artifacts),
                self.target_write(unary.operand, false, artifacts),
            ]),
            Expression::Conditional(conditional) => sequence([
                self.child(Node::Expression(conditional.condition)),
                Rc::new(F::Choice(vec![
                    conditional
                        .then
                        .map_or_else(empty, |expr| self.child(Node::Expression(expr))),
                    self.child(Node::Expression(conditional.r#else)),
                ])),
            ]),
            Expression::Binary(binary) if short_circuit(&binary.operator) => sequence([
                self.child(Node::Expression(binary.lhs)),
                Rc::new(F::Choice(vec![
                    empty(),
                    self.child(Node::Expression(binary.rhs)),
                ])),
            ]),
            Expression::Match(expression) => self.match_flow(expression, None, artifacts),
            Expression::ArrayAccess(access) => self.array_read(access, None, artifacts),
            Expression::Closure(closure) => {
                let mut imports = Vec::new();
                let mut escaped = HashSet::default();
                for parameter in &closure.parameter_list.parameters {
                    let name = word(parameter.variable.name);
                    if parameter.ampersand.is_some() {
                        escaped.insert(name);
                    } else {
                        imports.push(Self::write(&parameter.variable));
                    }
                }
                if let Some(uses) = &closure.use_clause {
                    for capture in &uses.variables {
                        if capture.ampersand.is_some() {
                            escaped.insert(word(capture.variable.name));
                        } else {
                            imports.push(Self::write(&capture.variable));
                        }
                    }
                }
                let flow = sequence(
                    imports
                        .into_iter()
                        .chain([self.body(closure.body.statements.as_slice())]),
                );
                self.completed.insert(
                    closure.span(),
                    resolver::resolve(&flow, escaped, closure.ampersand.is_some()),
                );
                sequence(
                    closure
                        .use_clause
                        .iter()
                        .flat_map(|uses| uses.variables.iter())
                        .map(|capture| {
                            let name = word(capture.variable.name);
                            if capture.ampersand.is_some() {
                                Rc::new(F::Escape(name))
                            } else {
                                F::read(name, None)
                            }
                        }),
                )
            }
            Expression::ArrowFunction(arrow) => Rc::new(F::Arrow {
                body: self.child(Node::Expression(arrow.expression)),
                outputs: sequence(
                    arrow
                        .parameter_list
                        .parameters
                        .iter()
                        .filter(|parameter| parameter.ampersand.is_some())
                        .map(|parameter| F::read(word(parameter.variable.name), None)),
                ),
                parameters: arrow
                    .parameter_list
                    .parameters
                    .iter()
                    .map(|parameter| word(parameter.variable.name))
                    .collect(),
            }),
            Expression::Call(Call::Function(call)) => {
                let ordinary = self.children(Node::Expression(expr));
                let special = match function_name {
                    Some(b"get_defined_vars" | b"extract") => Rc::new(F::ReadAll),
                    Some(b"func_get_arg" | b"func_get_args") => Rc::new(F::MentionAll),
                    Some(b"compact") => {
                        let names: Option<Vec<_>> = call
                            .argument_list
                            .arguments
                            .iter()
                            .map(|argument| {
                                if argument.is_unpacked() {
                                    return None;
                                }
                                compact_names(artifacts.get_expression_type(argument.value())?)
                            })
                            .collect();
                        match names {
                            None => Rc::new(F::ReadAll),
                            Some(names) => sequence(
                                names.into_iter().flatten().map(|name| F::read(name, None)),
                            ),
                        }
                    }
                    _ => empty(),
                };
                sequence([ordinary, special])
            }
            Expression::Construct(
                Construct::Eval(_)
                | Construct::Include(_)
                | Construct::IncludeOnce(_)
                | Construct::Require(_)
                | Construct::RequireOnce(_),
            ) => sequence([self.children(Node::Expression(expr)), Rc::new(F::ReadAll)]),
            Expression::Construct(Construct::Exit(_) | Construct::Die(_)) => {
                sequence([self.children(Node::Expression(expr)), Rc::new(F::Stop)])
            }
            Expression::AnonymousClass(class) => class
                .argument_list
                .as_ref()
                .map_or_else(empty, |arguments| {
                    self.child(Node::PartialArgumentList(arguments))
                }),
            _ => self.children(Node::Expression(expr)),
        }
    }

    fn write(variable: &DirectVariable<'_>) -> Flow {
        Rc::new(F::Write(VariableWrite {
            node: variable.span(),
            name: word(variable.name),
            offset: None,
            offset_write: false,
            replaces_offset: false,
            parent: None,
        }))
    }

    fn target_read(
        &self,
        target: &Expression<'_>,
        read: bool,
        value_target: Option<Span>,
        artifacts: &AnalysisArtifacts,
    ) -> Flow {
        match target {
            Expression::Variable(Variable::Direct(variable)) => {
                if read {
                    F::read(word(variable.name), value_target)
                } else {
                    empty()
                }
            }
            Expression::Variable(_) => self.children(Node::Expression(target)),
            Expression::List(_) | Expression::Array(_) => empty(),
            Expression::ArrayAccess(access) => {
                let root = if let Expression::Variable(Variable::Direct(variable)) = access.array {
                    Rc::new(F::Read {
                        name: word(variable.name),
                        target: value_target,
                        container: !read,
                        offset: if read {
                            offset(access.index, artifacts)
                        } else {
                            None
                        },
                    })
                } else {
                    self.target_read(access.array, true, value_target, artifacts)
                };
                sequence([root, self.child(Node::Expression(access.index))])
            }
            Expression::ArrayAppend(append) => {
                if let Expression::Variable(Variable::Direct(variable)) = append.array {
                    Rc::new(F::Read {
                        name: word(variable.name),
                        target: value_target,
                        container: !read,
                        offset: None,
                    })
                } else {
                    self.target_read(append.array, true, value_target, artifacts)
                }
            }
            Expression::Access(_) => self.children(Node::Expression(target)),
            _ => self.child(Node::Expression(target)),
        }
    }

    fn target_write(
        &self,
        target: &Expression<'_>,
        discard: bool,
        artifacts: &AnalysisArtifacts,
    ) -> Flow {
        if let Expression::UnaryPrefix(unary) = target
            && unary.operator.is_reference()
        {
            return sequence([
                self.target_write(unary.operand, discard, artifacts),
                Self::escape_root(unary.operand),
            ]);
        }
        let elements = match target {
            Expression::List(list) => Some(&list.elements),
            Expression::Array(array) => Some(&array.elements),
            _ => None,
        };
        if let Some(elements) = elements {
            return sequence(
                elements
                    .iter()
                    .filter_map(|item| item.get_value().map(|value| (item, value)))
                    .map(|(item, value)| {
                        let (value, by_ref) = if let Expression::UnaryPrefix(unary) = value
                            && unary.operator.is_reference()
                        {
                            (unary.operand, true)
                        } else {
                            (value, false)
                        };
                        sequence([
                            item.get_key()
                                .map_or_else(empty, |key| self.child(Node::Expression(key))),
                            self.target_read(value, false, None, artifacts),
                            self.target_write(value, discard, artifacts),
                            if by_ref {
                                Self::escape_root(value)
                            } else {
                                empty()
                            },
                        ])
                    }),
            );
        }
        let Some(write) = self.write_site(target, artifacts) else {
            return empty();
        };
        Rc::new(if discard {
            F::Discard(write)
        } else {
            F::Write(write)
        })
    }

    fn write_site(
        &self,
        target: &Expression<'_>,
        artifacts: &AnalysisArtifacts,
    ) -> Option<VariableWrite> {
        Some(match target {
            Expression::Variable(Variable::Direct(variable)) => VariableWrite {
                node: variable.span(),
                name: word(variable.name),
                offset: None,
                offset_write: false,
                replaces_offset: false,
                parent: None,
            },
            Expression::ArrayAccess(_) | Expression::ArrayAppend(_) => {
                let mut root = target;
                let mut first_index = None;
                let mut replaces_offset = true;
                loop {
                    let (array, index) = match root {
                        Expression::ArrayAccess(access) => (access.array, Some(access.index)),
                        Expression::ArrayAppend(append) => (append.array, None),
                        _ => break,
                    };
                    replaces_offset = std::ptr::eq(root, target);
                    root = array;
                    first_index = index;
                }
                let Expression::Variable(Variable::Direct(variable)) = root else {
                    return None;
                };
                if artifacts
                    .get_expression_type(root)
                    .is_some_and(|ty| !ty.is_array() && !ty.is_string())
                {
                    return None;
                }
                VariableWrite {
                    node: target.span(),
                    name: word(variable.name),
                    offset: first_index.and_then(|index| offset(index, artifacts)),
                    offset_write: true,
                    replaces_offset,
                    parent: None,
                }
            }
            _ => return None,
        })
    }

    fn escape_root(mut expr: &Expression<'_>) -> Flow {
        loop {
            expr = match expr {
                Expression::ArrayAccess(access) => access.array,
                Expression::ArrayAppend(append) => append.array,
                _ => break,
            };
        }
        if let Expression::Variable(Variable::Direct(variable)) = expr {
            Rc::new(F::Escape(word(variable.name)))
        } else {
            empty()
        }
    }

    fn statement(&self, stmt: &Statement<'_>, artifacts: &AnalysisArtifacts) -> Flow {
        match stmt {
            Statement::Expression(statement) => self
                .expressions
                .get(&statement.expression.span())
                .cloned()
                .unwrap_or_else(empty),
            Statement::If(statement) => {
                let mut otherwise = statement
                    .body
                    .else_statements()
                    .map_or_else(empty, |body| self.body(body));
                for (condition, body) in statement.body.else_if_clauses().into_iter().rev() {
                    otherwise = sequence([
                        self.child(Node::Expression(condition)),
                        Rc::new(F::Choice(vec![self.body(body), otherwise])),
                    ]);
                }
                sequence([
                    self.child(Node::Expression(statement.condition)),
                    Rc::new(F::Choice(vec![
                        self.body(statement.body.statements()),
                        otherwise,
                    ])),
                ])
            }
            Statement::Return(statement) => sequence([
                statement
                    .value
                    .map_or_else(empty, |value| self.child(Node::Expression(value))),
                Rc::new(F::Return(statement.value.and_then(|value| {
                    if let Expression::Variable(Variable::Direct(variable)) = value {
                        Some(word(variable.name))
                    } else {
                        None
                    }
                }))),
            ]),
            Statement::Break(statement) => Rc::new(F::Break(level(statement.level))),
            Statement::Continue(statement) => Rc::new(F::Continue(level(statement.level))),
            Statement::Goto(_) | Statement::Label(_) => Rc::new(F::Opaque),
            Statement::While(statement) => Rc::new(F::Loop {
                condition: self.child(Node::Expression(statement.condition)),
                body: self.body(statement.body.statements()),
                update: empty(),
                at_least_once: false,
            }),
            Statement::DoWhile(statement) => Rc::new(F::Loop {
                condition: empty(),
                body: self.child(Node::Statement(statement.statement)),
                update: self.child(Node::Expression(statement.condition)),
                at_least_once: true,
            }),
            Statement::For(statement) => sequence([
                sequence(
                    statement
                        .initializations
                        .iter()
                        .map(|expr| self.child(Node::Expression(expr))),
                ),
                Rc::new(F::Loop {
                    condition: sequence(
                        statement
                            .conditions
                            .iter()
                            .map(|expr| self.child(Node::Expression(expr))),
                    ),
                    body: self.body(statement.body.statements()),
                    update: sequence(
                        statement
                            .increments
                            .iter()
                            .map(|expr| self.child(Node::Expression(expr))),
                    ),
                    at_least_once: false,
                }),
            ]),
            Statement::Foreach(statement) => sequence([
                self.child(Node::Expression(statement.expression)),
                Rc::new(F::Loop {
                    condition: empty(),
                    body: sequence([
                        statement.target.key().map_or_else(empty, |key| {
                            self.target_read(key, false, None, artifacts)
                        }),
                        statement
                            .target
                            .key()
                            .map_or_else(empty, |key| self.target_write(key, false, artifacts)),
                        self.target_read(statement.target.value(), false, None, artifacts),
                        self.target_write(statement.target.value(), false, artifacts),
                        self.body(statement.body.statements()),
                    ]),
                    update: empty(),
                    at_least_once: false,
                }),
            ]),
            Statement::Try(statement) => Rc::new(F::Try {
                body: self.body(statement.block.statements.as_slice()),
                catches: statement
                    .catch_clauses
                    .iter()
                    .map(|catch| {
                        let points = self.catches.get(&catch.span()).cloned().unwrap_or_default();
                        let flow = if points.is_empty() {
                            mentioned(Node::TryCatchClause(catch))
                        } else {
                            sequence([
                                catch.variable.as_ref().map_or_else(empty, Self::write),
                                self.body(catch.block.statements.as_slice()),
                            ])
                        };
                        (points, flow)
                    })
                    .collect(),
                finally: statement
                    .finally_clause
                    .as_ref()
                    .map_or_else(empty, |finally| {
                        self.body(finally.block.statements.as_slice())
                    }),
            }),
            Statement::Switch(statement) => Rc::new(F::Switch {
                condition: self.child(Node::Expression(statement.expression)),
                exhaustive: statement.body.has_default_case(),
                cases: statement
                    .body
                    .cases()
                    .iter()
                    .map(|case| match case {
                        SwitchCase::Expression(case) => (
                            self.child(Node::Expression(case.expression)),
                            self.body(case.statements.as_slice()),
                            false,
                        ),
                        SwitchCase::Default(case) => {
                            (empty(), self.body(case.statements.as_slice()), true)
                        }
                    })
                    .collect(),
            }),
            Statement::Unset(statement) => sequence(statement.values.iter().map(|value| {
                sequence([
                    self.target_read(value, false, None, artifacts),
                    self.target_write(value, true, artifacts),
                ])
            })),
            Statement::Static(statement) => sequence(statement.items.iter().map(|item| {
                sequence([
                    item.value()
                        .map_or_else(empty, |value| self.child(Node::Expression(value))),
                    Rc::new(F::Escape(word(item.variable().name))),
                ])
            })),
            Statement::Global(statement) => sequence(statement.variables.iter().map(|variable| {
                if let Variable::Direct(variable) = variable {
                    Rc::new(F::Escape(word(variable.name)))
                } else {
                    self.children(Node::Variable(variable))
                }
            })),
            Statement::Function(_)
            | Statement::Class(_)
            | Statement::Interface(_)
            | Statement::Trait(_)
            | Statement::Enum(_) => empty(),
            _ => self.children(Node::Statement(stmt)),
        }
    }
}

fn offset(expr: &Expression<'_>, artifacts: &AnalysisArtifacts) -> Option<Offset> {
    let ty = artifacts.get_expression_type(expr)?;
    // VariableWriteOffset::fromType first calls Type::toArrayKey, then requires one distinct key.
    let mut keys = HashSet::default();
    for atomic in ty.types.as_ref() {
        let key = match atomic {
            TAtomic::Null => Offset::String(Vec::new()),
            TAtomic::Scalar(TScalar::Bool(boolean)) => Offset::Integer(i64::from(boolean.value?)),
            TAtomic::Scalar(TScalar::Float(float)) => {
                // PHP's integer cast wraps finite floats modulo 2^64 and casts NaN/INF to zero.
                let value = float.get_literal_value()?.trunc() % 18446744073709551616.0;
                Offset::Integer(if value >= 9223372036854775808.0 {
                    (value - 18446744073709551616.0) as i64
                } else if value < -9223372036854775808.0 {
                    (value + 18446744073709551616.0) as i64
                } else {
                    value as i64
                })
            }
            _ => {
                let key = atomic.to_array_key()?;
                if let Some(value) = key.get_integer() {
                    Offset::Integer(value)
                } else {
                    Offset::String(key.get_string()?.to_vec())
                }
            }
        };
        keys.insert(key);
    }
    (keys.len() == 1).then(|| keys.into_iter().next().unwrap())
}

fn level(expr: Option<&Expression<'_>>) -> usize {
    match expr {
        Some(Expression::Literal(Literal::Integer(LiteralInteger {
            value: Some(value), ..
        }))) => *value as usize,
        _ => 1,
    }
}

fn short_circuit(operator: &BinaryOperator) -> bool {
    matches!(
        operator,
        BinaryOperator::And(_)
            | BinaryOperator::Or(_)
            | BinaryOperator::LowAnd(_)
            | BinaryOperator::LowOr(_)
            | BinaryOperator::NullCoalesce(_)
    )
}

fn compact_names(ty: &TUnion) -> Option<Vec<Word>> {
    if ty.is_empty_array() {
        return Some(Vec::new());
    }
    let strings: Vec<_> = ty
        .types
        .iter()
        .filter_map(|atomic| atomic.get_literal_string_value())
        .map(|value| word(&[b"$".as_slice(), value].concat()))
        .collect();
    if !strings.is_empty() {
        return Some(strings);
    }
    let mut names = Vec::new();
    for atomic in ty.types.as_ref() {
        let values: Vec<_> = match atomic {
            TAtomic::Array(TArray::Keyed(array)) if array.parameters.is_none() => array
                .known_items
                .as_ref()?
                .values()
                .map(|(_, value)| value)
                .collect(),
            TAtomic::Array(TArray::List(array)) if array.known_count.is_some() => array
                .known_elements
                .as_ref()?
                .values()
                .map(|(_, value)| value)
                .collect(),
            _ => return None,
        };
        for value in values {
            names.extend(compact_names(value)?);
        }
    }
    Some(names)
}

/// PHPStan StatementsHandler::collectMentionedVariables for skipped catches. This deliberately
/// records names, never reads or writes, and is not a fallback for missing semantic fragments.
fn mentioned(node: Node<'_, '_>) -> Flow {
    match node {
        Node::DirectVariable(variable) => return Rc::new(F::Mention(word(variable.name))),
        Node::Variable(Variable::Nested(_) | Variable::Indirect(_)) => {
            return Rc::new(F::MentionAll);
        }
        Node::Closure(closure) => {
            return sequence(
                closure
                    .r#static
                    .is_none()
                    .then(|| Rc::new(F::Mention(word("$this"))))
                    .into_iter()
                    .chain(
                        closure
                            .use_clause
                            .iter()
                            .flat_map(|uses| uses.variables.iter())
                            .map(|capture| Rc::new(F::Mention(word(capture.variable.name)))),
                    ),
            );
        }
        Node::Construct(
            Construct::Eval(_)
            | Construct::Include(_)
            | Construct::IncludeOnce(_)
            | Construct::Require(_)
            | Construct::RequireOnce(_),
        ) => return Rc::new(F::MentionAll),
        Node::FunctionCall(call) => {
            if let Expression::Identifier(identifier) = call.function {
                let name = identifier
                    .value()
                    .strip_prefix(b"\\")
                    .unwrap_or(identifier.value())
                    .to_ascii_lowercase();
                if matches!(
                    name.as_slice(),
                    b"compact" | b"extract" | b"get_defined_vars"
                ) {
                    return Rc::new(F::MentionAll);
                }
            }
        }
        _ => {}
    }
    sequence(node.children().into_iter().map(mentioned))
}
