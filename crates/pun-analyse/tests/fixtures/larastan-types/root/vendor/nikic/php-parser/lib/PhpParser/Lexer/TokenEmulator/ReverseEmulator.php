<?php declare(strict_types=1);

namespace PhpParser\Lexer\TokenEmulator;

use PhpParser\PhpVersion;

/**
 * Reverses emulation direction of the inner emulator.
 */
final class ReverseEmulator extends TokenEmulator {
    /** @var TokenEmulator Inner emulator */
    private TokenEmulator $emulator;

    public function __construct(TokenEmulator $emulator) {}

    public function getPhpVersion(): PhpVersion {}

    public function isEmulationNeeded(string $code): bool {}

    public function emulate(string $code, array $tokens): array {}

    public function reverseEmulate(string $code, array $tokens): array {}

    public function preprocessCode(string $code, array &$patches): string {}
}
