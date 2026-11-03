<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\String;

use Symfony\Component\String\Exception\ExceptionInterface;
use Symfony\Component\String\Exception\InvalidArgumentException;
use Symfony\Component\String\Exception\RuntimeException;

/**
 * Represents a string of abstract characters.
 *
 * Unicode defines 3 types of "characters" (bytes, code points and grapheme clusters).
 * This class is the abstract type to use as a type-hint when the logic you want to
 * implement doesn't care about the exact variant it deals with.
 *
 * @author Nicolas Grekas <p@tchwork.com>
 * @author Hugo Hamon <hugohamon@neuf.fr>
 *
 * @throws ExceptionInterface
 */
abstract class AbstractString implements \Stringable, \JsonSerializable
{
    public const PREG_PATTERN_ORDER = \PREG_PATTERN_ORDER;
    public const PREG_SET_ORDER = \PREG_SET_ORDER;
    public const PREG_OFFSET_CAPTURE = \PREG_OFFSET_CAPTURE;
    public const PREG_UNMATCHED_AS_NULL = \PREG_UNMATCHED_AS_NULL;

    public const PREG_SPLIT = 0;
    public const PREG_SPLIT_NO_EMPTY = \PREG_SPLIT_NO_EMPTY;
    public const PREG_SPLIT_DELIM_CAPTURE = \PREG_SPLIT_DELIM_CAPTURE;
    public const PREG_SPLIT_OFFSET_CAPTURE = \PREG_SPLIT_OFFSET_CAPTURE;

    protected string $string = '';
    protected ?bool $ignoreCase = false;

    abstract public function __construct(string $string = '');

    /**
     * Unwraps instances of AbstractString back to strings.
     *
     * @return string[]|array
     */
    public static function unwrap(array $values): array
    {}

    /**
     * Wraps (and normalizes) strings in instances of AbstractString.
     *
     * @return static[]|array
     */
    public static function wrap(array $values): array
    {}

    /**
     * @param string|string[] $needle
     */
    public function after(string|iterable $needle, bool $includeNeedle = false, int $offset = 0): static
    {}

    /**
     * @param string|string[] $needle
     */
    public function afterLast(string|iterable $needle, bool $includeNeedle = false, int $offset = 0): static
    {}

    abstract public function append(string ...$suffix): static;

    /**
     * @param string|string[] $needle
     */
    public function before(string|iterable $needle, bool $includeNeedle = false, int $offset = 0): static
    {}

    /**
     * @param string|string[] $needle
     */
    public function beforeLast(string|iterable $needle, bool $includeNeedle = false, int $offset = 0): static
    {}

    /**
     * @return int[]
     */
    public function bytesAt(int $offset): array
    {}

    abstract public function camel(): static;

    /**
     * @return static[]
     */
    abstract public function chunk(int $length = 1): array;

    public function collapseWhitespace(): static
    {}

    /**
     * @param string|string[] $needle
     */
    public function containsAny(string|iterable $needle): bool
    {}

    /**
     * @param string|string[] $suffix
     */
    public function endsWith(string|iterable $suffix): bool
    {}

    public function ensureEnd(string $suffix): static
    {}

    public function ensureStart(string $prefix): static
    {}

    /**
     * @param string|string[] $string
     */
    public function equalsTo(string|iterable $string): bool
    {}

    abstract public function folded(): static;

    public function ignoreCase(): static
    {}

    /**
     * @param string|string[] $needle
     */
    public function indexOf(string|iterable $needle, int $offset = 0): ?int
    {}

    /**
     * @param string|string[] $needle
     */
    public function indexOfLast(string|iterable $needle, int $offset = 0): ?int
    {}

    public function isEmpty(): bool
    {}

    abstract public function join(array $strings, ?string $lastGlue = null): static;

    public function jsonSerialize(): string
    {}

    abstract public function length(): int;

    abstract public function lower(): static;

    /**
     * Matches the string using a regular expression.
     *
     * Pass PREG_PATTERN_ORDER or PREG_SET_ORDER as $flags to get all occurrences matching the regular expression.
     *
     * @return array All matches in a multi-dimensional array ordered according to flags
     */
    abstract public function match(string $regexp, int $flags = 0, int $offset = 0): array;

    abstract public function padBoth(int $length, string $padStr = ' '): static;

    abstract public function padEnd(int $length, string $padStr = ' '): static;

    abstract public function padStart(int $length, string $padStr = ' '): static;

    abstract public function prepend(string ...$prefix): static;

    public function repeat(int $multiplier): static
    {}

    abstract public function replace(string $from, string $to): static;

    /**
     * @param-immediately-invoked-callable $to
     */
    abstract public function replaceMatches(string $fromRegexp, string|callable $to): static;

    abstract public function reverse(): static;

    abstract public function slice(int $start = 0, ?int $length = null): static;

    abstract public function snake(): static;

    public function kebab(): static
    {}

    public function pascal(): static
    {}

    abstract public function splice(string $replacement, int $start = 0, ?int $length = null): static;

    /**
     * @return static[]
     */
    public function split(string $delimiter, ?int $limit = null, ?int $flags = null): array
    {}

    /**
     * @param string|string[] $prefix
     */
    public function startsWith(string|iterable $prefix): bool
    {}

    abstract public function title(bool $allWords = false): static;

    public function toByteString(?string $toEncoding = null): ByteString
    {}

    public function toCodePointString(): CodePointString
    {}

    public function toString(): string
    {}

    public function toUnicodeString(): UnicodeString
    {}

    abstract public function trim(string $chars = " \t\n\r\0\x0B\x0C\u{A0}\u{FEFF}"): static;

    abstract public function trimEnd(string $chars = " \t\n\r\0\x0B\x0C\u{A0}\u{FEFF}"): static;

    /**
     * @param string|string[] $prefix
     */
    public function trimPrefix($prefix): static
    {}

    abstract public function trimStart(string $chars = " \t\n\r\0\x0B\x0C\u{A0}\u{FEFF}"): static;

    /**
     * @param string|string[] $suffix
     */
    public function trimSuffix($suffix): static
    {}

    public function truncate(int $length, string $ellipsis = '', bool|TruncateMode $cut = TruncateMode::Char): static
    {}

    abstract public function upper(): static;

    /**
     * Returns the printable length on a terminal.
     */
    abstract public function width(bool $ignoreAnsiDecoration = true): int;

    public function wordwrap(int $width = 75, string $break = "\n", bool $cut = false): static
    {}

    public function __serialize(): array
    {}

    public function __clone()
    {}

    public function __toString(): string
    {}
}
