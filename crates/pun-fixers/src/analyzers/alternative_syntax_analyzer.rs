//! Port of `PhpCsFixer\Tokenizer\Analyzer\AlternativeSyntaxAnalyzer`.
use pun_tokens::{Kind, Kind::*, Proto, Tokens};

pub struct AlternativeSyntaxAnalyzer;

fn alternative_syntax_block_edges(kind: Kind) -> Option<&'static [Kind]> {
    Some(match kind {
        T_IF => &[T_ENDIF, T_ELSE, T_ELSEIF],
        T_ELSE => &[T_ENDIF],
        T_ELSEIF => &[T_ENDIF, T_ELSE, T_ELSEIF],
        T_FOR => &[T_ENDFOR],
        T_FOREACH => &[T_ENDFOREACH],
        T_WHILE => &[T_ENDWHILE],
        T_SWITCH => &[T_ENDSWITCH],
        _ => return None,
    })
}

impl AlternativeSyntaxAnalyzer {
    pub fn belongs_to_alternative_syntax(&self, tokens: &Tokens, index: usize) -> bool {
        if !tokens[index].equals(":", true) {
            return false;
        }
        let prev_index = tokens.get_prev_meaningful_token(index).unwrap();
        if tokens[prev_index].is_kind(T_ELSE) {
            return true;
        }
        if !tokens[prev_index].equals(")", true) {
            return false;
        }
        let open_parenthesis_index = tokens.find_block_start(Tokens::BLOCK_TYPE_PARENTHESIS, prev_index);
        let before_open_parenthesis_index = tokens.get_prev_meaningful_token(open_parenthesis_index).unwrap();
        tokens[before_open_parenthesis_index].is_given_kind(&[T_DECLARE, T_ELSEIF, T_FOR, T_FOREACH, T_IF, T_SWITCH, T_WHILE])
    }

    pub fn find_alternative_syntax_block_end(&self, tokens: &Tokens, mut index: usize) -> usize {
        assert!(index < tokens.len(), "There is no token at index {index}.");
        assert!(self.is_start_of_alternative_syntax_block(tokens, index), "Token at index {index} is not the start of an alternative syntax block.");
        let start_token_kind = tokens[index].get_id().unwrap();
        let end_token_kinds = alternative_syntax_block_edges(start_token_kind).unwrap();
        let mut find_kinds: Vec<Proto> = vec![start_token_kind.into()];
        find_kinds.extend(end_token_kinds.iter().map(|&k| Proto::from(k)));
        loop {
            index = tokens.get_next_token_of_kind(index, &find_kinds, true).unwrap();
            if tokens[index].is_given_kind(end_token_kinds) {
                return index;
            }
            if self.is_start_of_alternative_syntax_block(tokens, index) {
                index = self.find_alternative_syntax_block_end(tokens, index);
            }
        }
    }

    fn is_start_of_alternative_syntax_block(&self, tokens: &Tokens, index: usize) -> bool {
        let Some(start_token_kind) = tokens[index].get_id() else { return false };
        if alternative_syntax_block_edges(start_token_kind).is_none() {
            return false;
        }
        let mut index = tokens.get_next_meaningful_token(index).unwrap();
        if tokens[index].equals("(", true) {
            index = tokens.get_next_meaningful_token(tokens.find_block_end(Tokens::BLOCK_TYPE_PARENTHESIS, index)).unwrap();
        }
        tokens[index].equals(":", true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CODE: &[u8] = b"<?php
if ($a): foo(); elseif ($b): bar(); else: baz(); endif;
foreach ($x as $y): if ($y): z(); else: if ($q): endif; endif; endforeach;
for (;;): endfor;
while (1): endwhile;
switch ($a): case 1: break; endswitch;
declare(ticks=1): enddeclare;
$t = $a ? $b : $c;
if ($a) { } else { }
foo(1) ?: 2;
if ($a): if ($b): endif; endif;
";

    // Expected values: the real AlternativeSyntaxAnalyzer on CODE, for every token index
    // (findAlternativeSyntaxBlockEnd only where it does not throw).
    #[test]
    fn matches_php() {
        let tokens = Tokens::from_code(CODE);
        let a = AlternativeSyntaxAnalyzer;
        let belongs: Vec<usize> = (0..tokens.count()).filter(|&i| a.belongs_to_alternative_syntax(&tokens, i)).collect();
        assert_eq!(belongs, [6, 18, 26, 45, 52, 60, 67, 84, 94, 104, 123, 175, 182]);
        let starts: Vec<usize> = (0..tokens.count()).filter(|&i| a.is_start_of_alternative_syntax_block(&tokens, i)).collect();
        let ends: Vec<(usize, usize)> = starts.iter().map(|&i| (i, a.find_alternative_syntax_block_end(&tokens, i))).collect();
        assert_eq!(ends, [(1, 13), (13, 25), (25, 33), (36, 75), (47, 59), (59, 69), (62, 69), (78, 86), (89, 96), (99, 114), (170, 187), (177, 184)]);
    }

    #[test]
    #[should_panic(expected = "There is no token at index 9999.")]
    fn end_of_missing_index() {
        AlternativeSyntaxAnalyzer.find_alternative_syntax_block_end(&Tokens::from_code(CODE), 9999);
    }

    #[test]
    #[should_panic(expected = "Token at index 0 is not the start of an alternative syntax block.")]
    fn end_of_non_block() {
        AlternativeSyntaxAnalyzer.find_alternative_syntax_block_end(&Tokens::from_code(CODE), 0);
    }
}
