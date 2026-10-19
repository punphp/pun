//! Larastan's ConfigParser: the app's config files, and whether a config key's value is an array
//! (`getTypes()` read through `isArray()`, which is all ConfigCollectionRule asks of the types).
//!
//! Larastan types a value with `$scope->getType()` of its expression in the config file; Pun reads the expression
//! instead: literals, strings, casts, `X::class`, `env()` (Larastan's stub returns `bool|string|TDefault`), and a
//! function's declared return type. Anything else may be an array.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use mago_allocator::LocalArena;
use mago_codex::metadata::CodebaseMetadata;
use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::callable::TCallable;
use mago_codex::ttype::atomic::object::TObject;
use mago_codex::ttype::union::TUnion;
use mago_database::file::FileType;
use mago_names::ResolvedNames;
use mago_names::resolver::NameResolver;
use mago_span::HasSpan;
use mago_syntax::cst::*;
use mago_syntax::parser::parse_file;
use mago_syntax::walker::MutWalker;
use pun_phpstan::phpdoc::{self, Node, ShapeKey};

use crate::codebase::read;

/// `isArray()` of a type, as far as Pun can tell.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum IsArray {
    No,
    Maybe,
}

/// A config file's first `return`.
#[derive(Clone, Debug)]
enum Returned {
    /// Its `@return` PHPDoc type.
    Doc(Node),
    Array(Vec<Item>),
    /// Not an array literal: no type.
    Other,
}

/// An array literal's item: its key (`None` when it is no scalar, which stops the lookup; `Some(None)` for a scalar
/// without a value, which is skipped) and value.
#[derive(Clone, Debug)]
struct Item {
    key: Option<Option<String>>,
    value: Value,
}

#[derive(Clone, Debug)]
enum Value {
    Array(Vec<Item>),
    Other(IsArray),
}

/// The config files Larastan reads: those below the configured directories, else below `config_path()`.
#[derive(Clone, Debug, Default)]
pub(crate) struct Configs {
    /// Each file and its first `return` (`None`: a parse error or no `return`), by path.
    files: BTreeMap<PathBuf, Option<Returned>>,
}

impl Configs {
    pub(crate) fn load(
        directories: &[String],
        root: &Path,
        codebase: &CodebaseMetadata,
    ) -> Configs {
        let php = ["php".to_string()];
        let mut paths: Vec<PathBuf> = directories
            .iter()
            .flat_map(|d| crate::php_files(Path::new(d), &php))
            .collect();
        if paths.is_empty() {
            paths = crate::php_files(&root.join("config"), &php);
        }
        // ponytail: sorted, where PHP's RecursiveDirectoryIterator follows readdir order
        Configs {
            files: paths
                .into_iter()
                .map(|p| {
                    let returned = returned(&p, codebase);
                    (p, returned)
                })
                .collect(),
        }
    }

    /// `ConfigParser::getTypes()` for the keys: `None` when a key's file is missing (no types at all), else the
    /// keys' types (a key whose value is not found has none).
    pub(crate) fn types(&self, keys: &[String]) -> Option<Vec<IsArray>> {
        let mut types = Vec::new();
        for key in keys {
            let mut parts = key.split('.');
            let file = parts.next().unwrap_or_default();
            let parts: Vec<&str> = parts.collect();
            let suffix = format!("{file}.php");
            let returned = self
                .files
                .iter()
                .filter(|(p, _)| p.to_string_lossy().ends_with(&suffix))
                .find_map(|(_, r)| r.as_ref())?;
            types.extend(match returned {
                Returned::Doc(node) => Some(doc_type(node, &parts)),
                Returned::Array(items) => lookup(items, &parts),
                Returned::Other => None,
            });
        }
        Some(types)
    }
}

/// The value at `parts`, as getTypes() walks the items (a part not found keeps the value found so far).
fn lookup(items: &[Item], parts: &[&str]) -> Option<IsArray> {
    if parts.is_empty() {
        return Some(IsArray::Maybe);
    }
    let mut items = items;
    let mut found = None;
    for part in parts {
        let current = items;
        for item in current {
            let key = item.key.as_ref()?;
            if key.as_deref() != Some(*part) {
                continue;
            }
            if let Value::Array(nested) = &item.value {
                items = nested;
            }
            found = Some(&item.value);
        }
    }
    match found? {
        Value::Array(_) => Some(IsArray::Maybe),
        Value::Other(t) => Some(*t),
    }
}

/// The `@return` type's offset at `parts`, through array shapes and `array<K, V>`.
fn doc_type(node: &Node, parts: &[&str]) -> IsArray {
    let mut node = node;
    for part in parts {
        node = match node {
            Node::ArrayShape { items, .. } => match items.iter().find(|i| {
                matches!(&i.key, Some(ShapeKey::Int(k) | ShapeKey::Str(k) | ShapeKey::Ident(k)) if k == part)
            }) {
                Some(i) => &i.value,
                None => return IsArray::Maybe,
            },
            Node::Generic { base, args } if is_array_name(base) && !args.is_empty() => &args[args.len() - 1],
            Node::Array(item) => item,
            _ => return IsArray::Maybe,
        };
    }
    doc_is_array(node)
}

fn is_array_name(name: &str) -> bool {
    matches!(
        name.to_ascii_lowercase().as_str(),
        "array" | "list" | "non-empty-array" | "non-empty-list"
    )
}

fn doc_is_array(node: &Node) -> IsArray {
    let no = match node {
        Node::Ident(name) => matches!(
            name.to_ascii_lowercase().as_str(),
            "string"
                | "non-empty-string"
                | "non-falsy-string"
                | "numeric-string"
                | "lowercase-string"
                | "class-string"
                | "int"
                | "integer"
                | "positive-int"
                | "negative-int"
                | "float"
                | "double"
                | "bool"
                | "boolean"
                | "true"
                | "false"
                | "null"
                | "scalar"
        ),
        Node::Const(_) => true,
        Node::Nullable(n) => doc_is_array(n) == IsArray::No,
        Node::Union(nodes) => nodes.iter().all(|n| doc_is_array(n) == IsArray::No),
        _ => false,
    };
    if no { IsArray::No } else { IsArray::Maybe }
}

/// ConfigParser::parseConfigFile() and what getTypes() reads of it.
fn returned(path: &Path, codebase: &CodebaseMetadata) -> Option<Returned> {
    let file = read(path, FileType::Host).ok()??;
    let arena = LocalArena::new();
    let program = parse_file(&arena, &file);
    if program.has_errors() {
        return None;
    }
    let names = NameResolver::new(&arena).resolve(program);
    let mut first = FirstReturn(None);
    first.walk_program(program, &mut ());
    let r = first.0?;
    if let Some(doc) = crate::rules::doc_before(&file.contents, r.span().start.offset as usize)
        && let Some((_, ty)) = crate::rules::doc_tags(doc)
            .into_iter()
            .find(|(tag, _)| tag == "@return" || tag == "@phpstan-return")
        && let Ok(node) = phpdoc::parse_prefix(&ty).map(|(n, _)| n)
    {
        return Some(Returned::Doc(node));
    }
    Some(match r.value {
        Some(Expression::Array(a)) => Returned::Array(items(&a.elements, &names, codebase)),
        Some(Expression::LegacyArray(a)) => Returned::Array(items(&a.elements, &names, codebase)),
        _ => Returned::Other,
    })
}

struct FirstReturn<'ast, 'arena>(Option<&'ast Return<'arena>>);

impl<'ast, 'arena> MutWalker<'ast, 'arena, ()> for FirstReturn<'ast, 'arena> {
    fn walk_in_return(&mut self, r: &'ast Return<'arena>, _: &mut ()) {
        self.0.get_or_insert(r);
    }
}

fn items(
    elements: &TokenSeparatedSequence<ArrayElement>,
    names: &ResolvedNames,
    codebase: &CodebaseMetadata,
) -> Vec<Item> {
    elements
        .iter()
        .map(|e| match e {
            ArrayElement::KeyValue(kv) => Item {
                key: match kv.key {
                    Expression::Literal(Literal::String(s)) => Some(Some(
                        String::from_utf8_lossy(s.value.unwrap_or_default()).into_owned(),
                    )),
                    Expression::Literal(Literal::Integer(i)) => {
                        Some(i.value.map(|v| v.to_string()))
                    }
                    Expression::Literal(Literal::Float(f)) => {
                        Some(Some(pun_phpstan::types::float_to_string(*f.value)))
                    }
                    // a scalar without `value` (magic constants, interpolated strings)
                    Expression::MagicConstant(_) | Expression::CompositeString(_) => Some(None),
                    _ => None,
                },
                value: value(kv.value, names, codebase),
            },
            ArrayElement::Value(v) => Item {
                key: None,
                value: value(v.value, names, codebase),
            },
            _ => Item {
                key: None,
                value: Value::Other(IsArray::Maybe),
            },
        })
        .collect()
}

fn value(e: &Expression, names: &ResolvedNames, codebase: &CodebaseMetadata) -> Value {
    match e {
        Expression::Array(a) => Value::Array(items(&a.elements, names, codebase)),
        Expression::LegacyArray(a) => Value::Array(items(&a.elements, names, codebase)),
        e => Value::Other(is_array(e, names, codebase)),
    }
}

/// `$scope->getType($expr)->isArray()` for a config value.
fn is_array(e: &Expression, names: &ResolvedNames, codebase: &CodebaseMetadata) -> IsArray {
    let no = match e {
        Expression::Literal(_) | Expression::CompositeString(_) => true,
        Expression::Binary(b) => b.operator.is_concatenation(),
        Expression::Access(Access::ClassConstant(c)) => {
            matches!(&c.constant, ClassLikeConstantSelector::Identifier(i) if i.value.eq_ignore_ascii_case(b"class"))
        }
        Expression::UnaryPrefix(u) => matches!(
            u.operator,
            UnaryPrefixOperator::StringCast(..)
                | UnaryPrefixOperator::IntCast(..)
                | UnaryPrefixOperator::IntegerCast(..)
                | UnaryPrefixOperator::FloatCast(..)
                | UnaryPrefixOperator::DoubleCast(..)
                | UnaryPrefixOperator::RealCast(..)
                | UnaryPrefixOperator::BoolCast(..)
                | UnaryPrefixOperator::BooleanCast(..)
        ),
        Expression::Call(Call::Function(f)) => match f.function {
            Expression::Identifier(id) => {
                let name = names.get(&id.span());
                let short = name.rsplit(|b| *b == b'\\').next().unwrap_or(name);
                if short.eq_ignore_ascii_case(b"env") {
                    // bool|string|TDefault
                    f.argument_list
                        .arguments
                        .get(1)
                        .is_none_or(|d| is_array(d.value(), names, codebase) == IsArray::No)
                } else {
                    codebase
                        .get_function(name)
                        .or_else(|| codebase.get_function(short))
                        .and_then(|f| f.return_type_metadata.as_ref())
                        .is_some_and(|t| union_is_array(&t.type_union) == IsArray::No)
                }
            }
            _ => false,
        },
        _ => false,
    };
    if no { IsArray::No } else { IsArray::Maybe }
}

/// `isArray()` of a Mago type: no when every part is a scalar, null or an object other than a closure; a closure's
/// return type when it is one (ConfigHelper calls a callable default).
pub(crate) fn union_is_array(t: &TUnion) -> IsArray {
    let no = !t.types.is_empty()
        && t.types.iter().all(|a| match a {
            TAtomic::Scalar(_)
            | TAtomic::Null
            | TAtomic::Void
            | TAtomic::Never
            | TAtomic::Resource(_) => true,
            TAtomic::Object(TObject::Named(n)) => {
                !n.name.as_bytes().eq_ignore_ascii_case(b"closure")
            }
            TAtomic::Object(TObject::Enum(_)) => true,
            TAtomic::Callable(TCallable::Signature(s)) => s
                .get_return_type()
                .is_some_and(|r| union_is_array(r) == IsArray::No),
            _ => false,
        });
    if no { IsArray::No } else { IsArray::Maybe }
}
