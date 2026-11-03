<?php declare(strict_types=1);

namespace PhpParser\Lexer\TokenEmulator;

use PhpParser\PhpVersion;
use PhpParser\Token;

final class AsymmetricVisibilityTokenEmulator extends TokenEmulator {
    public function getPhpVersion(): PhpVersion {}
    public function isEmulationNeeded(string $code): bool {}

    public function emulate(string $code, array $tokens): array {}

    public function reverseEmulate(string $code, array $tokens): array {}

    /** @param Token[] $tokens */
    protected function isKeywordContext(array $tokens, int $pos): bool {}

    /** @param Token[] $tokens */
    private function getPreviousNonSpaceToken(array $tokens, int $start): ?Token {}
}
