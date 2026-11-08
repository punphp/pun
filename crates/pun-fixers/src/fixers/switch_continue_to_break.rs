//! Port of `PhpCsFixer\Fixer\ControlStructure\SwitchContinueToBreakFixer`.
use pun_format::Fixer;
use pun_tokens::Kind::*;
use pun_tokens::{Proto, Token, Tokens};
use std::path::Path;

struct SwitchContinueToBreakFixer;

pub(crate) fn fixer() -> Box<dyn Fixer> {
    Box::new(SwitchContinueToBreakFixer)
}

impl Fixer for SwitchContinueToBreakFixer {
    fn name(&self) -> &'static str {
        "switch_continue_to_break"
    }
    fn priority(&self) -> i32 {
        0
    }
    fn is_candidate(&self, tokens: &Tokens) -> bool {
        tokens.is_all_token_kinds_found(&[T_SWITCH, T_CONTINUE]) && !tokens.has_alternative_syntax()
    }
    fn fix(&self, _file: &Path, tokens: &mut Tokens) {
        let mut run = Run {
            tokens,
            switch_levels: vec![],
        };
        let count = run.tokens.count();
        let mut index = 1;
        while index + 1 < count {
            index = run.do_fix(index, 0, false) + 1;
        }
    }
}

struct Run<'a> {
    tokens: &'a mut Tokens,
    switch_levels: Vec<i64>,
}

impl Run<'_> {
    fn do_fix(&mut self, index: usize, depth: i64, is_in_switch: bool) -> usize {
        let tokens = &*self.tokens;
        let token = &tokens[index];
        if token.is_given_kind(&[T_FOREACH, T_FOR, T_WHILE]) {
            // go to first `(`, go to its close ')', go to first of '{', ';', '?>'
            let index = tokens
                .get_next_token_of_kind(index, &["(".into()], true)
                .unwrap();
            let index = tokens
                .get_next_token_of_kind(index, &[")".into()], true)
                .unwrap();
            let index = tokens
                .get_next_token_of_kind(
                    index,
                    &["{".into(), ";".into(), Proto::Kind(T_CLOSE_TAG)],
                    true,
                )
                .unwrap();
            if !tokens[index].equals("{", true) {
                return index;
            }
            return self.fix_in_loop(index, depth + 1);
        }
        if token.is_kind(T_DO) {
            let open = tokens
                .get_next_token_of_kind(index, &["{".into()], true)
                .unwrap();
            return self.fix_in_loop(open, depth + 1);
        }
        if token.is_kind(T_SWITCH) {
            return self.fix_in_switch(index, depth + 1);
        }
        if token.is_kind(T_CONTINUE) {
            return self.fix_continue_when_acts_as_break(index, is_in_switch, depth);
        }
        index
    }

    fn fix_in_switch(&mut self, switch_index: usize, depth: i64) -> usize {
        self.switch_levels.push(depth);
        let open_index = self
            .tokens
            .get_next_token_of_kind(switch_index, &["{".into()], true)
            .unwrap();
        let close_index = self
            .tokens
            .find_block_end(Tokens::BLOCK_TYPE_BRACE, open_index);
        let mut index = open_index + 1;
        while index < close_index {
            index = self.do_fix(index, depth, true) + 1;
        }
        self.switch_levels.pop();
        close_index
    }

    fn fix_in_loop(&mut self, mut open_index: usize, depth: i64) -> usize {
        let mut open_count = 1;
        loop {
            open_index += 1;
            let token = &self.tokens[open_index];
            if token.equals("{", true) {
                open_count += 1;
                continue;
            }
            if token.equals("}", true) {
                open_count -= 1;
                if open_count == 0 {
                    break;
                }
                continue;
            }
            open_index = self.do_fix(open_index, depth, false);
        }
        open_index
    }

    fn fix_continue_when_acts_as_break(
        &mut self,
        continue_index: usize,
        is_in_switch: bool,
        depth: i64,
    ) -> usize {
        let following_index = self
            .tokens
            .get_next_meaningful_token(continue_index)
            .unwrap();
        let following = &self.tokens[following_index];
        if is_in_switch && following.equals(";", true) {
            self.replace_continue_with_break_token(continue_index); // short continue 1 notation
            return following_index;
        }
        if !following.is_kind(T_LNUMBER) {
            return following_index;
        }
        let after_index = self
            .tokens
            .get_next_meaningful_token(following_index)
            .unwrap();
        if !self.tokens[after_index].equals(";", true) {
            return after_index; // e.g. `continue 1 ?><?php + $a;`
        }
        let jump: Vec<u8> = following
            .get_content()
            .iter()
            .copied()
            .filter(|&c| c != b'_')
            .collect();
        // hexdec / bindec / octdec ignore invalid digits. Upstream's "cannot process" and "> PHP_INT_MAX" exits are
        // unreachable: every T_LNUMBER is 0-prefixed or decimal, and fits in an int (bigger literals lex as T_DNUMBER).
        let jump = if jump.len() > 2 && jump[1] == b'x' {
            base_dec(&jump, 16)
        } else if jump.len() > 2 && jump[1] == b'b' {
            base_dec(&jump, 2)
        } else if jump.len() > 1 && jump[0] == b'0' {
            base_dec(&jump, 8)
        } else {
            base_dec(&jump, 10)
        };
        if is_in_switch && (jump == 1 || jump == 0) {
            self.replace_continue_with_break_token(continue_index); // long continue 0/1 notation
            return after_index;
        }
        let destination = i128::from(depth) - jump + 1;
        if self
            .switch_levels
            .iter()
            .any(|&l| i128::from(l) == destination)
        {
            self.replace_continue_with_break_token(continue_index);
        }
        after_index
    }

    fn replace_continue_with_break_token(&mut self, index: usize) {
        self.tokens.set(index, Token::new(T_BREAK, "break"));
    }
}

/// PHP's hexdec/bindec/octdec (and a decimal cast): skip non-digits.
fn base_dec(s: &[u8], base: u32) -> i128 {
    s.iter()
        .filter_map(|&c| (c as char).to_digit(base))
        .fold(0, |acc, d| acc * i128::from(base) + i128::from(d))
}
