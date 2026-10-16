//! The PHP files PHPStan config loading meets that need no PHP runtime: `phpstan-baseline.php` (and other configs that
//! only build and `return` a literal array) and phpstan/extension-installer's `GeneratedConfig.php`.

use crate::value::{Array, Key, Value};

struct P<'a> {
    s: &'a str,
    i: usize,
    dir: &'a str,
    vars: Vec<(String, Value)>,
}

impl P<'_> {
    fn skip(&mut self) {
        loop {
            let rest = &self.s[self.i..];
            let trimmed = rest.trim_start();
            self.i += rest.len() - trimmed.len();
            if trimmed.starts_with("//") || trimmed.starts_with('#') {
                self.i += trimmed.find('\n').unwrap_or(trimmed.len());
            } else if trimmed.starts_with("/*") {
                self.i += trimmed.find("*/").map_or(trimmed.len(), |e| e + 2);
            } else {
                return;
            }
        }
    }

    fn eat(&mut self, t: &str) -> bool {
        self.skip();
        let rest = &self.s[self.i..];
        let word = t.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
        let matches = if word {
            rest.len() >= t.len()
                && rest[..t.len()].eq_ignore_ascii_case(t)
                && !rest[t.len()..].starts_with(|c: char| c.is_ascii_alphanumeric() || c == '_')
        } else {
            rest.starts_with(t)
        };
        if matches {
            self.i += t.len();
        }
        matches
    }

    fn expect(&mut self, t: &str) -> Result<(), String> {
        if self.eat(t) {
            Ok(())
        } else {
            Err(self.unexpected())
        }
    }

    fn unexpected(&self) -> String {
        let line = self.s[..self.i].matches('\n').count() + 1;
        let near: String = self.s[self.i..].chars().take(20).collect();
        format!("unexpected {near:?} on line {line}")
    }

    fn var_name(&mut self) -> Result<String, String> {
        self.expect("$")?;
        let n: String = self.s[self.i..]
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
            .collect();
        self.i += n.len();
        Ok(n)
    }

    fn expr(&mut self) -> Result<Value, String> {
        let mut v = self.term()?;
        while self.eat(".") {
            let r = self.term()?;
            v = Value::Str(
                v.to_php_string().unwrap_or_default() + &r.to_php_string().unwrap_or_default(),
            );
        }
        Ok(v)
    }

    fn term(&mut self) -> Result<Value, String> {
        self.skip();
        let rest = &self.s[self.i..];
        if rest.starts_with('\'') {
            let mut out = String::new();
            let mut chars = rest.char_indices().skip(1);
            while let Some((j, c)) = chars.next() {
                match c {
                    '\\' => match chars.next() {
                        Some((_, n @ ('\\' | '\''))) => out.push(n),
                        Some((_, n)) => {
                            out.push('\\');
                            out.push(n);
                        }
                        None => break,
                    },
                    '\'' => {
                        self.i += j + 1;
                        return Ok(Value::Str(out));
                    }
                    c => out.push(c),
                }
            }
            return Err("unterminated string".into());
        }
        if self.eat("__DIR__") {
            return Ok(Value::str(self.dir));
        }
        if self.eat("NULL") {
            return Ok(Value::Null);
        }
        if self.eat("true") {
            return Ok(Value::Bool(true));
        }
        if self.eat("false") {
            return Ok(Value::Bool(false));
        }
        if self.eat("array") {
            self.expect("(")?;
            return self.items(")");
        }
        if self.eat("[") {
            return self.items("]");
        }
        if rest.starts_with('$') {
            let name = self.var_name()?;
            return self
                .vars
                .iter()
                .rev()
                .find(|(n, _)| *n == name)
                .map(|(_, v)| v.clone())
                .ok_or_else(|| format!("undefined ${name}"));
        }
        let num: String = rest
            .chars()
            .take_while(|c| c.is_ascii_digit() || *c == '-')
            .collect();
        // (`1.5` is a float, not `1 . 5`: floats are not supported)
        let float = rest[num.len()..].starts_with('.')
            && rest[num.len() + 1..].starts_with(|c: char| c.is_ascii_digit());
        if let (false, Ok(n)) = (float, num.parse::<i64>()) {
            self.i += num.len();
            return Ok(Value::Int(n));
        }
        Err(self.unexpected())
    }

    fn items(&mut self, close: &str) -> Result<Value, String> {
        let mut a = Array::new();
        loop {
            if self.eat(close) {
                return Ok(Value::Array(a));
            }
            let first = self.expr()?;
            if self.eat("=>") {
                let v = self.expr()?;
                let k = match first {
                    Value::Int(i) => Key::Int(i),
                    k => Key::from_string(k.to_php_string().unwrap_or_default()),
                };
                a.set(k, v);
            } else {
                a.push(first);
            }
            if !self.eat(",") {
                self.expect(close)?;
                return Ok(Value::Array(a));
            }
        }
    }
}

/// `require $file` for a config that only assigns literals to variables, appends to them and returns an array
/// (the shape of PHPStan's `--generate-baseline=*.php`). `__DIR__` is `dir`.
pub fn load_config(php: &str, dir: &str) -> Result<Array, String> {
    let mut p = P {
        s: php,
        i: 0,
        dir,
        vars: Vec::new(),
    };
    p.expect("<?php")?;
    loop {
        if p.eat("declare") {
            let end = p.s[p.i..].find(';').ok_or_else(|| p.unexpected())?;
            p.i += end + 1;
        } else if p.eat("return") {
            let v = p.expr()?;
            return match v {
                Value::Array(a) => Ok(a),
                _ => Err("the config does not return an array".into()),
            };
        } else {
            p.skip();
            if !p.s[p.i..].starts_with('$') {
                return Err(p.unexpected());
            }
            let name = p.var_name()?;
            let push = p.eat("[");
            if push {
                p.expect("]")?;
            }
            p.expect("=")?;
            let v = p.expr()?;
            p.expect(";")?;
            if push {
                let Some((_, Value::Array(a))) = p.vars.iter_mut().rev().find(|(n, _)| *n == name)
                else {
                    return Err(format!("${name} is not an array"));
                };
                a.push(v);
            } else {
                p.vars.push((name, v));
            }
        }
    }
}

/// `PHPStan\ExtensionInstaller\GeneratedConfig::EXTENSIONS` from the generated file.
pub fn generated_config_extensions(php: &str, dir: &str) -> Result<Array, String> {
    let at = php
        .find("const EXTENSIONS")
        .ok_or("no EXTENSIONS constant")?;
    let eq = php[at..].find('=').ok_or("no EXTENSIONS value")? + at + 1;
    let mut p = P {
        s: php,
        i: eq,
        dir,
        vars: Vec::new(),
    };
    match p.expr()? {
        Value::Array(a) => Ok(a),
        _ => Err("EXTENSIONS is not an array".into()),
    }
}
