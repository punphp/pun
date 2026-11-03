<?php declare(strict_types=1);

namespace PhpParser\Lexer\TokenEmulator;

use PhpParser\Token;

abstract class KeywordEmulator extends TokenEmulator {
    abstract public function getKeywordString(): string;
    abstract public function getKeywordToken(): int;

    public function isEmulationNeeded(string $code): bool {}

    /** @param Token[] $tokens */
    protected function isKeywordContext(array $tokens, int $pos): bool {}

    public function emulate(string $code, array $tokens): array {}

    /** @param Token[] $tokens */
    private function getPreviousNonIgnorableToken(array $tokens, int $start): ?Token {}

    public function reverseEmulate(string $code, array $tokens): array {}
}
