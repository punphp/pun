//! `pun check`'s grouped view (output.md Build 5, matched to the brand mockup): one status line per step, a coloured
//! pill for the step, its problems under it as two lines each (`path:line:col  rule`, then the message, dim), and one
//! bold summary line.
//!
//! ```text
//! ✔  format   All files formatted
//! ✖   lint    1 error in 1 file · 1 fixable
//!    app/Http/Controllers/PostController.php:24:9  lint/laravel/no-request-all
//!    Pass only validated input, not $request->all().
//!
//! ✖ analyse   1 error in 1 file
//!    app/Models/Post.php:18:16  analyse/undefined/method
//!    Call to an undefined method App\Models\Post::publsh().
//!
//! ✖ 2 errors · 1 fixable (run pun check --fix)
//! ```

use super::{Renderer, Role, Step, Summary, plural};
use std::path::PathBuf;

/// One problem under a step: where (`path`, then `:line:col` or `:line` when known), its rules, its message (if
/// any), and anything printed under it as is (`-v`'s diff).
#[derive(Clone, Debug, Default)]
pub(crate) struct CheckItem {
    pub(crate) path: PathBuf,
    pub(crate) at: String,
    pub(crate) rules: Vec<String>,
    pub(crate) message: Option<String>,
    pub(crate) below: String,
}

/// One step: its errors, warnings and fixable problems, the files they are in, and what to say when it is clean.
#[derive(Clone, Debug)]
pub(crate) struct StepLine<'a> {
    pub(crate) step: Step,
    pub(crate) errors: usize,
    pub(crate) warnings: usize,
    pub(crate) fixable: usize,
    pub(crate) files: usize,
    /// The status text that replaces the counts (`All files formatted`, `3 files need formatting`).
    pub(crate) text: Option<String>,
    /// The status is a failure even without errors (files need formatting).
    pub(crate) failed: bool,
    pub(crate) items: &'a [CheckItem],
}

impl Renderer {
    /// A step's status line and, under it, its problems; a step with problems ends with a blank line.
    pub(crate) fn check_step(&self, s: &StepLine) -> String {
        let (glyph, role) = if s.errors > 0 || s.failed {
            (self.look.glyph("✖", "x"), Role::Error)
        } else if s.warnings > 0 {
            (self.look.glyph("⚠", "!"), Role::Warning)
        } else {
            (self.look.glyph("✔", "ok"), Role::Ok)
        };
        let dot = self.look.glyph("·", "-");
        let text = s.text.clone().unwrap_or_else(|| {
            let mut counts = vec![];
            if s.errors > 0 {
                counts.push(plural(s.errors, "error", "errors"));
            }
            if s.warnings > 0 {
                counts.push(plural(s.warnings, "warning", "warnings"));
            }
            if counts.is_empty() {
                return "No problems".into();
            }
            let mut text = format!(
                "{} in {}",
                counts.join(", "),
                plural(s.files, "file", "files")
            );
            if s.fixable > 0 {
                text.push_str(&format!(" {dot} {} fixable", s.fixable));
            }
            text
        });
        let mut out = format!(
            "{} {}  {text}\n",
            self.look.paint(role, glyph),
            self.look.pill(s.step)
        );
        for item in s.items {
            out.push_str(&self.check_item(item));
        }
        if !s.items.is_empty() && !self.concise {
            out.push('\n');
        }
        out
    }

    /// `   path:line:col  rule`, then `   message` dim; one line under `--concise`.
    fn check_item(&self, item: &CheckItem) -> String {
        let location = format!("{}{}", item.path.display(), item.at);
        let location = self.path_link(&item.path, &location);
        let rules: Vec<String> = item.rules.iter().map(|r| self.rule(r)).collect();
        let rule = rules.join(", ");
        let mut out = match &item.message {
            Some(m) if self.concise => {
                format!("   {location}  {rule}  {}\n", self.look.paint(Role::Dim, m))
            }
            Some(m) => format!(
                "   {location}  {rule}\n   {}\n",
                self.look.paint(Role::Dim, m)
            ),
            None => format!("   {location}  {rule}\n"),
        };
        out.push_str(&item.below);
        out
    }

    /// The last line: the glyph, then the non-zero counts in bold yellow (`2 errors · 1 fixable (run pun check
    /// --fix)`), or `No problems`.
    pub(crate) fn check_summary(&self, s: &Summary) -> String {
        let mut parts = vec![];
        if s.errors > 0 {
            parts.push(plural(s.errors, "error", "errors"));
        }
        if s.warnings > 0 {
            parts.push(plural(s.warnings, "warning", "warnings"));
        }
        if s.fixable > 0 {
            parts.push(format!("{} fixable (run {})", s.fixable, s.fix_command));
        }
        if s.need_formatting > 0 {
            parts.push(plural(
                s.need_formatting,
                "file needs formatting",
                "files need formatting",
            ));
        }
        let (glyph, role) = if s.errors > 0 || s.need_formatting > 0 {
            (self.look.glyph("✖", "x"), Role::Error)
        } else if s.warnings > 0 {
            (self.look.glyph("⚠", "!"), Role::Warning)
        } else {
            parts.push("No problems".into());
            (self.look.glyph("✔", "ok"), Role::Ok)
        };
        let dot = self.look.glyph("·", "-");
        format!(
            "{} {}\n",
            self.look.paint(role, glyph),
            self.look
                .paint(Role::Summary, &parts.join(&format!(" {dot} ")))
        )
    }
}
