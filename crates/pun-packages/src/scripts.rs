//! The root package's `scripts`, run for an event as Composer runs them: `@php x`, `@putenv K=V`, `@other-script`,
//! a `Class::method` callback (called through a small PHP runner with the parts of Composer's event that scripts
//! use), or a shell command with `vendor/bin` first on `PATH`.

#![expect(
    clippy::print_stderr,
    reason = "each script is echoed before it runs, as Composer echoes it"
)]

use anyhow::{Context, Result, bail};
use serde_json::{Map, Value};
use std::path::Path;
use std::process::Command;

pub struct Runner<'a> {
    pub base: &'a Path,
    pub vendor: &'a str,
    pub bin_dir: &'a str,
    pub manifest: &'a Map<String, Value>,
    pub dev_mode: bool,
    /// `PHP_BINARY`, else `php` on `PATH`.
    pub php: String,
    env: Vec<(String, Option<String>)>,
    stack: Vec<String>,
}

impl<'a> Runner<'a> {
    pub fn new(
        base: &'a Path,
        vendor: &'a str,
        bin_dir: &'a str,
        manifest: &'a Map<String, Value>,
        dev_mode: bool,
    ) -> Self {
        let php = php_binary(std::env::var("PHP_BINARY").ok());
        Runner {
            base,
            vendor,
            bin_dir,
            manifest,
            dev_mode,
            php,
            env: vec![],
            stack: vec![],
        }
    }

    fn listeners(&self, event: &str) -> Vec<String> {
        match self.manifest.get("scripts").and_then(|s| s.get(event)) {
            Some(Value::String(s)) => vec![s.clone()],
            Some(Value::Array(items)) => items
                .iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect(),
            _ => vec![],
        }
    }

    /// Runs every listener of `event`; a failing one stops the run with its exit code in the error.
    pub fn dispatch(&mut self, event: &str, args: &[String]) -> Result<()> {
        if self.stack.iter().any(|e| e == event) {
            bail!("Circular call to script handler '{event}' detected");
        }
        self.stack.push(event.to_string());
        let result = (|| {
            for callable in self.listeners(event) {
                self.run(event, &callable, args)?;
            }
            Ok(())
        })();
        self.stack.pop();
        result
    }

    fn run(&mut self, event: &str, callable: &str, args: &[String]) -> Result<()> {
        let (callable, args) = match callable.contains("@no_additional_args") {
            true => (
                callable
                    .replace(" @no_additional_args", "")
                    .replace("@no_additional_args", ""),
                &[][..],
            ),
            false => (callable.to_string(), args),
        };
        if callable.is_empty() {
            return Ok(());
        }
        let quoted = args.iter().map(|a| quote(a)).collect::<Vec<_>>().join(" ");
        if let Some(rest) = callable.strip_prefix("@putenv ") {
            match rest.split_once('=') {
                Some((k, v)) => self.env.push((k.to_string(), Some(v.to_string()))),
                None => self.env.push((rest.to_string(), None)),
            }
            return Ok(());
        }
        if callable.starts_with('@')
            && !callable.starts_with("@php ")
            && !callable.starts_with("@composer ")
        {
            let mut words = split_args(&callable[1..]);
            let name = if words.is_empty() {
                String::new()
            } else {
                words.remove(0)
            };
            let mut forwarded = vec![];
            let mut had = false;
            for w in words {
                if w == "@additional_args" {
                    forwarded.extend(args.iter().cloned());
                    had = true;
                } else {
                    forwarded.push(w);
                }
            }
            if !had {
                forwarded.extend(args.iter().cloned());
            }
            if self.listeners(&name).is_empty() {
                eprintln!("You made a reference to a non-existent script {callable}");
            }
            return self
                .dispatch(&name, &forwarded)
                .with_context(|| format!("Script {callable} was called via {event}"));
        }
        if !callable.contains(' ') && callable.contains("::") {
            return self.callback(event, &callable, args);
        }
        let with_args = |c: &str| match (c.contains("@additional_args"), quoted.is_empty()) {
            (true, _) => c.replace("@additional_args", &quoted),
            (false, true) => c.to_string(),
            (false, false) => format!("{c} {quoted}"),
        };
        let exec = if let Some(rest) = callable.strip_prefix("@composer ") {
            format!("composer {}", with_args(rest))
        } else if let Some(rest) = callable.strip_prefix("@php ") {
            format!("{} {}", quote(&self.php), with_args(rest))
        } else {
            with_args(&callable)
        };
        eprintln!("> {}", with_args(&callable));
        let status = self.command("sh").arg("-c").arg(&exec).status()?;
        if !status.success() {
            bail!(
                "Script {callable} handling the {event} event returned with error code {}",
                status.code().unwrap_or(1)
            );
        }
        Ok(())
    }

    fn callback(&mut self, event: &str, callable: &str, args: &[String]) -> Result<()> {
        eprintln!("> {callable}");
        let runner = std::env::temp_dir().join(format!("pun-script-{}.php", std::process::id()));
        std::fs::write(&runner, include_str!("../assets/script-callback.php"))?;
        let status = (self.command(&self.php.clone()))
            .arg(&runner)
            .arg(self.vendor)
            .arg(event)
            .arg(if self.dev_mode { "1" } else { "0" })
            .arg(callable)
            .args(args)
            .env(
                "PUN_ROOT_PACKAGE",
                Value::Object(self.manifest.clone()).to_string(),
            )
            .status();
        let _ = std::fs::remove_file(&runner);
        let status = status.with_context(|| format!("can't run {} for {callable}", self.php))?;
        if !status.success() {
            bail!(
                "Script {callable} handling the {event} event returned with error code {}",
                status.code().unwrap_or(1)
            );
        }
        Ok(())
    }

    fn command(&self, program: &str) -> Command {
        let mut c = Command::new(program);
        c.current_dir(self.base);
        let path = std::env::var("PATH").unwrap_or_default();
        if Path::new(self.bin_dir).is_dir() && !path.split(':').any(|p| p == self.bin_dir) {
            c.env("PATH", format!("{}:{path}", self.bin_dir));
        }
        c.env("COMPOSER_DEV_MODE", if self.dev_mode { "1" } else { "0" });
        c.env("PHP_BINARY", &self.php);
        for (k, v) in &self.env {
            match v {
                Some(v) => c.env(k, v),
                None => c.env_remove(k),
            };
        }
        c
    }
}

/// The PHP that `@php` and callbacks run: `PHP_BINARY` when set, else `php` from PATH.
fn php_binary(env: Option<String>) -> String {
    env.filter(|p| !p.is_empty())
        .unwrap_or_else(|| "php".into())
}

/// `ProcessExecutor::escape` on a POSIX shell: always single-quoted.
fn quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

/// Splits on whitespace, keeping quoted parts together and unquoting them.
fn split_args(command: &str) -> Vec<String> {
    let (mut out, mut word, mut quote, mut started) = (vec![], String::new(), None::<char>, false);
    for ch in command.chars() {
        match (quote, ch) {
            (Some(q), c) if c == q => quote = None,
            (Some(_), c) => word.push(c),
            (None, '"' | '\'') => {
                quote = Some(ch);
                started = true;
            }
            (None, c) if c.is_whitespace() => {
                if started {
                    out.push(std::mem::take(&mut word));
                    started = false;
                }
            }
            (None, c) => {
                word.push(c);
                started = true;
            }
        }
    }
    if started {
        out.push(word);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn project(name: &str) -> std::path::PathBuf {
        let d = std::env::temp_dir().join(format!("pun-scripts-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(d.join("vendor/bin")).unwrap();
        d
    }

    fn run(dir: &Path, scripts: Value, event: &str) -> Result<()> {
        let manifest = json!({"scripts": scripts}).as_object().unwrap().clone();
        let vendor = dir.join("vendor").to_string_lossy().into_owned();
        let bin = dir.join("vendor/bin").to_string_lossy().into_owned();
        Runner::new(dir, &vendor, &bin, &manifest, true).dispatch(event, &[])
    }

    /// What Composer 2.11.2 does with the same scripts (`composer run-script a` in a copy): `@putenv` sets the
    /// environment of later commands, `@b 'x y'` runs script b with that argument appended, `vendor/bin` comes first
    /// on PATH, and COMPOSER_DEV_MODE is 1.
    #[test]
    fn commands_references_and_environment() {
        let d = project("run");
        // `composer` on PATH stands in for Composer, which `@composer` runs.
        for (bin, line) in [("tool", "tool"), ("composer", "composer $*")] {
            std::fs::write(
                d.join("vendor/bin").join(bin),
                format!("#!/bin/sh\necho {line} >> out.txt\n"),
            )
            .unwrap();
            std::fs::set_permissions(
                d.join("vendor/bin").join(bin),
                std::os::unix::fs::PermissionsExt::from_mode(0o755),
            )
            .unwrap();
        }
        let scripts = json!({
            "a": ["@putenv FOO=bar", "echo \"$FOO $COMPOSER_DEV_MODE\" > out.txt", "@b 'x y'", "tool", "@composer show x", "@putenv FOO", "echo \"[$FOO]\" >> out.txt"],
            "b": "echo b >> out.txt",
        });
        run(&d, scripts, "a").unwrap();
        assert_eq!(
            std::fs::read_to_string(d.join("out.txt")).unwrap(),
            "bar 1\nb x y\ntool\ncomposer show x\n[]\n"
        );
        std::fs::remove_dir_all(&d).unwrap();
    }

    /// Composer 2.11.2's `composer run-script a -- one "two words"` on the same scripts: a reference passes the
    /// arguments where `@additional_args` stands (or at the end), a command takes them there too, `@no_additional_args`
    /// drops them, `@php` runs PHP, and a reference to a script that doesn't exist (or to none) does nothing.
    #[test]
    fn arguments_like_composer() {
        let d = project("args");
        std::fs::remove_dir_all(d.join("vendor/bin")).unwrap();
        let manifest = json!({"scripts": {
            "a": ["@b @additional_args  extra ", "echo c @additional_args >> out.txt", "echo n @no_additional_args >> out.txt", "@php -r 'echo \"php\\n\";' >> out.txt", "@missing", "@"],
            "b": "echo b >> out.txt",
        }});
        let manifest = manifest.as_object().unwrap().clone();
        let vendor = d.join("vendor").to_string_lossy().into_owned();
        let bin = d.join("vendor/bin").to_string_lossy().into_owned();
        let args = ["one".to_string(), "two words".to_string()];
        Runner::new(&d, &vendor, &bin, &manifest, true)
            .dispatch("a", &args)
            .unwrap();
        assert_eq!(
            std::fs::read_to_string(d.join("out.txt")).unwrap(),
            "b one two words extra\nc one two words\nn\nphp\n"
        );
        assert_eq!(quote("it's"), "'it'\\''s'");
        assert_eq!(
            (php_binary(None), php_binary(Some(String::new()))),
            ("php".into(), "php".into())
        );
        assert_eq!(
            php_binary(Some("/usr/bin/php8.4".into())),
            "/usr/bin/php8.4"
        );
        std::fs::remove_dir_all(&d).unwrap();
    }

    #[test]
    fn failures_use_composers_messages() {
        let d = project("fail");
        let error =
            |scripts: Value, event: &str| format!("{:#}", run(&d, scripts, event).unwrap_err());
        assert_eq!(
            error(json!({"a": "exit 3"}), "a"),
            "Script exit 3 handling the a event returned with error code 3"
        );
        assert_eq!(
            error(json!({"a": "@a"}), "a"),
            "Script @a was called via a: Circular call to script handler 'a' detected"
        );
        assert_eq!(
            error(json!({"a": "@b", "b": "exit 4"}), "a"),
            "Script @b was called via a: Script exit 4 handling the b event returned with error code 4"
        );
        // An event without scripts, and an empty one, do nothing.
        run(&d, json!({"a": ""}), "a").unwrap();
        run(&d, json!({}), "post-install-cmd").unwrap();
        std::fs::remove_dir_all(&d).unwrap();
    }

    /// A `Class::method` callback gets an event with Composer's accessors; returning false fails it with code 1.
    #[test]
    fn php_callbacks() {
        let d = project("callback");
        std::fs::write(
            d.join("vendor/autoload.php"),
            "<?php\nnamespace Acme;\nfinal class Hooks {\n    public static function dump($event) {\n        file_put_contents('out.txt', $event->getName().' '.(int) $event->isDevMode().' '.basename($event->getComposer()->getConfig()->get('vendor-dir')).' '.$event->getComposer()->getPackage()->getName());\n    }\n    public static function fail($event) { return false; }\n}\n",
        )
        .unwrap();
        run(
            &d,
            json!({"post-autoload-dump": "Acme\\Hooks::dump"}),
            "post-autoload-dump",
        )
        .unwrap();
        assert_eq!(
            std::fs::read_to_string(d.join("out.txt")).unwrap(),
            "post-autoload-dump 1 vendor __root__"
        );
        let error = format!(
            "{:#}",
            run(&d, json!({"x": "Acme\\Hooks::fail"}), "x").unwrap_err()
        );
        assert_eq!(
            error,
            "Script Acme\\Hooks::fail handling the x event returned with error code 1"
        );
        // A class that isn't there is reported and skipped, as Composer skips it.
        run(&d, json!({"x": "Acme\\Missing::run"}), "x").unwrap();
        std::fs::remove_dir_all(&d).unwrap();
    }
}
