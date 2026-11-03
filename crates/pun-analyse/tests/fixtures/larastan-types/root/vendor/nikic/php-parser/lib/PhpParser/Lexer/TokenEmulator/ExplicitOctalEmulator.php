<?php declare(strict_types=1);

namespace PhpParser\Lexer\TokenEmulator;

use PhpParser\PhpVersion;
use PhpParser\Token;

class ExplicitOctalEmulator extends TokenEmulator {
    public function getPhpVersion(): PhpVersion {}

    public function isEmulationNeeded(string $code): bool {}

    public function emulate(string $code, array $tokens): array {}

    private function resolveIntegerOrFloatToken(string $str): int {}

    public function reverseEmulate(string $code, array $tokens): array {}
}
