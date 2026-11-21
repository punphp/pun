use std::fmt;

use crate::di::Files;
use crate::phpstan_api::{self, ClassKind};
use crate::value::{Array, Key, Value};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    Unexpected { near: String, line: usize },
    UnterminatedString,
    UndefinedVariable(String),
    NotAnArray(String),
    NoArrayReturned,
    UnknownClass(String),
    NoExtensionsConstant,
    NoExtensionsValue,
    ExtensionsNotAnArray,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Unexpected { near, line } => write!(f, "unexpected {near:?} on line {line}"),
            Error::UnterminatedString => f.write_str("unterminated string"),
            Error::UndefinedVariable(name) => write!(f, "undefined ${name}"),
            Error::NotAnArray(name) => write!(f, "${name} is not an array"),
            Error::NoArrayReturned => f.write_str("the config does not return an array"),
            Error::UnknownClass(name) => write!(f, "{name} is neither a PHP nor a PHPStan class"),
            Error::NoExtensionsConstant => f.write_str("no EXTENSIONS constant"),
            Error::NoExtensionsValue => f.write_str("no EXTENSIONS value"),
            Error::ExtensionsNotAnArray => f.write_str("EXTENSIONS is not an array"),
        }
    }
}

impl std::error::Error for Error {}

enum Flow {
    Next,
    Return(Value),
}

struct P<'a> {
    s: &'a str,
    i: usize,
    dir: &'a str,
    files: &'a dyn Files,
    vars: Vec<(String, Value)>,
    run: bool,
}

fn class_exists(name: &str, interface: bool) -> Result<bool, Error> {
    let name = name.trim_start_matches('\\');
    if phpstan_api::owns(name) {
        let kind = phpstan_api::class_kind(name);
        return Ok(kind
            == Some(if interface {
                ClassKind::Interface
            } else {
                ClassKind::Class
            }));
    }
    let class = crate::symbols::class(name).ok_or_else(|| Error::UnknownClass(name.to_string()))?;
    Ok(if interface {
        class.is_interface
    } else {
        !class.is_interface && !class.is_trait
    })
}

fn truthy(v: &Value) -> bool {
    match v {
        Value::Null | Value::Bool(false) | Value::Int(0) => false,
        Value::Str(s) => !s.is_empty() && s != "0",
        Value::Array(a) => !a.is_empty(),
        _ => true,
    }
}

impl<'a> P<'a> {
    fn new(s: &'a str, i: usize, dir: &'a str, files: &'a dyn Files) -> P<'a> {
        P {
            s,
            i,
            dir,
            files,
            vars: Vec::new(),
            run: true,
        }
    }

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
            rest.as_bytes()
                .get(..t.len())
                .is_some_and(|p| p.eq_ignore_ascii_case(t.as_bytes()))
                && !rest[t.len()..].starts_with(|c: char| c.is_ascii_alphanumeric() || c == '_')
        } else {
            rest.starts_with(t)
        };
        if matches {
            self.i += t.len();
        }
        matches
    }

    fn expect(&mut self, t: &str) -> Result<(), Error> {
        if self.eat(t) {
            Ok(())
        } else {
            Err(self.unexpected())
        }
    }

    fn unexpected(&self) -> Error {
        Error::Unexpected {
            near: self.s[self.i..].chars().take(20).collect(),
            line: self.s[..self.i].matches('\n').count() + 1,
        }
    }

    fn var_name(&mut self) -> Result<String, Error> {
        self.expect("$")?;
        let n: String = self.s[self.i..]
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
            .collect();
        self.i += n.len();
        Ok(n)
    }

    fn expr(&mut self) -> Result<Value, Error> {
        let mut v = self.term()?;
        while self.eat(".") {
            let r = self.term()?;
            v = Value::Str(
                v.to_php_string().unwrap_or_default() + &r.to_php_string().unwrap_or_default(),
            );
        }
        Ok(v)
    }

    fn term(&mut self) -> Result<Value, Error> {
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
            return Err(Error::UnterminatedString);
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
            if !self.run {
                return Ok(Value::Null);
            }
            return self
                .vars
                .iter()
                .rev()
                .find(|(n, _)| *n == name)
                .map(|(_, v)| v.clone())
                .ok_or(Error::UndefinedVariable(name));
        }
        if self.eat("!") {
            return Ok(Value::Bool(!truthy(&self.term()?)));
        }
        for function in ["glob", "is_file", "class_exists", "interface_exists"] {
            let start = self.i;
            if self.eat(function) && self.eat("(") {
                let arg = self.expr()?.to_php_string().unwrap_or_default();
                self.expect(")")?;
                return Ok(match (self.run, function) {
                    (false, _) => Value::Null,
                    (true, "glob") => Value::Array(Array::list(
                        crate::php::glob(&arg).into_iter().map(Value::Str),
                    )),
                    (true, "is_file") => Value::Bool(self.files.is_file(&arg)),
                    _ => Value::Bool(class_exists(&arg, function == "interface_exists")?),
                });
            }
            self.i = start;
        }
        let name: String = rest
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '\\')
            .collect();
        let class_constant = !name.is_empty() && rest[name.len()..].starts_with("::class");
        if class_constant {
            self.i += name.len() + "::class".len();
            return Ok(Value::Str(name.trim_start_matches('\\').to_string()));
        }
        let num: String = rest
            .chars()
            .take_while(|c| c.is_ascii_digit() || *c == '-')
            .collect();
        let float = rest[num.len()..].starts_with('.')
            && rest[num.len() + 1..].starts_with(|c: char| c.is_ascii_digit());
        if let (false, Ok(n)) = (float, num.parse::<i64>()) {
            self.i += num.len();
            return Ok(Value::Int(n));
        }
        Err(self.unexpected())
    }

    fn statement(&mut self) -> Result<Flow, Error> {
        if self.eat("declare") {
            let end = self.s[self.i..]
                .find(';')
                .ok_or_else(|| self.unexpected())?;
            self.i += end + 1;
            return Ok(Flow::Next);
        }
        if self.eat("return") {
            let v = self.expr()?;
            if self.run {
                return Ok(Flow::Return(v));
            }
            self.expect(";")?;
            return Ok(Flow::Next);
        }
        if self.eat("foreach") {
            return self.foreach();
        }
        if self.eat("if") {
            return self.branches();
        }
        if self.eat("{") {
            while !self.eat("}") {
                if let Flow::Return(v) = self.statement()? {
                    return Ok(Flow::Return(v));
                }
            }
            return Ok(Flow::Next);
        }
        self.skip();
        if !self.s[self.i..].starts_with('$') {
            return Err(self.unexpected());
        }
        let name = self.var_name()?;
        let push = self.eat("[");
        if push {
            self.expect("]")?;
        }
        self.expect("=")?;
        let v = self.expr()?;
        self.expect(";")?;
        if !self.run {
            return Ok(Flow::Next);
        }
        if push {
            let Some((_, Value::Array(a))) = self.vars.iter_mut().rev().find(|(n, _)| *n == name)
            else {
                return Err(Error::NotAnArray(name));
            };
            a.push(v);
        } else {
            self.vars.push((name, v));
        }
        Ok(Flow::Next)
    }

    fn foreach(&mut self) -> Result<Flow, Error> {
        self.expect("(")?;
        let subject = self.expr()?;
        self.expect("as")?;
        let first = self.var_name()?;
        let (key, value) = if self.eat("=>") {
            (Some(first), self.var_name()?)
        } else {
            (None, first)
        };
        self.expect(")")?;
        let body = self.i;
        if let (true, Value::Array(items)) = (self.run, subject) {
            for (k, v) in items.iter() {
                if let Some(key) = &key {
                    self.vars.push((key.clone(), Value::from(k.clone())));
                }
                self.vars.push((value.clone(), v.clone()));
                self.i = body;
                if let Flow::Return(v) = self.statement()? {
                    return Ok(Flow::Return(v));
                }
            }
        }
        self.i = body;
        self.skipped()
    }

    fn branches(&mut self) -> Result<Flow, Error> {
        let run = self.run;
        let mut taken = false;
        loop {
            self.run = run && !taken;
            self.expect("(")?;
            let condition = self.expr()?;
            self.expect(")")?;
            self.run = run && !taken && truthy(&condition);
            taken |= self.run;
            let flow = self.statement();
            self.run = run;
            if let Flow::Return(v) = flow? {
                return Ok(Flow::Return(v));
            }
            if self.eat("elseif") {
                continue;
            }
            if !self.eat("else") {
                return Ok(Flow::Next);
            }
            if taken {
                return self.skipped();
            }
            return self.statement();
        }
    }

    fn skipped(&mut self) -> Result<Flow, Error> {
        let run = std::mem::replace(&mut self.run, false);
        let flow = self.statement();
        self.run = run;
        flow
    }

    fn items(&mut self, close: &str) -> Result<Value, Error> {
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

pub fn load_config(php: &str, dir: &str, files: &dyn Files) -> Result<Array, Error> {
    let mut p = P::new(php, 0, dir, files);
    p.expect("<?php")?;
    loop {
        if let Flow::Return(v) = p.statement()? {
            return match v {
                Value::Array(a) => Ok(a),
                _ => Err(Error::NoArrayReturned),
            };
        }
    }
}

pub fn generated_config_extensions(
    php: &str,
    dir: &str,
    files: &dyn Files,
) -> Result<Array, Error> {
    let at = php
        .find("const EXTENSIONS")
        .ok_or(Error::NoExtensionsConstant)?;
    let eq = php[at..].find('=').ok_or(Error::NoExtensionsValue)? + at + 1;
    let mut p = P::new(php, eq, dir, files);
    match p.expr()? {
        Value::Array(a) => Ok(a),
        _ => Err(Error::ExtensionsNotAnArray),
    }
}
