//! `lint/phpunit/prefer-named-data-provider-keys`, ported from rector-phpunit's `NamedArgumentForDataProviderRector`
//! (rules/PHPUnit110/Rector/Class_).
//!
//! For each public test method of a test class with parameters and `#[DataProvider('name')]` attributes, the data
//! sets of each named provider method of the class (`extractDataProviderArrayItem`: arrays its top-level `yield`s
//! give, the arrays of its top-level `return [...]`, and the arrays in a variable such a `return` lists that a
//! top-level `$x = [...]` gives) get string keys named after the test's parameters, by position: a string key is
//! renamed, and where any string key is there a key is added to a value without one (`String_::fromString`, written
//! double-quoted). A data set that already has every parameter name among its string keys is left alone. Rector
//! changes the tree as it goes, so a provider shared by several tests sees the keys the tests before gave it. Tests
//! that disagree on a shared provider's names make Rector rename the keys back and forth on every run; Pun's fix loop
//! then stops without converging, as Rector never settles either.

use std::collections::HashMap;

use mago_syntax::cst::{
    ArrayElement, ClassLikeMember, Expression, Literal, Method, MethodBody, Node, NodeKind,
    PartialArgument, Statement, Variable,
};
use pun_problem::{Edit, Safety, Span};

use super::{is_in_test_class, is_public, report, resolve};
use crate::rules::rector_stmts::assign;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

const DATA_PROVIDER: &str = "PHPUnit\\Framework\\Attributes\\DataProvider";

pub struct PreferNamedDataProviderKeys;

/// A key of a data set as Rector's tree has it after the changes so far.
#[derive(Clone)]
enum Key {
    /// A string literal key, with its value.
    Str(String),
    /// No key.
    None,
    /// Any other key.
    Other,
}

impl Rule for PreferNamedDataProviderKeys {
    fn id(&self) -> &'static str {
        "lint/phpunit/prefer-named-data-provider-keys"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Class])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Class(class) = node else { return };
        let mut scope = ancestors.to_vec();
        scope.push(node);
        if !is_in_test_class(ctx, &scope, None) {
            return;
        }
        let methods: Vec<&Method<'_>> = class
            .members
            .iter()
            .filter_map(|m| match m {
                ClassLikeMember::Method(m) => Some(m),
                _ => None,
            })
            .collect();
        // the key of each data set item, by the item's start, once Rector changed it
        let mut keys: HashMap<usize, Key> = HashMap::new();
        let mut changed: HashMap<usize, (Span, String)> = HashMap::new();
        for method in &methods {
            if !is_public(method) || method.parameter_list.parameters.is_empty() {
                continue;
            }
            let names: Vec<String> = method
                .parameter_list
                .parameters
                .iter()
                .map(|p| String::from_utf8_lossy(&p.variable.name[1..]).into_owned())
                .collect();
            for provider in provider_names(&scope, method) {
                let Some(provider) = methods
                    .iter()
                    .find(|m| m.name.value.eq_ignore_ascii_case(provider.as_bytes()))
                else {
                    continue;
                };
                for elements in data_sets(provider) {
                    let current: Vec<Key> = elements
                        .iter()
                        .map(|e| {
                            let start = node_span(*e).start;
                            keys.get(&start).cloned().unwrap_or_else(|| key_of(e))
                        })
                        .collect();
                    let string_keys: Vec<&String> = current
                        .iter()
                        .filter_map(|k| match k {
                            Key::Str(s) => Some(s),
                            _ => None,
                        })
                        .collect();
                    if names.iter().all(|n| string_keys.contains(&n)) {
                        continue;
                    }
                    let need_all = !string_keys.is_empty();
                    for (at, element) in elements.iter().enumerate() {
                        let Some(name) = names.get(at) else {
                            continue;
                        };
                        let start = node_span(*element).start;
                        let (span, text) = match (&current[at], element) {
                            (Key::None, _) if need_all => {
                                (Span::new(start, start), format!("\"{name}\" => "))
                            }
                            (Key::Str(s), ArrayElement::KeyValue(kv)) if s != name => {
                                let quote = match kv.key {
                                    Expression::Literal(Literal::String(s))
                                        if s.raw.starts_with(b"\"") =>
                                    {
                                        '"'
                                    }
                                    _ => '\'',
                                };
                                (node_span(kv.key), format!("{quote}{name}{quote}"))
                            }
                            _ => continue,
                        };
                        keys.insert(start, Key::Str(name.clone()));
                        changed.insert(start, (span, text));
                    }
                }
            }
        }
        if changed.is_empty() {
            return;
        }
        let mut edits: Vec<Edit> = changed
            .into_values()
            .map(|(span, text)| Edit::new(span, text))
            .collect();
        edits.sort_by_key(|e| e.span.start);
        out.push(report(
            node_span(&class.name),
            "Name the data provider's keys after the test's parameters.",
            Safety::Unsafe,
            edits,
        ));
    }
}

/// The key of a data set item as written.
fn key_of(element: &ArrayElement<'_>) -> Key {
    match element {
        ArrayElement::KeyValue(kv) => match kv.key {
            Expression::Literal(Literal::String(s)) => {
                Key::Str(String::from_utf8_lossy(s.value.unwrap_or_default()).into_owned())
            }
            _ => Key::Other,
        },
        ArrayElement::Value(_) | ArrayElement::Variadic(_) => Key::None,
        ArrayElement::Missing(_) => Key::Other,
    }
}

/// `getDataProviderMethodNames`: the string arguments of the method's `#[DataProvider]` attributes.
fn provider_names(ancestors: &[Node<'_, '_>], method: &Method<'_>) -> Vec<String> {
    let mut names = Vec::new();
    for list in method.attribute_lists.iter() {
        for attribute in list.attributes.iter() {
            if !resolve(ancestors, &attribute.name).eq_ignore_ascii_case(DATA_PROVIDER) {
                continue;
            }
            for arg in attribute
                .argument_list
                .iter()
                .flat_map(|a| a.arguments.iter())
            {
                let value = match arg {
                    PartialArgument::Positional(a) => a.value,
                    PartialArgument::Named(a) => a.value,
                    _ => continue,
                };
                if let Expression::Literal(Literal::String(s)) = value {
                    names.push(String::from_utf8_lossy(s.value.unwrap_or_default()).into_owned());
                }
            }
        }
    }
    names
}

fn elements<'a>(expr: &'a Expression<'a>) -> Option<Vec<&'a ArrayElement<'a>>> {
    match unparenthesize(expr) {
        Expression::Array(a) => Some(a.elements.iter().collect()),
        Expression::LegacyArray(a) => Some(a.elements.iter().collect()),
        _ => None,
    }
}

fn element_value<'a>(element: &'a ArrayElement<'a>) -> Option<&'a Expression<'a>> {
    match element {
        ArrayElement::KeyValue(kv) => Some(kv.value),
        ArrayElement::Value(v) => Some(v.value),
        ArrayElement::Variadic(v) => Some(v.value),
        ArrayElement::Missing(_) => None,
    }
}

/// `extractDataProviderArrayItem`: the items of each data set array of a provider method.
fn data_sets<'a>(method: &'a Method<'a>) -> Vec<Vec<&'a ArrayElement<'a>>> {
    let MethodBody::Concrete(block) = &method.body else {
        return Vec::new();
    };
    let mut variables: HashMap<&[u8], &Expression<'_>> = HashMap::new();
    for statement in block.statements.iter() {
        if let Statement::Expression(e) = statement
            && let Some(a) = assign(e.expression)
            && let Expression::Variable(Variable::Direct(v)) = a.lhs
            && elements(a.rhs).is_some()
        {
            variables.insert(v.name, a.rhs);
        }
    }
    let mut out = Vec::new();
    for statement in block.statements.iter() {
        match statement {
            Statement::Expression(e) => {
                if let Expression::Yield(y) = e.expression {
                    let value = match y {
                        mago_syntax::cst::Yield::Value(v) => v.value,
                        mago_syntax::cst::Yield::Pair(p) => Some(p.value),
                        mago_syntax::cst::Yield::From(_) => None,
                    };
                    out.extend(value.and_then(elements));
                }
            }
            Statement::Return(r) => {
                let Some(cases) = r.value.and_then(elements) else {
                    continue;
                };
                for case in cases {
                    let Some(value) = element_value(case) else {
                        continue;
                    };
                    if let Some(items) = elements(value) {
                        out.push(items);
                    }
                    if let Expression::Variable(Variable::Direct(v)) = unparenthesize(value)
                        && let Some(list) = variables.get(v.name).and_then(|e| elements(e))
                    {
                        out.extend(
                            list.into_iter()
                                .filter_map(element_value)
                                .filter_map(elements),
                        );
                    }
                }
            }
            _ => {}
        }
    }
    out
}
