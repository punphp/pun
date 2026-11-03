<?php declare(strict_types=1);

namespace PhpParser\Internal;

if (\PHP_VERSION_ID >= 80000) {
    class TokenPolyfill extends \PhpToken {
    }
    return;
}

/**
 * This is a polyfill for the PhpToken class introduced in PHP 8.0. We do not actually polyfill
 * PhpToken, because composer might end up picking a different polyfill implementation, which does
 * not meet our requirements.
 *
 * @internal
 */
class TokenPolyfill {
    /** @var int The ID of the token. Either a T_* constant of a character code < 256. */
    public int $id;
    /** @var string The textual content of the token. */
    public string $text;
    /** @var int The 1-based starting line of the token (or -1 if unknown). */
    public int $line;
    /** @var int The 0-based starting position of the token (or -1 if unknown). */
    public int $pos;

    /** @var array<int, bool> Tokens ignored by the PHP parser. */
    private const IGNORABLE_TOKENS = [
        \T_WHITESPACE => true,
        \T_COMMENT => true,
        \T_DOC_COMMENT => true,
        \T_OPEN_TAG => true,
    ];

    /** @var array<int, bool> Tokens that may be part of a T_NAME_* identifier. */
    private static array $identifierTokens;

    /**
     * Create a Token with the given ID and text, as well optional line and position information.
     */
    final public function __construct(int $id, string $text, int $line = -1, int $pos = -1) {}

    /**
     * Get the name of the token. For single-char tokens this will be the token character.
     * Otherwise it will be a T_* style name, or null if the token ID is unknown.
     */
    public function getTokenName(): ?string {}

    /**
     * Check whether the token is of the given kind. The kind may be either an integer that matches
     * the token ID, a string that matches the token text, or an array of integers/strings. In the
     * latter case, the function returns true if any of the kinds in the array match.
     *
     * @param int|string|(int|string)[] $kind
     */
    public function is($kind): bool {}

    /**
     * Check whether this token would be ignored by the PHP parser. Returns true for T_WHITESPACE,
     * T_COMMENT, T_DOC_COMMENT and T_OPEN_TAG, and false for everything else.
     */
    public function isIgnorable(): bool {}

    /**
     * Return the textual content of the token.
     */
    public function __toString(): string {}

    /**
     * Tokenize the given source code and return an array of tokens.
     *
     * This performs certain canonicalizations to match the PHP 8.0 token format:
     *  * Bad characters are represented using T_BAD_CHARACTER rather than omitted.
     *  * T_COMMENT does not include trailing newlines, instead the newline is part of a following
     *    T_WHITESPACE token.
     *  * Namespaced names are represented using T_NAME_* tokens.
     *
     * @return static[]
     */
    public static function tokenize(string $code, int $flags = 0): array {}

    /** Initialize private static state needed by tokenize(). */
    private static function init(): void {}
}
