//! Port of Mago's `no-implicit-model-query` (Laravel): `User::where(...)` → `User::query()->where(...)`.
//! Opt-in only (`Pun/no_implicit_model_query`). Risky: like Mago it is a name heuristic (query-builder
//! method called statically on any class except Laravel's own facades/helpers), with no type information.
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Token, Tokens};
use std::collections::HashMap;
use std::path::Path;

struct NoImplicitModelQuery;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(NoImplicitModelQuery)
}

impl Fixer for NoImplicitModelQuery {
    fn name(&self) -> &'static str {
        "Pun/no_implicit_model_query"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_token_kind_found(T_DOUBLE_COLON)
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let aliases = import_aliases(tokens);
        for i in (0..tokens.count()).rev() {
            if !tokens[i].is_kind(T_DOUBLE_COLON) {
                continue;
            }
            // PHP-CS-Fixer tokens split `\App\User` into T_NS_SEPARATOR / T_STRING parts.
            let Some(class) = tokens
                .get_prev_meaningful_token(i)
                .filter(|&c| tokens[c].is_kind(T_STRING))
            else {
                continue;
            };
            let mut start = class;
            while let Some(p) = tokens
                .get_prev_meaningful_token(start)
                .filter(|&p| tokens[p].is_given_kind(&[T_NS_SEPARATOR, T_STRING, T_NAMESPACE]))
            {
                start = p;
            }
            if tokens.get_prev_meaningful_token(start).is_some_and(|p| {
                tokens[p].is_given_kind(&[
                    T_OBJECT_OPERATOR,
                    T_NULLSAFE_OBJECT_OPERATOR,
                    T_DOUBLE_COLON,
                    T_NEW,
                ])
            }) {
                continue;
            }
            let Some(method) = tokens
                .get_next_meaningful_token(i)
                .filter(|&m| tokens[m].is_kind(T_STRING))
            else {
                continue;
            };
            if !tokens
                .get_next_meaningful_token(method)
                .is_some_and(|n| tokens[n].equals("(", true))
            {
                continue;
            }
            if !QUERY_BUILDER_METHODS.iter().any(|m| {
                tokens[method]
                    .get_content()
                    .eq_ignore_ascii_case(m.as_bytes())
            }) {
                continue;
            }
            let short = tokens[class].get_content().to_ascii_lowercase();
            if start == class && (short == b"self" || short == b"parent") {
                continue;
            }
            let resolved = if start != class {
                short
            } else {
                aliases.get(&short).cloned().unwrap_or(short)
            };
            if KNOWN_NON_MODEL_CLASSES
                .iter()
                .any(|c| c.as_bytes().eq_ignore_ascii_case(&resolved))
            {
                continue;
            }
            tokens.insert_at(
                method,
                vec![
                    Token::new(T_STRING, "query"),
                    Token::char("("),
                    Token::char(")"),
                    Token::new(T_OBJECT_OPERATOR, "->"),
                ],
            );
        }
    }
}

/// `use Foo\Bar as Baz;` → `baz` => `bar` (lowercased short names), so aliased facades are still excluded.
fn import_aliases(tokens: &Tokens) -> HashMap<Vec<u8>, Vec<u8>> {
    let mut map = HashMap::new();
    for i in 0..tokens.count() {
        if !tokens[i].is_kind(T_AS) {
            continue;
        }
        let name = tokens
            .get_prev_meaningful_token(i)
            .filter(|&n| tokens[n].is_kind(T_STRING));
        let alias = tokens
            .get_next_meaningful_token(i)
            .filter(|&a| tokens[a].is_kind(T_STRING));
        if let (Some(name), Some(alias)) = (name, alias) {
            map.insert(
                tokens[alias].get_content().to_ascii_lowercase(),
                tokens[name].get_content().to_ascii_lowercase(),
            );
        }
    }
    map
}

/// Eloquent query-builder methods that start or constrain a query when forwarded from a model (Mago's list).
const QUERY_BUILDER_METHODS: &[&str] = &[
    "where",
    "orWhere",
    "whereNot",
    "whereIn",
    "whereNotIn",
    "whereNull",
    "whereNotNull",
    "whereBetween",
    "whereNotBetween",
    "whereDate",
    "whereTime",
    "whereColumn",
    "whereExists",
    "whereHas",
    "whereDoesntHave",
    "whereRelation",
    "whereBelongsTo",
    "orderBy",
    "orderByDesc",
    "latest",
    "oldest",
    "inRandomOrder",
    "groupBy",
    "having",
    "with",
    "withCount",
    "withSum",
    "withMax",
    "withMin",
    "withAvg",
    "withWhereHas",
    "withTrashed",
    "withoutTrashed",
    "onlyTrashed",
    "has",
    "orHas",
    "doesntHave",
    "limit",
    "take",
    "skip",
    "offset",
    "distinct",
];

/// Laravel facade and helper classes whose static methods share query-builder names (Mago's list).
const KNOWN_NON_MODEL_CLASSES: &[&str] = &[
    "Arr",
    "Str",
    "Number",
    "Js",
    "Collection",
    "LazyCollection",
    "Fluent",
    "Stringable",
    "Uri",
    "DB",
    "Schema",
    "Cache",
    "Config",
    "Route",
    "View",
    "Log",
    "Http",
    "Storage",
    "File",
    "Validator",
    "Hash",
    "Auth",
    "Gate",
    "Event",
    "Queue",
    "Mail",
    "Notification",
    "Session",
    "Cookie",
    "Redirect",
    "Response",
    "Request",
    "URL",
    "App",
    "Artisan",
    "Blade",
    "Bus",
    "Date",
    "Lang",
    "Password",
    "Process",
    "RateLimiter",
    "Pipeline",
    "Context",
    "Crypt",
    "Broadcast",
    "Concurrency",
    "Vite",
];

#[cfg(test)]
mod tests {
    fn fix(code: &str) -> String {
        crate::fix_with(&*super::fixer(), code)
    }

    #[test]
    fn mago_cases() {
        // fix_inserts_query, static_where_on_model, static_with_on_model (count = 1)
        assert_eq!(
            fix("<?php\n\n$users = User::where('is_active', true)->get();\n"),
            "<?php\n\n$users = User::query()->where('is_active', true)->get();\n"
        );
        assert_eq!(
            fix("<?php\n\n$posts = Post::with('author')->latest()->get();\n"),
            "<?php\n\n$posts = Post::query()->with('author')->latest()->get();\n"
        );
        // explicit_query, finder_methods_allowed, support_helper_not_flagged
        for same in [
            "<?php\n\n$users = User::query()->where('is_active', true)->get();\n",
            "<?php\n\n$user = User::find(1);\n$first = User::create(['name' => 'Jane']);\n",
            "<?php\n\n$filtered = Arr::where($items, fn ($value) => $value !== null);\n",
        ] {
            assert_eq!(fix(same), same);
        }
    }

    #[test]
    fn edge_cases() {
        assert_eq!(
            fix("<?php \\App\\Models\\User::WhereIn('id', [1]);"),
            "<?php \\App\\Models\\User::query()->WhereIn('id', [1]);"
        );
        for same in [
            "<?php use Illuminate\\Support\\Facades\\DB as Database; Database::where('a');",
            "<?php \\Illuminate\\Support\\Facades\\Cache::has('k');",
            "<?php static::where('a'); self::with('b'); parent::latest();",
            "<?php $q->User::where('a');",
            "<?php User::$where; User::WHERE; User::find(1)->where('a');",
        ] {
            assert_eq!(fix(same), same);
        }
    }
}
