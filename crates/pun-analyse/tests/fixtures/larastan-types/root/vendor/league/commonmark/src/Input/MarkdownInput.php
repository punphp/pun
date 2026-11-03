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

namespace League\CommonMark\Input;

use League\CommonMark\Exception\UnexpectedEncodingException;

class MarkdownInput implements MarkdownInputInterface
{
    /**
     * @var array<int, string>|null
     *
     * @psalm-readonly-allow-private-mutation
     */
    private ?array $lines = null;

    /** @psalm-readonly-allow-private-mutation */
    private string $content;

    /** @psalm-readonly-allow-private-mutation */
    private ?int $lineCount = null;

    /** @psalm-readonly */
    private int $lineOffset;

    public function __construct(string $content, int $lineOffset = 0)
    {}

    public function getContent(): string
    {}

    /**
     * {@inheritDoc}
     */
    public function getLines(): iterable
    { yield; }

    public function getLineCount(): int
    {}

    private function splitLinesIfNeeded(): void
    {}
}
