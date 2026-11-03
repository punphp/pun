<?php declare(strict_types=1);

namespace PhpParser\Lexer\TokenEmulator;

use PhpParser\PhpVersion;

final class EnumTokenEmulator extends KeywordEmulator {
    public function getPhpVersion(): PhpVersion {}

    public function getKeywordString(): string {}

    public function getKeywordToken(): int {}

    protected function isKeywordContext(array $tokens, int $pos): bool {}
}
