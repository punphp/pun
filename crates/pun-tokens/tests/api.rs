//! Tokens / Token / TokensAnalyzer API parity with PHP-CS-Fixer: every case in api_cases.json is run here and
//! through tools/api.php (the real classes); the PHP results are the expectations. Skipped without php or the
//! .oracle/php-cs-fixer checkout (or PUN_PHP_CS_FIXER).
//!
//! A case is `{"code": "...", "ops": [[op, args...], ...]}`; `["code"]` and `["changed"]` are appended to every
//! case. `["each", op, args...]` runs `op` with every token index as first argument. Protos/tokens: `"x"` (plain
//! token), `["T_STRING"]` (kind), `["T_STRING", "foo"]` (kind + content); kinds by `token_name()` / `CT::T_*`.
use pun_tokens::tokens_analyzer::TokensAnalyzer;
use pun_tokens::{BlockType, FoundKey, Kind, Proto, Token, Tokens};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::io::Write;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::Path;
use std::process::{Command, Stdio};

fn kind(v: &Value) -> Kind {
    let n = v.as_str().unwrap();
    Kind::from_name(n).unwrap_or_else(|| panic!("unknown kind {n}"))
}
fn proto(v: &Value) -> Proto<'_> {
    match v {
        Value::String(s) => Proto::Char(s.as_bytes()),
        Value::Array(a) if a.len() == 1 => Proto::Kind(kind(&a[0])),
        Value::Array(a) => Proto::KindContent(kind(&a[0]), a[1].as_str().unwrap().as_bytes()),
        _ => panic!("bad proto {v}"),
    }
}
fn protos(v: &Value) -> Vec<Proto<'_>> {
    v.as_array().unwrap().iter().map(proto).collect()
}
fn tok(v: &Value) -> Token {
    match proto(v) {
        Proto::Char(c) => Token::char(c),
        Proto::KindContent(k, c) => Token::new(k, c),
        Proto::Kind(_) => panic!("token needs content"),
    }
}
fn toks(v: &Value) -> Vec<Token> {
    v.as_array().unwrap().iter().map(tok).collect()
}
fn key(v: &Value) -> FoundKey<'_> {
    match v {
        Value::String(s) => s.as_str().into(),
        _ => kind(&v[0]).into(),
    }
}
fn kinds(v: &Value) -> Vec<Kind> {
    v.as_array().unwrap().iter().map(kind).collect()
}
fn s(b: &[u8]) -> Value {
    Value::String(String::from_utf8_lossy(b).into_owned())
}
fn opt_ws(v: Option<&Value>) -> Option<&[u8]> {
    v.and_then(Value::as_str).map(str::as_bytes)
}

fn op(t: &mut Tokens, o: &[Value]) -> Value {
    let name = o[0].as_str().unwrap();
    let x = &o[1..];
    let u = |i: usize| x[i].as_u64().unwrap() as usize;
    let d = |i: usize| x[i].as_i64().unwrap() as isize;
    let b = |i: usize, default: bool| x.get(i).and_then(Value::as_bool).unwrap_or(default);
    let end = |i: usize| x.get(i).and_then(Value::as_u64).map(|e| e as usize);
    let a = TokensAnalyzer::new(t);

    match name {
        "code" => s(&t.generate_code()),
        "contents" => json!(t.iter().map(|k| s(k.get_content())).collect::<Vec<_>>()),
        "count" => json!([
            t.count(),
            t.len(),
            t.is_empty(),
            t.as_slice().len(),
            (&*t).into_iter().count()
        ]),
        "changed" => json!(t.is_changed()),
        "partial" => s(&t.generate_partial_code(u(0), u(1))),
        "detect" => {
            json!(Tokens::detect_block_type(&t[u(0)]).map(|(bt, start)| json!([bt as u8, start])))
        }
        "blocks" => json!(Tokens::detect_block_type(&t[u(0)]).map(|(bt, start)| {
            if start {
                t.find_block_end(bt, u(0))
            } else {
                t.find_block_start(bt, u(0))
            }
        })),
        // Upstream throws on an unknown type (0, 16); `BlockType::ALL` has none, so `[..]` panics there.
        "blockEnd" => json!(t.find_block_end(BlockType::ALL[u(0).wrapping_sub(1)], u(1))),
        "blockStart" => json!(t.find_block_start(BlockType::ALL[u(0).wrapping_sub(1)], u(1))),
        "givenKind" => {
            let start = x.get(1).and_then(Value::as_u64).unwrap_or(0) as usize;
            if x[0].is_string() {
                json!(t.find_given_kind(kind(&x[0]), start, end(2)))
            } else {
                let m = t.find_given_kinds(&kinds(&x[0]), start, end(2));
                json!(
                    m.into_iter()
                        .map(|(k, v)| (k.name().to_string(), v))
                        .collect::<BTreeMap<_, _>>()
                )
            }
        }
        "nextNonWs" => json!(t.get_next_non_whitespace(u(0), opt_ws(x.get(1)))),
        "prevNonWs" => json!(t.get_prev_non_whitespace(u(0), opt_ws(x.get(1)))),
        "nonWsSibling" => json!(t.get_non_whitespace_sibling(u(0), d(1), opt_ws(x.get(2)))),
        "nextOfKind" => json!(t.get_next_token_of_kind(u(0), &protos(&x[1]), b(2, true))),
        "prevOfKind" => json!(t.get_prev_token_of_kind(u(0), &protos(&x[1]), b(2, true))),
        "ofKindSibling" => {
            json!(t.get_token_of_kind_sibling(u(0), d(1), &protos(&x[2]), b(3, true)))
        }
        "notOfKindSibling" => json!(t.get_token_not_of_kind_sibling(u(0), d(1), &protos(&x[2]))),
        "notOfKindsSibling" => json!(t.get_token_not_of_kinds_sibling(u(0), d(1), &kinds(&x[2]))),
        "meaningfulSibling" => json!(t.get_meaningful_token_sibling(u(0), d(1))),
        "nonEmptySibling" => json!(t.get_non_empty_sibling(u(0), d(1))),
        "nextMeaningful" => json!(t.get_next_meaningful_token(u(0))),
        "prevMeaningful" => json!(t.get_prev_meaningful_token(u(0))),
        "sequence" => {
            let start = x.get(1).and_then(Value::as_u64).unwrap_or(0) as usize;
            let cs = x.get(3).cloned().unwrap_or(json!(true));
            let f = |k: usize| match &cs {
                Value::Object(m) => m
                    .get(&k.to_string())
                    .and_then(Value::as_bool)
                    .unwrap_or(true),
                v => v.as_bool().unwrap(),
            };
            json!(t.find_sequence(&protos(&x[0]), start, end(2), f))
        }
        "emptyAt" => json!(t.is_empty_at(u(0))),
        "allFound" => json!(t.is_all_token_kinds_found(&kinds(&x[0]))),
        "anyFound" => json!(t.is_any_token_kinds_found(&kinds(&x[0]))),
        "found" => json!(t.is_token_kind_found(key(&x[0]))),
        "countKind" => json!(t.count_token_kind(key(&x[0]))),
        "monolithic" => json!(t.is_monolithic_php()),
        "multiline" => json!(t.is_partial_code_multiline(u(0), u(1))),
        "altSyntax" => json!(t.has_alternative_syntax()),
        "tok" => {
            let k = &t[u(0)];
            json!([
                k.get_name(),
                k.get_id().map(Kind::name),
                s(k.get_content()),
                k.is_array(),
                k.is_cast(),
                k.is_classy(),
                k.is_comment(),
                k.is_object_operator(),
                k.is_keyword(),
                k.is_magic_constant(),
                k.is_native_constant(),
                k.is_whitespace(None)
            ])
        }
        "isWs" => json!(t[u(0)].is_whitespace(opt_ws(x.get(1)))),
        "equals" => json!(t[u(0)].equals(proto(&x[1]), b(2, true))),
        "equalsProto" => json!(t[u(0)].equals(&t[u(1)], b(2, true))),
        "equalsToken" => json!(t[u(0)].equals_token(&t[u(1)], b(2, true))),
        "newEquals" => json!(tok(&x[0]).equals(proto(&x[1]), b(2, true))),
        "newEqualsToken" => json!(tok(&x[0]).equals_token(&tok(&x[1]), b(2, true))),
        "equalsAny" => json!(t[u(0)].equals_any(&protos(&x[1]), b(2, true))),
        "isGivenKind" => json!(if x[1].is_string() {
            t[u(0)].is_kind(kind(&x[1]))
        } else {
            t[u(0)].is_given_kind(&kinds(&x[1]))
        }),
        "classyElements" => {
            json!(
                a.get_classy_elements()
                    .iter()
                    .map(|(i, e)| json!([i, e.kind.as_str(), e.class_index]))
                    .collect::<Vec<_>>()
            )
        }
        "classyModifiers" => {
            let m = a.get_classy_modifiers(u(0));
            json!([m.final_, m.abstract_, m.readonly])
        }
        "importUses" => {
            let m = a.get_import_use_indexes(b(0, false));
            if b(0, false) {
                json!(
                    m.into_iter()
                        .map(|(k, v)| (k.to_string(), v))
                        .collect::<BTreeMap<_, _>>()
                )
            } else {
                json!(m.get(&0).cloned().unwrap_or_default())
            }
        }
        "isArray" => json!(a.is_array(u(0))),
        "arrayMultiLine" => json!(a.is_array_multi_line(u(0))),
        "blockMultiline" => json!(a.is_block_multiline(u(0))),
        "methodAttributes" => {
            let m = a.get_method_attributes(u(0));
            json!([
                m.visibility.map(Kind::name),
                m.static_,
                m.abstract_,
                m.final_
            ])
        }
        "anonClass" => json!(a.is_anonymous_class(u(0))),
        "lambda" => json!(a.is_lambda(u(0))),
        "arrowEnd" => json!(a.get_last_token_index_of_arrow_function(u(0))),
        "unarySucc" => json!(a.is_unary_successor_operator(u(0))),
        "unaryPred" => json!(a.is_unary_predecessor_operator(u(0))),
        "binary" => json!(a.is_binary_operator(u(0))),
        "doWhile" => json!(a.is_while_part_of_do_while(u(0))),
        "enumCase" => json!(a.is_enum_case(u(0))),
        "superGlobal" => json!(a.is_super_global(u(0))),
        // mutations
        _ => {
            match name {
                "clearChanged" => t.clear_changed(),
                "insertAt" => t.insert_at(u(0), toks(&x[1])),
                "insertSlices" => t.insert_slices(
                    x[0].as_object()
                        .unwrap()
                        .iter()
                        .map(|(i, l)| (i.parse().unwrap(), toks(l)))
                        .collect(),
                ),
                "clearAt" => t.clear_at(u(0)),
                "clearMerge" => t.clear_token_and_merge_surrounding_whitespace(u(0)),
                "set" => t.set(u(0), tok(&x[1])),
                "overrideRange" => t.override_range(u(0), u(1), toks(&x[2])),
                "removeLeadingWs" => t.remove_leading_whitespace(u(0), opt_ws(x.get(1))),
                "removeTrailingWs" => t.remove_trailing_whitespace(u(0), opt_ws(x.get(1))),
                "setCode" => t.set_code(x[0].as_str().unwrap().as_bytes()),
                "transform" => pun_tokens::transformers::transform(t),
                "fromArray" => *t = Tokens::from_array(toks(&x[0])),
                "clearRange" => t.clear_range(u(0), u(1)),
                "ensureWs" => {
                    return json!(t.ensure_whitespace_at_index(
                        u(0),
                        u(1),
                        x[2].as_str().unwrap().as_bytes()
                    ));
                }
                "clearEmpty" => t.clear_empty_tokens(),
                _ => panic!("unknown op {name}"),
            }
            Value::Null
        }
    }
}

fn guarded(t: &mut Tokens, o: &[Value]) -> Value {
    catch_unwind(AssertUnwindSafe(|| op(t, o))).unwrap_or_else(|_| json!({"throws": true}))
}

/// The case's ops plus the implicit `["code"]`, `["changed"]` (unless `"noTail": true`).
fn ops_of(case: &Value) -> Vec<Value> {
    let mut ops = case["ops"].as_array().unwrap().clone();
    if case.get("noTail").is_none() {
        ops.extend([json!(["code"]), json!(["changed"])]);
    }
    ops
}

fn run_case(case: &Value) -> Vec<Value> {
    let mut t = Tokens::from_code(case["code"].as_str().unwrap().as_bytes());
    ops_of(case)
        .iter()
        .map(|o| {
            let o = o.as_array().unwrap();
            if o[0] == "each" {
                let n = t.count();
                json!(
                    (0..n)
                        .map(|i| {
                            let mut args = vec![o[1].clone(), json!(i)];
                            args.extend_from_slice(&o[2..]);
                            guarded(&mut t, &args)
                        })
                        .collect::<Vec<_>>()
                )
            } else {
                guarded(&mut t, o)
            }
        })
        .collect()
}

#[test]
fn api_matches_php_cs_fixer() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let oracle = root.join("../../.oracle/php-cs-fixer/vendor/autoload.php");
    if std::env::var("PUN_PHP_CS_FIXER").is_err() && !oracle.exists() {
        return;
    }
    let cases_json = std::fs::read_to_string(root.join("tests/api_cases.json")).unwrap();
    let Ok(mut child) = Command::new("php")
        .arg(root.join("tools/api.php"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    else {
        return;
    };
    child
        .stdin
        .take()
        .unwrap()
        .write_all(cases_json.as_bytes())
        .unwrap();
    let out = child.wait_with_output().unwrap();
    assert!(out.status.success(), "tools/api.php failed");
    let expected: Vec<Vec<Value>> = String::from_utf8(out.stdout)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    let cases: Vec<Value> = serde_json::from_str(&cases_json).unwrap();
    assert_eq!(expected.len(), cases.len());
    std::panic::set_hook(Box::new(|_| {}));
    let mut bad = Vec::new();
    for (n, (case, want)) in cases.iter().zip(&expected).enumerate() {
        let got = run_case(case);
        for ((o, g), w) in ops_of(case).iter().zip(&got).zip(want) {
            if g != w {
                bad.push(format!(
                    "case {n} {:?} {o}\n  php:  {w}\n  ours: {g}",
                    case["code"].as_str().unwrap()
                ));
            }
        }
    }
    let _ = std::panic::take_hook();
    assert!(
        bad.is_empty(),
        "{} mismatches:\n{}",
        bad.len(),
        bad.join("\n")
    );
}
