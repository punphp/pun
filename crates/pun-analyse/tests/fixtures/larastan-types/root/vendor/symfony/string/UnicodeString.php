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

/**
 * Represents a string of Unicode grapheme clusters encoded as UTF-8.
 *
 * A letter followed by combining characters (accents typically) form what Unicode defines
 * as a grapheme cluster: a character as humans mean it in written texts. This class knows
 * about the concept and won't split a letter apart from its combining accents. It also
 * ensures all string comparisons happen on their canonically-composed representation,
 * ignoring e.g. the order in which accents are listed when a letter has many of them.
 *
 * @see https://unicode.org/reports/tr15/
 *
 * @author Nicolas Grekas <p@tchwork.com>
 * @author Hugo Hamon <hugohamon@neuf.fr>
 *
 * @throws ExceptionInterface
 */
class UnicodeString extends AbstractUnicodeString
{
    public function __construct(string $string = '')
    {}

    public function append(string ...$suffix): static
    {}

    public function chunk(int $length = 1): array
    {}

    public function endsWith(string|iterable|AbstractString $suffix): bool
    {}

    public function equalsTo(string|iterable|AbstractString $string): bool
    {}

    public function indexOf(string|iterable|AbstractString $needle, int $offset = 0): ?int
    {}

    public function indexOfLast(string|iterable|AbstractString $needle, int $offset = 0): ?int
    {}

    public function join(array $strings, ?string $lastGlue = null): static
    {}

    public function length(): int
    {}

    public function normalize(int $form = self::NFC): static
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

    public function slice(int $start = 0, ?int $length = null): static
    {}

    public function splice(string $replacement, int $start = 0, ?int $length = null): static
    {}

    public function split(string $delimiter, ?int $limit = null, ?int $flags = null): array
    {}

    public function startsWith(string|iterable|AbstractString $prefix): bool
    {}

    public function trimPrefix($prefix): static
    {}

    public function trimSuffix($suffix): static
    {}

    public function __unserialize(array $data): void
    {}

    public function __clone()
    {}

    /**
     * Returns the byte offset of the first occurrence of $needle in $haystack and sets $length to its length in bytes.
     */
    private function search(string $haystack, string $needle, ?int &$length = null): int|false
    {}
}
