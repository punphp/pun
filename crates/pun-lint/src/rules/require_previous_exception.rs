//! `lint/complexity/require-previous-exception`, ported from Rector 2.6.1's `ThrowWithPreviousExceptionRector`
//! (rules/CodeQuality/Rector/Catch_).
//!
//! In a `catch` with a variable, each `throw new <Name>(...)` below it (at any depth) passes the caught exception on:
//! the message (`$e->getMessage()`) when there is no argument, the code (`$e->getCode()`) when the constructor has a
//! `$code` parameter, and the exception at the constructor's first parameter that takes a `Throwable`. The constructor
//! comes from the class's reflection, docblock types read: an unknown class, or one without a constructor, takes the
//! exception third.
//!
//! Rector drops the `new`'s original node, so php-parser prints it anew (`pExpr_New`): `new`, the class as written,
//! and the arguments as written joined with `, ` (`pMaybeMultiline`: a line each, one level deeper, when one has
//! comments before it).

use mago_codex::metadata::CodebaseMetadata;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::atomic::reference::TReference;
use mago_codex::ttype::union::TUnion;
use mago_span::HasSpan;
use mago_syntax::cst::{Argument, Expression, Instantiation, Node, NodeKind, Variable};
use pun_problem::{Edit, Fix, Safety, Span};

use super::php_parser::previous_token_end;
use super::rector_batch5::resolve_class_name;
use super::rector_conditions::{indentation, print_comments, reprint};
use super::types::facts::facts;
use crate::syntax::{node_span, unparenthesize};
use crate::{Context, Report, Rule};

pub struct RequirePreviousException;

impl Rule for RequirePreviousException {
    fn id(&self) -> &'static str {
        "lint/complexity/require-previous-exception"
    }

    fn targets(&self) -> Option<&'static [NodeKind]> {
        Some(&[NodeKind::Throw])
    }

    fn visit<'ast, 'arena>(
        &self,
        node: Node<'ast, 'arena>,
        ancestors: &[Node<'ast, 'arena>],
        ctx: &Context<'_>,
        out: &mut Vec<Report>,
    ) {
        let Node::Throw(t) = node else { return };
        let Expression::Instantiation(new) = t.exception else {
            return;
        };
        // Each `catch` around the throw refactors it in turn, the outermost first; the first that changes it wins.
        let caught = ancestors.iter().filter_map(|a| match a {
            Node::TryCatchClause(c) => c.variable.as_ref().map(|v| v.name),
            _ => None,
        });
        for variable in caught {
            let Some(text) = refactor(
                ctx,
                ancestors,
                t.span().start.offset as usize,
                new,
                variable,
            ) else {
                continue;
            };
            out.push(Report {
                span: node_span(t.exception),
                message: "Pass the caught exception on to the exception thrown in its `catch`."
                    .into(),
                help: None,
                fix: Some(Fix {
                    safety: Safety::Safe,
                    edits: vec![Edit::new(node_span(t.exception), text)],
                }),
            });
            return;
        }
    }
}

/// One argument of the new `new`: its name, and its value, as source text (`None`: as written at that span).
struct Arg {
    name: Option<String>,
    value: String,
    /// The written argument, when it is one (for the renamed one, the text between its name and its value).
    written: Option<Span>,
    /// Where the written argument starts.
    start: usize,
}

/// `refactorThrow`: the `new` printed anew with the caught `$variable` passed on, if Rector changes it.
fn refactor(
    ctx: &Context<'_>,
    ancestors: &[Node<'_, '_>],
    throw_start: usize,
    new: &Instantiation<'_>,
    variable: &[u8],
) -> Option<String> {
    let source = ctx.source;
    let class = match new.class {
        Expression::Identifier(name) => Some(resolve_class_name(ancestors, name)),
        Expression::Self_(_) | Expression::Static(_) | Expression::Parent(_) => None,
        _ => return None,
    };
    let facts = facts(ctx);
    let codebase = facts.documented();
    // `hasClass()` and `getConstructor()`: an unknown class (`self` too) has no constructor
    let constructor = class
        .as_deref()
        .filter(|c| codebase.get_class_like(c.as_bytes()).is_some())
        .and_then(|c| codebase.get_declaring_method(c.as_bytes(), b"__construct"));
    let has_parameter = |name: &str| {
        constructor.is_some_and(|m| {
            m.parameters
                .iter()
                .any(|p| &p.get_name().0.as_bytes()[1..] == name.as_bytes())
        })
    };
    let position = match constructor {
        None => 2,
        Some(m) => m.parameters.iter().position(|p| {
            let t = p.type_metadata.as_ref().map(|t| &t.type_union);
            takes_throwable(codebase, t)
        })?,
    };
    let written: Vec<&Argument<'_>> = new
        .argument_list
        .as_ref()
        .map(|l| l.arguments.iter().collect())
        .unwrap_or_default();
    // exception is bundled; Pun also stops when a positional argument is the caught exception, which Rector passes
    // again where the constructor takes it after the arguments (`new \ErrorException('m', 1, $e)`), so that its fix
    // rounds settle
    if written.len() > position
        || written
            .iter()
            .any(|a| matches!(a, Argument::Positional(p) if is_variable(p.value, variable)))
    {
        return None;
    }
    let mut args: Vec<Arg> = written
        .iter()
        .map(|a| Arg {
            name: match a {
                Argument::Named(n) => Some(String::from_utf8_lossy(n.name.value).into_owned()),
                Argument::Positional(_) => None,
            },
            value: String::new(),
            written: Some(node_span(*a)),
            start: node_span(*a).start,
        })
        .collect();
    let has_argument =
        |args: &[Arg], name: &str| args.iter().any(|a| a.name.as_deref() == Some(name));
    let named = args.first().is_some_and(|a| a.name.is_some());
    let var = String::from_utf8_lossy(variable).into_owned();
    let has_code_parameter = has_parameter("code");
    let has_code_argument = has_argument(&args, "code");
    if args.len() < 2 && (!has_code_parameter || has_code_argument) {
        return None;
    }
    let mut changed = false;
    if args.is_empty() {
        args.push(new_arg(None, format!("{var}->getMessage()")));
        changed = true;
    } else if let Some(Argument::Named(first)) = written.first()
        && first.name.value == b"previous"
        && is_variable(first.value, variable)
    {
        args[0] = Arg {
            name: Some("message".into()),
            value: format!("{var}->getMessage()"),
            // the text between the name and the value
            written: Some(Span::new(
                node_span(&first.name).end,
                node_span(first.value).start,
            )),
            start: node_span(&first.name).start,
        };
        changed = true;
    }
    let code_name = named.then(|| "code".to_string());
    // fewer than two arguments here means a missing code (checked above)
    if args.len() < 2 {
        args.push(new_arg(code_name.clone(), format!("{var}->getCode()")));
        changed = true;
    }
    if args[1].name.as_deref() == Some("previous") {
        if has_code_parameter && !has_code_argument {
            let previous = std::mem::replace(
                &mut args[1],
                new_arg(code_name, format!("{var}->getCode()")),
            );
            args.push(previous);
        } else {
            // nothing changed before: a named `previous` second follows a positional or named first argument that
            // is not `previous` too
            return None;
        }
    } else if has_parameter("previous") && !has_argument(&args, "previous") {
        args.push(new_arg(named.then(|| "previous".to_string()), var));
    } else if !changed {
        return None;
    }
    Some(print_new(source, throw_start, new, &args))
}

fn new_arg(name: Option<String>, value: String) -> Arg {
    Arg {
        name,
        value,
        written: None,
        start: 0,
    }
}

/// `pExpr_New` with the arguments `args`, at the indent of the line `throw_start` is on.
fn print_new(source: &str, throw_start: usize, new: &Instantiation<'_>, args: &[Arg]) -> String {
    let level = indentation(source, throw_start);
    let tokens = crate::rules::php_parser::lex(source);
    let mut items: Vec<(Vec<Span>, &Arg)> = Vec::new();
    for arg in args {
        let comments = match arg.written {
            Some(_) => {
                let start = previous_token_end(source, &tokens, arg.start);
                let found = tokens
                    .iter()
                    .filter(|t| t.start >= start && t.end <= arg.start && t.kind.is_comment())
                    .map(|t| Span::new(t.start, t.end))
                    .collect();
                found
            }
            None => Vec::new(),
        };
        items.push((comments, arg));
    }
    let multiline = items.iter().any(|(c, _)| !c.is_empty());
    let item_level = if multiline { level + 4 } else { level };
    let print = |arg: &Arg| match (arg.written, &arg.name) {
        (Some(span), _) if arg.value.is_empty() => {
            let adjust = item_level as isize - indentation(source, span.start) as isize;
            reprint(source, &tokens, span, adjust)
        }
        (Some(span), Some(name)) => {
            // the renamed `previous:` argument: the text between its name and value kept
            format!("{name}{}{}", &source[span.start..span.end], arg.value)
        }
        (_, Some(name)) => format!("{name}: {}", arg.value),
        (_, None) => arg.value.clone(),
    };
    let class = &source[node_span(new.class).start..node_span(new.class).end];
    let list = if multiline {
        let nl = format!("\n{}", " ".repeat(item_level));
        let lines: Vec<String> = items
            .iter()
            .map(|(comments, arg)| {
                let mut line = String::new();
                if !comments.is_empty() {
                    line.push_str(&nl);
                    line.push_str(&print_comments(source, comments, &nl));
                }
                line.push_str(&nl);
                line.push_str(&print(arg));
                line
            })
            .collect();
        format!("{}\n{}", lines.join(","), " ".repeat(level))
    } else {
        items
            .iter()
            .map(|(_, a)| print(a))
            .collect::<Vec<_>>()
            .join(", ")
    };
    format!("new {class}({list})")
}

/// `areNodesEqual($value, $caughtThrowableVariable)`: the same variable.
fn is_variable(value: &Expression<'_>, variable: &[u8]) -> bool {
    matches!(unparenthesize(value), Expression::Variable(Variable::Direct(d)) if d.name == variable)
}

/// Whether `resolveExceptionArgumentPosition` stops at a parameter of type `t`: with one object class in it,
/// `Throwable` not a no for a supertype of it; else, `null` taken out, a yes.
fn takes_throwable(codebase: &CodebaseMetadata, t: Option<&TUnion>) -> bool {
    let Some(t) = t else {
        // no type: mixed, a maybe
        return false;
    };
    let mut classes: Vec<String> = t.types.iter().filter_map(class_name).collect();
    classes.dedup();
    if let [class] = &classes[..] {
        // Rector 2.6.1 on PHPStan 2.2.7: a lone class is a maybe unless it is final or a trait; in a union with
        // other types (`?Foo` too), only a `Throwable` class stops it
        if t.types.len() == 1 {
            return throwable_of(codebase, class) != Some(false);
        }
        return throwable_of(codebase, class) == Some(true);
    }
    let kept: Vec<&TAtomic> = t
        .types
        .iter()
        .filter(|a| !matches!(a, TAtomic::Null))
        .collect();
    !kept.is_empty()
        && kept
            .iter()
            .all(|a| class_name(a).is_some_and(|c| throwable_of(codebase, &c) == Some(true)))
}

/// The class an object type names (`getObjectClassNames()`), a class Mago does not know too; an enum, which cannot
/// be a `Throwable`, has none here.
fn class_name(atomic: &TAtomic) -> Option<String> {
    let name = match atomic {
        TAtomic::Object(TObject::Named(n)) => n.name,
        TAtomic::Reference(TReference::Symbol { name, .. }) => *name,
        _ => return None,
    };
    Some(String::from_utf8_lossy(name.as_bytes()).into_owned())
}

/// `(new ObjectType('Throwable'))->isSuperTypeOf(new ObjectType($class))`: `Some(true)` for a yes, `Some(false)` for a
/// no, `None` for a maybe.
fn throwable_of(codebase: &CodebaseMetadata, class: &str) -> Option<bool> {
    let Some(metadata) = codebase.get_class_like(class.as_bytes()) else {
        return None;
    };
    if codebase.is_instance_of(class.as_bytes(), b"Throwable") {
        return Some(true);
    }
    if metadata.kind.is_trait() || metadata.kind.is_enum() || metadata.flags.is_final() {
        return Some(false);
    }
    None
}
