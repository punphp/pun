<?php declare(strict_types=1);

namespace PhpParser\Lexer;

use PhpParser\Error;
use PhpParser\ErrorHandler;
use PhpParser\Lexer;
use PhpParser\Lexer\TokenEmulator\AsymmetricVisibilityTokenEmulator;
use PhpParser\Lexer\TokenEmulator\AttributeEmulator;
use PhpParser\Lexer\TokenEmulator\EnumTokenEmulator;
use PhpParser\Lexer\TokenEmulator\ExplicitOctalEmulator;
use PhpParser\Lexer\TokenEmulator\FnTokenEmulator;
use PhpParser\Lexer\TokenEmulator\MatchTokenEmulator;
use PhpParser\Lexer\TokenEmulator\NullsafeTokenEmulator;
use PhpParser\Lexer\TokenEmulator\PipeOperatorEmulator;
use PhpParser\Lexer\TokenEmulator\PropertyTokenEmulator;
use PhpParser\Lexer\TokenEmulator\ReadonlyFunctionTokenEmulator;
use PhpParser\Lexer\TokenEmulator\ReadonlyTokenEmulator;
use PhpParser\Lexer\TokenEmulator\ReverseEmulator;
use PhpParser\Lexer\TokenEmulator\TokenEmulator;
use PhpParser\Lexer\TokenEmulator\VoidCastEmulator;
use PhpParser\PhpVersion;
use PhpParser\Token;

class Emulative extends Lexer {
    /** @var array{int, string, string}[] Patches used to reverse changes introduced in the code */
    private array $patches = [];

    /** @var list<TokenEmulator> */
    private array $emulators = [];

    private PhpVersion $targetPhpVersion;

    private PhpVersion $hostPhpVersion;

    /**
     * @param PhpVersion|null $phpVersion PHP version to emulate. Defaults to newest supported.
     */
    public function __construct(?PhpVersion $phpVersion = null) {}

    public function tokenize(string $code, ?ErrorHandler $errorHandler = null): array {}

    private function isForwardEmulationNeeded(PhpVersion $emulatorPhpVersion): bool {}

    private function isReverseEmulationNeeded(PhpVersion $emulatorPhpVersion): bool {}

    private function sortPatches(): void {}

    /**
     * @param list<Token> $tokens
     * @return list<Token>
     */
    private function fixupTokens(array $tokens): array {}

    /**
     * Fixup line and position information in errors.
     *
     * @param Error[] $errors
     */
    private function fixupErrors(array $errors): void {}
}
