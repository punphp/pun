<?php declare(strict_types=1);

namespace PhpParser\Lexer\TokenEmulator;

use PhpParser\PhpVersion;

final class PropertyTokenEmulator extends KeywordEmulator {
    public function getPhpVersion(): PhpVersion {}

    public function getKeywordString(): string {}

    public function getKeywordToken(): int {}
}
