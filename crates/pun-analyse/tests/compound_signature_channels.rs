//! Source-owned ExtendedParametersAcceptor channels, captured directly from PHPStan 2.3.1.

use pun_phpstan::symbols::{PhpVersion, symbols};
use pun_phpstan::types::Level;
use serde_json::Value;

#[test]
fn builtin_return_channels_and_template_maps_match_the_original_reflection() {
    let rows: Value = serde_json::from_str(include_str!(
        "fixtures/compound-method-signatures/channels.json"
    ))
    .unwrap();
    for row in rows.as_array().unwrap() {
        let version = if row["version"] == 80400 {
            PhpVersion::Php84
        } else {
            PhpVersion::Php85
        };
        let class = row["class"].as_str().unwrap();
        let reflection = &symbols(version).classes[&class.to_ascii_lowercase()].methods
            [&row["method"].as_str().unwrap().to_ascii_lowercase()];
        let variant = &reflection.variants[row["variant"].as_u64().unwrap() as usize];
        let signature = variant.signature.as_ref().expect("source-owned channels");
        let describe =
            |ty: &str| pun_phpstan::resolve::resolve_str(ty, Some(class)).describe(Level::Precise);
        for (name, ty) in [
            ("effective", &variant.ret),
            ("phpdoc", &signature.phpdoc_return),
            ("native", &signature.native_return),
        ] {
            assert_eq!(describe(ty), row[name].as_str().unwrap(), "{row}: {name}");
        }
        for (name, entries) in [
            ("templates", &signature.templates),
            ("resolvedTemplates", &signature.resolved_templates),
            ("variances", &signature.variances),
        ] {
            let got: Vec<_> = entries
                .iter()
                .map(|(key, value)| {
                    (
                        key.clone(),
                        if name == "variances" {
                            value.clone()
                        } else {
                            describe(value)
                        },
                    )
                })
                .collect();
            let want: Vec<_> = row[name]
                .as_object()
                .into_iter()
                .flatten()
                .map(|(key, value)| (key.clone(), value.as_str().unwrap().to_string()))
                .collect();
            assert_eq!(got, want, "{row}: {name}");
        }
    }
}
