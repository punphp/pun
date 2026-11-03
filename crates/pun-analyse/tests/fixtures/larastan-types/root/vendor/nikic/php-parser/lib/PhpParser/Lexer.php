<?php declare(strict_types=1);

namespace PhpParser;

require __DIR__ . '/compatibility_tokens.php';

class Lexer {
    /**
     * Tokenize the provided source code.
     *
     * The token array is in the same format as provided by the PhpToken::tokenize() method in
     * PHP 8.0. The tokens are instances of PhpParser\Token, to abstract over a polyfill
     * implementation in earlier PHP version.
     *
     * The token array is terminated by a sentinel token with token ID 0.
     * The token array does not discard any tokens (i.e. whitespace and comments are included).
     * The token position attributes are against this token array.
     *
     * @param string $code The source code to tokenize.
     * @param ErrorHandler|null $errorHandler Error handler to use for lexing errors. Defaults to
     *                                        ErrorHandler\Throwing.
     * @return Token[] Tokens
     */
    public function tokenize(string $code, ?ErrorHandler $errorHandler = null): array {}

    private function handleInvalidCharacter(Token $token, ErrorHandler $errorHandler): void {}

    private function isUnterminatedComment(Token $token): bool {}

    /**
     * @param list<Token> $tokens
     */
    protected function postprocessTokens(array &$tokens, ErrorHandler $errorHandler): void {}
}
