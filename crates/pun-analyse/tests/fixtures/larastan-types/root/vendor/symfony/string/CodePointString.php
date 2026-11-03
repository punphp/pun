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
 * Represents a string of Unicode code points encoded as UTF-8.
 *
 * @author Nicolas Grekas <p@tchwork.com>
 * @author Hugo Hamon <hugohamon@neuf.fr>
 *
 * @throws ExceptionInterface
 */
class CodePointString extends AbstractUnicodeString
{
    public function __construct(string $string = '')
    {}

    public function append(string ...$suffix): static
    {}

    public function chunk(int $length = 1): array
    {}

    public function codePointsAt(int $offset): array
    {}

    public function endsWith(string|iterable|AbstractString $suffix): bool
    {}

    public function equalsTo(string|iterable|AbstractString $string): bool
    {}

    public function indexOf(string|iterable|AbstractString $needle, int $offset = 0): ?int
    {}

    public function indexOfLast(string|iterable|AbstractString $needle, int $offset = 0): ?int
    {}

    public function length(): int
    {}

    public function prepend(string ...$prefix): static
    {}

    public function replace(string $from, string $to): static
    {}

    public function slice(int $start = 0, ?int $length = null): static
    {}

    public function splice(string $replacement, int $start = 0, ?int $length = null): static
    {}

    public function split(string $delimiter, ?int $limit = null, ?int $flags = null): array
    {}

    public function startsWith(string|iterable|AbstractString $prefix): bool
    {}
}
