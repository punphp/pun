//! `@phpstan-ignore*` comment parsing and `ignoreErrors` matching against PHPStan 2.2.7's own classes
//! (tests/fixtures/comments.json and ignore.json, exported by scripts/export-ignore-fixtures.php).

mod common;

use common::{fixture, placeholders, tagged, untagged};
use pun_phpstan::comments::{self, LinesToIgnore};
use pun_phpstan::ignore::{Error, IgnoredErrors};
use pun_phpstan::{Array, Key, Value};
use serde_json::{Value as J, json};

fn lines_value(lines: &LinesToIgnore) -> Value {
    let mut a = Array::new();
    for (&line, ids) in lines {
        let v = match ids {
            None => Value::Null,
            Some(ids) => Value::Array(Array::list(ids.iter().map(|i| {
                let mut e = Array::new();
                e.set("name".into(), Value::str(&i.name));
                e.set(
                    "comment".into(),
                    i.comment.as_deref().map_or(Value::Null, Value::str),
                );
                Value::Array(e)
            }))),
        };
        a.set(Key::Int(line), v);
    }
    Value::Array(a)
}

#[test]
fn rich_parser_lines_to_ignore() {
    let mut failures = Vec::new();
    for case in fixture("comments.json").as_array().unwrap() {
        let (lines, errors) = comments::lines_to_ignore(case["code"].as_str().unwrap().as_bytes());
        let mut e = Array::new();
        for (&line, messages) in &errors {
            e.set(
                Key::Int(line),
                Value::Array(Array::list(messages.iter().map(Value::str))),
            );
        }
        let got =
            json!({ "lines": tagged(&lines_value(&lines)), "errors": tagged(&Value::Array(e)) });
        let want = json!({ "lines": case["lines"], "errors": case["errors"] });
        if got != want {
            failures.push(format!("{:?}\n  want {want}\n  got  {got}", case["code"]));
        }
    }
    assert!(
        failures.is_empty(),
        "{} differ:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

fn opt(j: &J) -> Option<String> {
    j.as_str().map(String::from)
}

fn error(j: &J) -> Error {
    Error {
        message: j["message"].as_str().unwrap().into(),
        file: j["file"].as_str().unwrap().into(),
        line: j["line"].as_i64(),
        can_be_ignored: j["canBeIgnored"].as_bool().unwrap(),
        file_path: j["filePath"].as_str().unwrap().into(),
        trait_file_path: opt(&j["traitFilePath"]),
        tip: opt(&j["tip"]),
        identifier: opt(&j["identifier"]),
    }
}

fn dump(e: &Error) -> J {
    json!({
        "message": e.message, "file": e.file, "line": e.line, "canBeIgnored": e.can_be_ignored,
        "filePath": e.file_path, "traitFilePath": e.trait_file_path, "tip": e.tip, "identifier": e.identifier,
    })
}

#[test]
fn ignored_error_helper() {
    let dir = format!("{}/tests/fixtures/ignore/files", env!("CARGO_MANIFEST_DIR"));
    let to_real = [("{dir}", dir.as_str())];
    let to_placeholder = [(dir.as_str(), "{dir}")];
    let mut failures = Vec::new();
    for case in fixture("ignore.json").as_array().unwrap() {
        let case = placeholders(case.clone(), &to_real);
        let Value::Array(entries) = untagged(&case["ignoreErrors"]) else {
            panic!()
        };
        let helper = IgnoredErrors::new(&entries, case["reportUnmatched"].as_bool().unwrap());
        let analysed: Vec<String> = case["analysedFiles"]
            .as_array()
            .unwrap()
            .iter()
            .map(|f| f.as_str().unwrap().into())
            .collect();
        let result = helper.process(
            case["errors"]
                .as_array()
                .unwrap()
                .iter()
                .map(error)
                .collect(),
            case["onlyFiles"].as_bool().unwrap(),
            &analysed,
            case["hasInternalErrors"].as_bool().unwrap(),
        );
        let got = json!({
            "configErrors": helper.errors,
            "errors": result.errors.iter().map(dump).collect::<Vec<_>>(),
            "ignored": result.ignored.iter().map(|(e, i)| json!([dump(e), tagged(i)])).collect::<Vec<_>>(),
            "stringErrors": result.string_errors,
        });
        let got = placeholders(got, &to_placeholder);
        let want = placeholders(case["expected"].clone(), &to_placeholder);
        if got != want {
            failures.push(format!(
                "{}\n  want {want}\n  got  {got}",
                case["ignoreErrors"]
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "{} differ:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

/// The comment pipeline of `FileAnalyser` + `AnalyserResultFinalizer` against a real `phpstan analyse` run
/// (tests/fixtures/local.json, scripts/export-local-ignore-fixtures.php): the raw errors come from the same files with
/// the comments blanked out.
#[test]
fn phpstan_analyse_local_ignores() {
    let dir = format!("{}/tests/fixtures/local", env!("CARGO_MANIFEST_DIR"));
    let key = |j: &J| {
        let s = |k: &str| j.get(k).map(J::to_string).unwrap_or_default();
        (
            j["line"].as_i64(),
            j["message"].as_str().unwrap().to_string(),
            s("ignorable"),
            s("identifier"),
            s("tip"),
        )
    };
    let mut failures = Vec::new();
    let all = fixture("local.json");
    for (name, data) in all.as_object().unwrap() {
        let file = format!("{dir}/{name}");
        let code = std::fs::read(&file).unwrap();
        for (without_comments, reported) in [(false, "reported"), (true, "reportedWithoutComments")]
        {
            let (mut lines, parse) = comments::lines_to_ignore(&code);
            let mut unmatched = lines.clone();
            let mut errors = pun_phpstan::ignore::parse_errors(&file, &parse);
            let mut raw: Vec<Error> = data["raw"]
                .as_array()
                .unwrap()
                .iter()
                .map(|m| Error {
                    can_be_ignored: m["ignorable"].as_bool().unwrap(),
                    tip: opt(&m["tip"]),
                    identifier: opt(&m["identifier"]),
                    ..Error::new(m["message"].as_str().unwrap(), &file, m["line"].as_i64())
                })
                .collect();
            raw.append(&mut errors);
            let mut out = Vec::new();
            if without_comments {
                out.extend(pun_phpstan::ignore::ignores_without_comments(
                    &file,
                    &mut lines,
                    &mut unmatched,
                ));
            }
            let (kept, _) =
                pun_phpstan::ignore::local_ignores(&file, raw, &mut lines, &mut unmatched);
            out.extend(kept);
            out.extend(pun_phpstan::ignore::unmatched_line_errors(
                &file, &unmatched,
            ));
            let mut got: Vec<_> = out
                .iter()
                .map(|e| {
                    let mut j = json!({ "line": e.line, "message": e.message, "ignorable": e.can_be_ignored });
                    if let Some(id) = &e.identifier {
                        j["identifier"] = json!(id);
                    }
                    if let Some(tip) = &e.tip {
                        j["tip"] = json!(tip);
                    }
                    key(&j)
                })
                .collect();
            let mut want: Vec<_> = data[reported].as_array().unwrap().iter().map(key).collect();
            got.sort();
            want.sort();
            if got != want {
                failures.push(format!(
                    "{name} {reported}\n  want {want:?}\n  got  {got:?}"
                ));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "{} differ:\n{}",
        failures.len(),
        failures.join("\n")
    );
}
