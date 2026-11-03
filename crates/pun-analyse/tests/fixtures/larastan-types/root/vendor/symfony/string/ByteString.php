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

use Random\Randomizer;
use Symfony\Component\String\Exception\ExceptionInterface;
use Symfony\Component\String\Exception\InvalidArgumentException;
use Symfony\Component\String\Exception\RuntimeException;

/**
 * Represents a binary-safe string of bytes.
 *
 * @author Nicolas Grekas <p@tchwork.com>
 * @author Hugo Hamon <hugohamon@neuf.fr>
 *
 * @throws ExceptionInterface
 */
class ByteString extends AbstractString
{
    private const ALPHABET_ALPHANUMERIC = '123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz';

    public function __construct(string $string = '')
    {}

    /*
     * The following method was derived from code of the Hack Standard Library (v4.40 - 2020-05-03)
     *
     * https://github.com/hhvm/hsl/blob/80a42c02f036f72a42f0415e80d6b847f4bf62d5/src/random/private.php#L16
     *
     * Code subject to the MIT license (https://github.com/hhvm/hsl/blob/master/LICENSE).
     *
     * Copyright (c) 2004-2020, Facebook, Inc. (https://www.facebook.com/)
     */

    public static function fromRandom(int $length = 16, ?string $alphabet = null): self
    {}

    public function bytesAt(int $offset): array
    {}

    public function append(string ...$suffix): static
    {}

    public function camel(): static
    {}

    public function chunk(int $length = 1): array
    {}

    public function endsWith(string|iterable|AbstractString $suffix): bool
    {}

    public function equalsTo(string|iterable|AbstractString $string): bool
    {}

    public function folded(): static
    {}

    public function indexOf(string|iterable|AbstractString $needle, int $offset = 0): ?int
    {}

    public function indexOfLast(string|iterable|AbstractString $needle, int $offset = 0): ?int
    {}

    public function isUtf8(): bool
    {}

    public function join(array $strings, ?string $lastGlue = null): static
    {}

    public function length(): int
    {}

    public function lower(): static
    {}

    public function match(string $regexp, int $flags = 0, int $offset = 0): array
    {}

    public function padBoth(int $length, string $padStr = ' '): static
    {}

    public function padEnd(int $length, string $padStr = ' '): static
    {}

    public function padStart(int $length, string $padStr = ' '): static
    {}

    public function prepend(string ...$prefix): static
    {}

    public function replace(string $from, string $to): static
    {}

    /**
     * @param-immediately-invoked-callable $to
     */
    public function replaceMatches(string $fromRegexp, string|callable $to): static
    {}

    public function reverse(): static
    {}

    public function slice(int $start = 0, ?int $length = null): static
    {}

    public function snake(): static
    {}

    public function splice(string $replacement, int $start = 0, ?int $length = null): static
    {}

    public function split(string $delimiter, ?int $limit = null, ?int $flags = null): array
    {}

    public function startsWith(string|iterable|AbstractString $prefix): bool
    {}

    public function title(bool $allWords = false): static
    {}

    public function toUnicodeString(?string $fromEncoding = null): UnicodeString
    {}

    public function toCodePointString(?string $fromEncoding = null): CodePointString
    {}

    public function trim(string $chars = " \t\n\r\0\x0B\x0C"): static
    {}

    public function trimEnd(string $chars = " \t\n\r\0\x0B\x0C"): static
    {}

    public function trimStart(string $chars = " \t\n\r\0\x0B\x0C"): static
    {}

    public function upper(): static
    {}

    public function width(bool $ignoreAnsiDecoration = true): int
    {}
}
