<?php declare(strict_types=1);

namespace PhpParser\Node\Scalar;

use PhpParser\Error;
use PhpParser\Node\Scalar;

class String_ extends Scalar {
    /* For use in "kind" attribute */
    public const KIND_SINGLE_QUOTED = 1;
    public const KIND_DOUBLE_QUOTED = 2;
    public const KIND_HEREDOC = 3;
    public const KIND_NOWDOC = 4;

    /** @var string String value */
    public string $value;

    /** @var array<string, string> Escaped character to its decoded value */
    protected static array $replacements = [
        '\\' => '\\',
        '$'  =>  '$',
        'n'  => "\n",
        'r'  => "\r",
        't'  => "\t",
        'f'  => "\f",
        'v'  => "\v",
        'e'  => "\x1B",
    ];

    /**
     * Constructs a string scalar node.
     *
     * @param string $value Value of the string
     * @param array<string, mixed> $attributes Additional attributes
     */
    public function __construct(string $value, array $attributes = []) {}

    public function getSubNodeNames(): array {}

    /**
     * @param array<string, mixed> $attributes
     * @param bool $parseUnicodeEscape Whether to parse PHP 7 \u escapes
     */
    public static function fromString(string $str, array $attributes = [], bool $parseUnicodeEscape = true): self {}

    /**
     * @internal
     *
     * Parses a string token.
     *
     * @param string $str String token content
     * @param bool $parseUnicodeEscape Whether to parse PHP 7 \u escapes
     *
     * @return string The parsed string
     */
    public static function parse(string $str, bool $parseUnicodeEscape = true): string {}

    /**
     * @internal
     *
     * Parses escape sequences in strings (all string types apart from single quoted).
     *
     * @param string $str String without quotes
     * @param null|string $quote Quote type
     * @param bool $parseUnicodeEscape Whether to parse PHP 7 \u escapes
     *
     * @return string String with escape sequences parsed
     */
    public static function parseEscapeSequences(string $str, ?string $quote, bool $parseUnicodeEscape = true): string {}

    /**
     * Converts a Unicode code point to its UTF-8 encoded representation.
     *
     * @param int $num Code point
     *
     * @return string UTF-8 representation of code point
     */
    private static function codePointToUtf8(int $num): string {}

    public function getType(): string {}
}
