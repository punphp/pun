//! PHP's own parser, for linting: bison's LALR(1) driver (`yyparse` from yacc.c) over the tables generated from
//! php-src's zend_language_parser.y (php_grammar.rs), fed by pun-tokens' parser-mode scanner. Accepts exactly
//! what `token_get_all($code, TOKEN_PARSE)` accepts, and fails with the same `ParseError`/`CompileError`
//! message and line: bison's verbose syntax error rendered by `zend_yytnamerr`, the scanner's errors, and the
//! errors grammar actions throw (modifier checks, `__halt_compiler` scope, `declare(encoding=...)`). The
//! actions' `zend_lex_tstring` failure (`<?=` as an identifier) cannot happen: `<?=` always follows `?>`, a `';'`. The AST is not built: each stack entry only records the tokens it covers.

use crate::php_grammar::*;
use pun_tokens::{Kind, ParseLexer, ParseToken};

/// `CompileError` when `fatal` (PHP-CS-Fixer reports it as "Fatal error"), else `ParseError`.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct SyntaxError {
    pub message: String,
    pub line: u32,
    pub fatal: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum Target {
    Property,
    Method,
    Constant,
    Cpp,
    Hook,
}

const PUBLIC: u32 = 1;
const PROTECTED: u32 = 2;
const PRIVATE: u32 = 4;
const PPP: u32 = PUBLIC | PROTECTED | PRIVATE;
const STATIC: u32 = 8;
const FINAL: u32 = 16;
const ABSTRACT: u32 = 32;
const READONLY: u32 = 64;
const PUBLIC_SET: u32 = 128;
const PROTECTED_SET: u32 = 256;
const PRIVATE_SET: u32 = 512;
const PPP_SET: u32 = PUBLIC_SET | PROTECTED_SET | PRIVATE_SET;

fn modifier_name(k: Kind) -> &'static str {
    match k {
        Kind::T_PUBLIC => "public",
        Kind::T_PROTECTED => "protected",
        Kind::T_PRIVATE => "private",
        Kind::T_STATIC => "static",
        Kind::T_FINAL => "final",
        Kind::T_READONLY => "readonly",
        Kind::T_ABSTRACT => "abstract",
        Kind::T_PUBLIC_SET => "public(set)",
        Kind::T_PROTECTED_SET => "protected(set)",
        _ => "private(set)",
    }
}

/// `zend_modifier_token_to_flag`.
fn modifier_flag(target: Target, k: Kind) -> Result<u32, String> {
    use Target::*;
    let flag = match k {
        Kind::T_PUBLIC if target != Hook => PUBLIC,
        Kind::T_PROTECTED if target != Hook => PROTECTED,
        Kind::T_PRIVATE if target != Hook => PRIVATE,
        Kind::T_READONLY if matches!(target, Property | Cpp) => READONLY,
        Kind::T_ABSTRACT if matches!(target, Method | Property) => ABSTRACT,
        Kind::T_FINAL => FINAL,
        Kind::T_STATIC if matches!(target, Property | Method) => STATIC,
        Kind::T_PUBLIC_SET if matches!(target, Property | Cpp) => PUBLIC_SET,
        Kind::T_PROTECTED_SET if matches!(target, Property | Cpp) => PROTECTED_SET,
        Kind::T_PRIVATE_SET if matches!(target, Property | Cpp) => PRIVATE_SET,
        _ => {
            // Properties accept every modifier.
            let member = match target {
                Method => "method",
                Constant => "class constant",
                Cpp => "parameter",
                _ => "property hook",
            };
            return Err(format!("Cannot use the {} modifier on a {member}", modifier_name(k)));
        }
    };
    Ok(flag)
}

/// `zend_modifier_list_to_flags` (with `zend_add_member_modifier`).
fn member_modifiers(target: Target, list: &[Kind]) -> Result<(), String> {
    let mut flags = 0;
    for &k in list {
        let new = modifier_flag(target, k)?;
        let duplicate = flags & new != 0;
        // Set-visibility flags only pass modifier_flag for properties and promoted parameters.
        if (flags & PPP != 0 && new & PPP != 0) || (flags & PPP_SET != 0 && new & PPP_SET != 0) {
            return Err("Multiple access type modifiers are not allowed".into());
        }
        flags |= new;
        // Only methods and properties accept `abstract`.
        if flags & ABSTRACT != 0 && flags & FINAL != 0 {
            let member = if target == Target::Method { "method" } else { "property" };
            return Err(format!("Cannot use the final modifier on an abstract {member}"));
        }
        if duplicate {
            return Err(format!("Multiple {} modifiers are not allowed", modifier_name(k)));
        }
    }
    Ok(())
}

fn class_flag(k: Kind) -> u32 {
    match k {
        Kind::T_ABSTRACT => ABSTRACT,
        Kind::T_FINAL => FINAL,
        _ => READONLY,
    }
}

/// `zend_add_class_modifier` over the whole list.
fn class_modifiers(list: &[Kind]) -> Result<(), String> {
    let mut flags = 0;
    for &k in list {
        let new = class_flag(k);
        if flags & new != 0 {
            return Err(format!("Multiple {} modifiers are not allowed", modifier_name(k)));
        }
        flags |= new;
        if flags & ABSTRACT != 0 && flags & FINAL != 0 {
            return Err("Cannot use the final modifier on an abstract class".into());
        }
    }
    Ok(())
}

/// `zend_add_anonymous_class_modifier` over the whole list.
fn anonymous_class_modifiers(list: &[Kind]) -> Result<(), String> {
    let mut flags = 0;
    for &k in list {
        let new = class_flag(k);
        if new == ABSTRACT || new == FINAL {
            return Err(format!("Cannot use the {} modifier on an anonymous class", modifier_name(k)));
        }
        if flags & new != 0 {
            return Err("Multiple readonly modifiers are not allowed".into());
        }
        flags |= new;
    }
    Ok(())
}

struct Parser<'a> {
    src: &'a [u8],
    lx: ParseLexer<'a>,
    /// Shifted tokens; stack entries cover ranges of it.
    toks: Vec<ParseToken>,
}

impl Parser<'_> {
    fn kinds(&self, (a, b): (usize, usize)) -> Vec<Kind> {
        self.toks[a..b].iter().filter_map(|t| t.kind).collect()
    }

    fn text(&self, t: &ParseToken) -> &[u8] {
        &self.src[t.start..t.end]
    }

    /// Is the expression a `ZEND_AST_ZVAL` once parsed: a literal, a heredoc without interpolation, a parenthesized
    /// one, or a `.` chain of them (zend_ast_create_concat_op folds)?
    fn is_zval(&self, toks: &[ParseToken]) -> bool {
        let is = |t: &ParseToken, c: u8| t.kind.is_none() && t.ch == c;
        let mut depth = 0;
        let mut parts = vec![0];
        for (i, t) in toks.iter().enumerate() {
            match t.kind {
                None if matches!(t.ch, b'(' | b'[' | b'{') => depth += 1,
                None if matches!(t.ch, b')' | b']' | b'}') => depth -= 1,
                None if t.ch == b'.' && depth == 0 => parts.push(i + 1),
                _ => {}
            }
        }
        if parts.len() > 1 {
            parts.push(toks.len() + 1);
            return parts.windows(2).all(|w| self.is_zval(&toks[w[0]..w[1] - 1]));
        }
        match toks {
            [t] => matches!(t.kind, Some(Kind::T_LNUMBER | Kind::T_DNUMBER | Kind::T_CONSTANT_ENCAPSED_STRING)),
            [s, .., e] if s.kind == Some(Kind::T_START_HEREDOC) && e.kind == Some(Kind::T_END_HEREDOC) => toks.len() <= 3,
            [o, inner @ .., c] if is(o, b'(') && is(c, b')') => self.is_zval(inner),
            _ => false,
        }
    }

    /// `zend_handle_encoding_declaration` on a `const_list` (`name = expr, ...`).
    fn encoding_declaration(&self, (a, b): (usize, usize)) -> Result<(), String> {
        let toks = &self.toks[a..b];
        let mut depth = 0;
        let mut elems = vec![];
        let mut start = 0;
        for (i, t) in toks.iter().enumerate() {
            match t.kind.map_or(t.ch, |_| 0) {
                b'(' | b'[' | b'{' => depth += 1,
                b')' | b']' | b'}' => depth -= 1,
                b',' if depth == 0 => {
                    elems.push(&toks[start..i]);
                    start = i + 1;
                }
                _ => {}
            }
        }
        elems.push(&toks[start..]);
        if elems.iter().any(|e| self.text(&e[0]).eq_ignore_ascii_case(b"encoding") && !self.is_zval(&e[2..])) {
            return Err("Encoding must be a literal".into());
        }
        Ok(())
    }

    /// The semantic checks of grammar actions, run when `rule` reduces `rhs` (ranges of its right-hand side).
    fn action(&mut self, rule: usize, rhs: &[(usize, usize)], below: Option<(usize, usize)>) -> Result<(), SyntaxError> {
        let fatal = |message: String| SyntaxError { message, line: 0, fatal: true };
        let all = rhs.first().map_or((0, 0), |f| (f.0, rhs.last().unwrap().1));
        let r = match rule {
            RULE_HALT_TOP => {
                self.lx.stop();
                Ok(())
            }
            RULE_HALT_INNER => Err(fatal("__HALT_COMPILER() can only be used from the outermost scope".into())),
            RULE_DECLARE => self.encoding_declaration(below.unwrap()).map_err(fatal),
            RULE_CLASS_MODIFIERS => class_modifiers(&self.kinds(all)).map_err(fatal),
            RULE_ANON_MODIFIER_FIRST | RULE_ANON_MODIFIERS => anonymous_class_modifiers(&self.kinds(all)).map_err(fatal),
            RULE_CPP_MODIFIERS => member_modifiers(Target::Cpp, &self.kinds(all)).map_err(fatal),
            RULE_PROPERTY_MODIFIERS => member_modifiers(Target::Property, &self.kinds(all)).map_err(fatal),
            RULE_METHOD_MODIFIERS => member_modifiers(Target::Method, &self.kinds(all)).map_err(fatal),
            RULE_CONST_MODIFIERS => member_modifiers(Target::Constant, &self.kinds(all)).map_err(fatal),
            RULE_HOOK_MODIFIERS => member_modifiers(Target::Hook, &self.kinds(all)).map_err(fatal),
            RULE_ALIAS_MODIFIER_NAME | RULE_ALIAS_MODIFIER => modifier_flag(Target::Method, self.kinds(rhs[2])[0]).map(|_| ()).map_err(fatal),
            _ => Ok(()),
        };
        r.map_err(|e| SyntaxError { line: self.lx.lineno(), ..e })
    }

    /// `zend_yytnamerr` for the unexpected token.
    fn unexpected(&self, symbol: i32, tok: Option<&ParseToken>) -> String {
        let Some(tok) = tok else {
            return "end of file".into();
        };
        let name = YYTNAME[symbol as usize];
        match name {
            "\"'\\\\'\"" => return "token \"\\\"".into(),
            "\"amp\"" => return "token \"&\"".into(),
            "'\"'" => return "double-quote mark".into(),
            _ => {}
        }
        let mut toktype = name.strip_prefix('"').and_then(|n| n.strip_suffix('"')).unwrap_or(name);
        if toktype.starts_with('\'') {
            return format!("token \"{}\"", &toktype[1..toktype.len() - 1]);
        }
        let mut content = self.text(tok);
        if content.len() == 1 && name == "\"invalid character\"" {
            return format!("character 0x{:02X}", content[0]);
        }
        if let Some(nl) = content.iter().position(|&c| c == b'\n') {
            content = &content[..nl];
        }
        if !content.is_empty() && name == "\"quoted string\"" {
            match content[0] {
                b'"' => toktype = "double-quoted string",
                b'\'' => toktype = "single-quoted string",
                _ => {}
            }
        }
        if content.first().is_some_and(|&c| c == b'\'' || c == b'"') {
            content = &content[1..];
        }
        if content.last().is_some_and(|&c| c == b'\'' || c == b'"') {
            content = &content[..content.len() - 1];
        }
        if content.len() > 33 {
            return format!("{toktype} \"{}...\"", String::from_utf8_lossy(&content[..30]));
        }
        format!("{toktype} \"{}\"", String::from_utf8_lossy(content))
    }

    /// `yysyntax_error` + `zenderror`.
    fn syntax_error(&self, state: i32, symbol: i32, tok: Option<&ParseToken>) -> SyntaxError {
        let line = self.lx.lineno();
        let mut msg = format!("syntax error, unexpected {}", self.unexpected(symbol, tok));
        let mut expected = Vec::new();
        // The state read a lookahead, so its yypact entry is not the default.
        let yyn = YYPACT[state as usize] as i32;
        for x in yyn.max(0) - yyn..(YYLAST - yyn + 1).min(YYNTOKENS) {
            if YYCHECK[(x + yyn) as usize] as i32 == x && x != 1 && YYTABLE[(x + yyn) as usize] as i32 != YYTABLE_NINF {
                expected.push(x);
            }
        }
        if expected.len() <= 4 {
            for (i, x) in expected.into_iter().enumerate() {
                let name = YYTNAME[x as usize];
                let shown = match name {
                    "\"'\\\\'\"" => "\"\\\"".to_string(),
                    "\"amp\"" => "token \"&\"".to_string(),
                    _ => name.strip_prefix('"').and_then(|n| n.strip_suffix('"')).unwrap_or(name).replace('\'', "\""),
                };
                msg += if i == 0 { ", expecting " } else { " or " };
                msg += &shown;
            }
        }
        SyntaxError { message: msg, line, fatal: false }
    }

    fn symbol(t: &ParseToken) -> i32 {
        match t.kind {
            Some(k) => kind_symbol(k),
            None => char_symbol(t.ch),
        }
    }

    fn run(&mut self) -> Result<(), SyntaxError> {
        let lex_error = |e: pun_tokens::LexError| SyntaxError { message: e.message, line: e.line, fatal: false };
        let mut states: Vec<i32> = vec![0];
        let mut values: Vec<(usize, usize)> = vec![(0, 0)];
        // Lookahead: (symbol, token or None at the end of file).
        let mut la: Option<(i32, Option<ParseToken>)> = None;
        'parse: loop {
            let state = *states.last().unwrap();
            if state == YYFINAL {
                return Ok(());
            }
            let mut yyn = YYPACT[state as usize] as i32;
            let rule = 'decide: {
                if yyn == YYPACT_NINF {
                    break 'decide YYDEFACT[state as usize] as i32;
                }
                if la.is_none() {
                    la = Some(match self.lx.next_token().map_err(lex_error)? {
                        Some(t) => (Self::symbol(&t), Some(t)),
                        None => (0, None),
                    });
                }
                let symbol = la.unwrap().0;
                yyn += symbol;
                if yyn < 0 || yyn > YYLAST || YYCHECK[yyn as usize] as i32 != symbol {
                    break 'decide YYDEFACT[state as usize] as i32;
                }
                let action = YYTABLE[yyn as usize] as i32;
                if action <= 0 {
                    if action == YYTABLE_NINF {
                        break 'decide 0;
                    }
                    break 'decide -action;
                }
                // Shift.
                let (_, tok) = la.take().unwrap();
                let n = self.toks.len();
                if let Some(t) = tok {
                    self.toks.push(t);
                }
                states.push(action);
                values.push((n, self.toks.len()));
                continue 'parse;
            };
            if rule == 0 {
                // PHP's tables have no error-by-default state without a lookahead.
                let (symbol, tok) = la.unwrap();
                return Err(self.syntax_error(state, symbol, tok.as_ref()));
            }
            let rule = rule as usize;
            let len = YYR2[rule] as usize;
            let rhs: Vec<(usize, usize)> = values[values.len() - len..].to_vec();
            let below = values.len().checked_sub(2).map(|i| values[i]);
            self.action(rule, &rhs, below)?;
            let n = self.toks.len();
            let range = if len == 0 { (n, n) } else { (rhs[0].0, rhs[len - 1].1) };
            states.truncate(states.len() - len);
            values.truncate(values.len() - len);
            let top = *states.last().unwrap();
            let lhs = YYR1[rule] as i32 - YYNTOKENS;
            let i = YYPGOTO[lhs as usize] as i32 + top;
            let next = if (0..=YYLAST).contains(&i) && YYCHECK[i as usize] as i32 == top { YYTABLE[i as usize] as i32 } else { YYDEFGOTO[lhs as usize] as i32 };
            states.push(next);
            values.push(range);
        }
    }
}

/// Parse like `token_get_all($code, TOKEN_PARSE)`.
pub(crate) fn parse(src: &[u8]) -> Result<(), SyntaxError> {
    Parser { src, lx: ParseLexer::new(src), toks: Vec::new() }.run()
}
