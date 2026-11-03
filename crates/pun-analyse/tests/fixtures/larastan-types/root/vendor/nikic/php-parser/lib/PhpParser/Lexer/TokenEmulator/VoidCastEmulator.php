<?php declare(strict_types=1);

namespace PhpParser\Lexer\TokenEmulator;

use PhpParser\PhpVersion;
use PhpParser\Token;

class VoidCastEmulator extends TokenEmulator {
    public function getPhpVersion(): PhpVersion {}

    public function isEmulationNeeded(string $code): bool {}

    public function emulate(string $code, array $tokens): array {}

    public function reverseEmulate(string $code, array $tokens): array {}
}
