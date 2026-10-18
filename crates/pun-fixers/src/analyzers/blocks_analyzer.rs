//! Port of `PhpCsFixer\Tokenizer\Analyzer\BlocksAnalyzer`.
use pun_tokens::{BlockType, Token, Tokens};

pub struct BlocksAnalyzer;

impl BlocksAnalyzer {
    pub fn is_block(&self, tokens: &Tokens, open_index: usize, close_index: usize) -> bool {
        assert!(
            open_index < tokens.len(),
            "Tokex index {open_index} for potential block opening does not exist."
        );
        assert!(
            close_index < tokens.len(),
            "Token index {close_index} for potential block closure does not exist."
        );
        let Some(block_type) = Self::get_block_type(&tokens[open_index]) else {
            return false;
        };
        close_index == tokens.find_block_end(block_type, open_index)
    }

    /// First block type (in `getBlockEdgeDefinitions` order) whose start the token equals.
    fn get_block_type(token: &Token) -> Option<BlockType> {
        BlockType::ALL
            .into_iter()
            .find(|&t| token.equals(Tokens::block_edges(t).0, true))
    }
}
