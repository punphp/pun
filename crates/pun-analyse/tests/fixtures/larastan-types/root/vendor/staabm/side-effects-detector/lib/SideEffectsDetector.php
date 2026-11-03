<?php

namespace staabm\SideEffectsDetector;

final class SideEffectsDetector {
    /**
     * @var array<int>
     */
    private array $scopePollutingTokens = [
        T_CLASS,
        T_FUNCTION,
        T_NEW,
        T_EVAL,
        T_GLOBAL,
        T_GOTO,
        T_HALT_COMPILER,
        T_INCLUDE,
        T_INCLUDE_ONCE,
        T_REQUIRE,
        T_REQUIRE_ONCE,
        T_THROW,
        T_UNSET,
        T_UNSET_CAST
    ];

    private const PROCESS_EXIT_TOKENS = [
        T_EXIT
    ];

    private const OUTPUT_TOKENS = [
        T_PRINT,
        T_ECHO,
        T_INLINE_HTML
    ];

    private const SCOPE_POLLUTING_FUNCTIONS = [
        'putenv',
        'setlocale',
        'class_exists',
        'ini_set',
    ];

    private const STANDARD_OUTPUT_FUNCTIONS = [
        'printf',
        'vprintf'
    ];

    private const INPUT_OUTPUT_FUNCTIONS = [
        'fopen',
        'file_get_contents',
        'file_put_contents',
        'fwrite',
        'fputs',
        'fread',
        'unlink'
    ];

    /**
     * @var array<string, array{'hasSideEffects': bool}>
     */
    private array $functionMetadata;

    public function __construct() {}

    /**
     * @api
     *
     * @return array<SideEffect::*>
     */
    public function getSideEffects(string $code): array {}

    /**
     * @return SideEffect::*|null
     */
    private function getFunctionCallSideEffect(string $functionName): ?string {}

    /**
     * @param array<int, array{0:int,1:string,2:int}|string|int> $tokens
     */
    private function getFunctionCall(array $tokens, int $index): ?string {}

    /**
     * @param array<int, array{0:int,1:string,2:int}|string|int> $tokens
     */
    private function getMethodCall(array $tokens, int $index): ?string {}

    /**
     * @param array<int, array{0:int,1:string,2:int}|string|int> $tokens
     */
    private function getPropertyAccess(array $tokens, int $index): ?string {}

    /**
     * @param array<int, array{0:int,1:string,2:int}|string|int> $tokens
     */
    private function isAnonymousFunction(array $tokens, int $index): bool
    {}

    /**
     * @param array<int, array{0:int,1:string,2:int}|string|int> $tokens
     */
    private function isNonLocalVariable(array $tokens, int $index): bool
    {}

    /**
     * @param array<int, array{0:int,1:string,2:int}|string|int> $tokens
     */
    private function consumeWhitespaces(array $tokens, int &$index): void {}
}
