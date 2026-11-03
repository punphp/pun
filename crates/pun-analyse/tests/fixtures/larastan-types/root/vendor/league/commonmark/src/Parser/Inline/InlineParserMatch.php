<?php

declare(strict_types=1);

/*
 * This file is part of the league/commonmark package.
 *
 * (c) Colin O'Dell <colinodell@gmail.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace League\CommonMark\Parser\Inline;

use League\CommonMark\Exception\InvalidArgumentException;

final class InlineParserMatch
{
    private string $regex;

    private bool $caseSensitive;

    private function __construct(string $regex, bool $caseSensitive = false)
    {}

    public function caseSensitive(): self
    {}

    /**
     * @internal
     *
     * @psalm-return non-empty-string
     */
    public function getRegex(): string
    {}

    /**
     * Match the given string (case-insensitive)
     */
    public static function string(string $str): self
    {}

    /**
     * Match any of the given strings (case-insensitive)
     */
    public static function oneOf(string ...$str): self
    {}

    /**
     * Match a partial regular expression without starting/ending delimiters, anchors, or flags
     */
    public static function regex(string $regex): self
    {}

    public static function join(self ...$definitions): self
    {}
}
