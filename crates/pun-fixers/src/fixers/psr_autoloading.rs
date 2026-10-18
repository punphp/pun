//! Port of `PhpCsFixer\Fixer\Basic\PsrAutoloadingFixer`.
//!
//! `SplFileInfo::getRealPath()` is `std::fs::canonicalize`. Upstream passes its `false` (a file that does not exist)
//! on to `Preg::match`, which throws under strict types; Pun does not support such a file, as it does not support
//! `StdinFileInfo`.
use crate::analyzers::docblock::type_expression::REGEX_IDENTIFIER;
use crate::analyzers::fixer_configuration_resolver::resolve;
use crate::analyzers::preg::Preg;
use anyhow::{Result, bail};
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::php_compat::strtolower;
use pun_tokens::tokens_analyzer::TokensAnalyzer;
use pun_tokens::{Token, Tokens};
use serde_json::Value;
use std::path::Path;

struct PsrAutoloadingFixer {
    dir: Option<String>,
}

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(PsrAutoloadingFixer { dir: None })
}

fn real_path(path: &Path) -> Option<String> {
    std::fs::canonicalize(path)
        .ok()
        .map(|p| p.to_string_lossy().into_owned())
}

/// PHP `dirname` for an absolute path.
fn dirname(path: &str) -> &str {
    &path[..path.rfind('/').unwrap().max(1)]
}

/// PHP `substr($s, $start)` for a non-negative start.
fn substr_from(s: &str, start: usize) -> &str {
    s.get(start..).unwrap_or("")
}

/// PHP `substr($s, -$len)`.
fn substr_tail(s: &str, len: usize) -> &str {
    &s[s.len().saturating_sub(len)..]
}

fn basename_php(path: &Path) -> String {
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    // `supports` only accepts a `.php` extension, so the suffix is always there.
    name.strip_suffix(".php").unwrap().to_string()
}

impl Fixer for PsrAutoloadingFixer {
    fn name(&self) -> &'static str {
        "psr_autoloading"
    }
    fn priority(&self) -> i32 {
        -10
    }
    fn is_risky(&self) -> bool {
        true
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_any_token_kinds_found(&[T_CLASS, T_TRAIT, T_INTERFACE, T_ENUM])
    }
    fn supports(&self, file: &Path) -> bool {
        // ignore file with extension other than php
        if file.extension().is_none_or(|e| e != "php") {
            return false;
        }
        // ignore file with name that cannot be a class name
        let name = basename_php(file);
        if !Preg::r#match(format!("/^{REGEX_IDENTIFIER}$/"), name.as_bytes()) {
            return false;
        }
        // `Tokens::fromCode` throws a ParseError on a reserved word; its token is a keyword or a magic constant either
        // way, so the raw token answers both checks (Pun's transformers expect code that parses).
        let code = format!("<?php class {name} {{}}");
        let raw = &pun_tokens::lexer::tokenize(code.as_bytes())[3];
        // An identifier always lexes as a word token, never a single character.
        let token = Token::new(raw.kind.unwrap(), raw.text);
        if token.is_keyword() || token.is_magic_constant() {
            return false;
        }
        let Some(real) = real_path(file) else {
            return false;
        };
        // ignore stubs/fixtures, since they typically contain invalid files for various reasons
        !Preg::r#match(r"{[/\\](stub|fixture)s?[/\\]}i", real.as_bytes())
    }
    fn configure(&mut self, options: &Value) -> Result<()> {
        let o = resolve(self.name(), options, &["dir"])?;
        self.dir = match o.get("dir") {
            None | Some(Value::Null) => None,
            Some(Value::String(dir)) => match real_path(Path::new(dir)) {
                Some(real) => Some(real),
                None => bail!("Failed to resolve configured directory \"{dir}\"."),
            },
            Some(v) => return Err(o.type_error("dir", v, "\"null\" or \"string\"")),
        };
        Ok(())
    }
    fn fix(&self, file: &Path, tokens: &mut Tokens) {
        let real = real_path(file).unwrap();
        if let Some(dir) = &self.dir
            && !real.starts_with(dir.as_str())
        {
            return;
        }
        let mut namespace: Option<(String, usize, usize)> = None;
        let mut classy: Option<(Vec<u8>, usize)> = None;
        for index in 0..tokens.count() {
            if tokens[index].is_kind(T_NAMESPACE) {
                if namespace.is_some() {
                    return;
                }
                let start = tokens.get_next_meaningful_token(index).unwrap();
                let end = tokens
                    .get_next_token_of_kind(start, &[";".into()], true)
                    .unwrap();
                let code = tokens.generate_partial_code(start, end - 1);
                let name = String::from_utf8_lossy(&code)
                    .trim_matches(TRIM)
                    .to_string();
                namespace = Some((name, start, end));
            } else if tokens[index].is_classy() {
                if TokensAnalyzer::new(tokens).is_anonymous_class(index) {
                    continue;
                }
                if classy.is_some() {
                    return;
                }
                let classy_index = tokens.get_next_meaningful_token(index).unwrap();
                classy = Some((tokens[classy_index].content.to_vec(), classy_index));
            }
        }
        let Some((classy_name, classy_index)) = classy else {
            return;
        };
        let ns = namespace.as_ref().map(|(n, _, _)| n.as_str());
        let expected =
            self.calculate_classy_name(&real, file, ns, &String::from_utf8_lossy(&classy_name));
        if classy_name != expected.as_bytes() {
            tokens.set(classy_index, Token::new(T_STRING, expected));
        }
        let (Some(dir), Some((namespace, start, end))) = (&self.dir, namespace) else {
            return;
        };
        if !Path::new(dir).is_dir() {
            return;
        }
        let file_dir = dirname(&real);
        if dir.len() >= file_dir.len() {
            return;
        }
        let new_namespace = substr_from(&file_dir.replace('/', "\\"), dir.len() + 1).to_string();
        let original_namespace = substr_tail(&namespace, new_namespace.len());
        if original_namespace != new_namespace
            && original_namespace.to_ascii_lowercase() == new_namespace.to_ascii_lowercase()
        {
            tokens.clear_range(start, end);
            let namespace = format!(
                "{}{}",
                &namespace[..namespace.len() - new_namespace.len()],
                new_namespace
            );
            let mut new_tokens =
                Tokens::from_code(format!("<?php namespace {namespace};").as_bytes());
            new_tokens.clear_range(0, 2);
            new_tokens.clear_empty_tokens();
            tokens.insert_at(start, new_tokens.iter().cloned().collect());
        }
    }
}

/// PHP `trim`'s default characters.
const TRIM: &[char] = &[' ', '\t', '\n', '\r', '\0', '\x0B'];

impl PsrAutoloadingFixer {
    fn calculate_classy_name(
        &self,
        real: &str,
        file: &Path,
        namespace: Option<&str>,
        current_name: &str,
    ) -> String {
        let mut name = basename_php(file);
        let max_namespace = self.calculate_max_namespace(real, namespace);
        if self.dir.is_some() {
            return if max_namespace.is_empty() {
                name
            } else {
                format!("{}_{name}", max_namespace.replace('\\', "_"))
            };
        }
        for namespace_part in max_namespace.split('\\').rev() {
            let name_candidate = format!("{namespace_part}_{name}");
            let tail = substr_tail(current_name, name_candidate.len());
            if strtolower(name_candidate.as_bytes()) != strtolower(tail.as_bytes()) {
                break;
            }
            name = name_candidate;
        }
        name
    }

    fn calculate_max_namespace(&self, real: &str, namespace: Option<&str>) -> String {
        let root = match &self.dir {
            None => "/".to_string(),
            Some(dir) => dir.clone(),
        };
        let location = substr_from(dirname(real), root.len())
            .replace('/', "\\")
            .trim_matches('\\')
            .to_string();
        let Some(namespace) = namespace else {
            return location;
        };
        let mut location_parts_reversed: Vec<&str> = location.split('\\').rev().collect();
        let mut removed = 0;
        for (key, namespace_part) in namespace.split('\\').rev().enumerate() {
            let Some(part) = location_parts_reversed.get(key) else {
                break;
            };
            if strtolower(namespace_part.as_bytes()) != strtolower(part.as_bytes()) {
                break;
            }
            removed = key + 1;
        }
        location_parts_reversed.drain(..removed);
        location_parts_reversed.reverse();
        location_parts_reversed.join("\\")
    }
}
