//! Canonical rule IDs select constructor and override owners independently despite their shared identifiers.
mod common;

use common::{PUN, clean_env, project};
use serde_json::Value;
use std::process::Command;

#[test]
fn constructor_diagnostics_follow_the_registered_owner_not_the_identifier_owner() {
    let dir = project(
        "named-registration",
        "constructors",
        &[(
            "constructor-only.php",
            include_str!(
                "../../pun-analyse/tests/fixtures/named-producer-gates/constructor-only.php"
            ),
        )],
    );
    let expected: Value = serde_json::from_str(include_str!(
        "../../pun-analyse/tests/fixtures/named-producer-gates/expected.json"
    ))
    .unwrap();
    for join in [true, false] {
        for overrides in [true, false] {
            for constructors in [true, false] {
                for calls in [true, false] {
                    let on = |b| if b { "on" } else { "off" };
                    std::fs::write(
                        dir.join("pun.toml"),
                        format!(
                            concat!(
                                "[php]\nversion = \"8.5\"\n",
                                "[analyse]\npreset = \"recommended\"\npaths = [\"constructor-only.php\"]\n",
                                "[analyse.rules]\n",
                                "\"analyse/classes/incompatible-override\" = \"{}\"\n",
                                "\"analyse/classes/inconsistent-constructor\" = \"{}\"\n",
                                "\"analyse/undefined/method\" = \"{}\"\n",
                                "\"analyse/types/renamed-parameter-in-subtype\" = \"{}\"\n",
                            ),
                            on(overrides), on(constructors), on(calls), on(join),
                        ),
                    )
                    .unwrap();
                    let output = clean_env(&mut Command::new(PUN), &["PATH", "HOME"])
                        .current_dir(&dir)
                        .env("PUN_AGENT", "0")
                        .env("RAYON_NUM_THREADS", "1")
                        .args(["analyse", "--format=json"])
                        .output()
                        .unwrap();
                    assert!(output.stderr.is_empty(), "{output:?}");
                    let result: Value = serde_json::from_slice(&output.stdout).unwrap();
                    assert_eq!(result["errors"], serde_json::json!([]));
                    let root = dir.to_str().unwrap();
                    let mut got: Vec<_> = result["files"]
                        .as_object()
                        .unwrap()
                        .iter()
                        .flat_map(|(file, data)| {
                            data["messages"].as_array().unwrap().iter().map(move |e| {
                                let mut e = e.clone();
                                e["file"] = file.replace(root, "$ROOT").into();
                                e
                            })
                        })
                        .collect();
                    let case = if constructors {
                        if calls && join {
                            "constructor-all"
                        } else {
                            "constructor-no-join"
                        }
                    } else {
                        "constructor-no-constructors"
                    };
                    let mut want = expected[case].as_array().unwrap().clone();
                    got.sort_by_key(Value::to_string);
                    want.sort_by_key(Value::to_string);
                    assert_eq!(
                        got, want,
                        "overrides={overrides}, constructors={constructors}, calls={calls}, join={join}"
                    );
                    assert_eq!(output.status.code(), Some(i32::from(constructors)));
                }
            }
        }
    }
    std::fs::remove_dir_all(dir).unwrap();
}
