<?php

declare(strict_types=1);

/*
 * This is part of the league/commonmark package.
 *
 * (c) Martin Hasoň <martin.hason@gmail.com>
 * (c) Webuni s.r.o. <info@webuni.cz>
 * (c) Colin O'Dell <colinodell@gmail.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace League\CommonMark\Extension\Table;

use League\CommonMark\Parser\Block\BlockStart;
use League\CommonMark\Parser\Block\BlockStartParserInterface;
use League\CommonMark\Parser\Block\ParagraphParser;
use League\CommonMark\Parser\Cursor;
use League\CommonMark\Parser\MarkdownParserStateInterface;

final class TableStartParser implements BlockStartParserInterface
{
    private int $maxAutocompletedCells;

    public function __construct(int $maxAutocompletedCells = TableParser::DEFAULT_MAX_AUTOCOMPLETED_CELLS)
    {}

    public function tryStart(Cursor $cursor, MarkdownParserStateInterface $parserState): ?BlockStart
    {}

    /**
     * @return array<int, string|null>
     *
     * @psalm-return array<int, TableCell::ALIGN_*|null>
     *
     * @phpstan-return array<int, TableCell::ALIGN_*|null>
     */
    private static function parseSeparator(Cursor $cursor): array
    {}

    /**
     * @psalm-return TableCell::ALIGN_*|null
     *
     * @phpstan-return TableCell::ALIGN_*|null
     *
     * @psalm-pure
     */
    private static function getAlignment(bool $left, bool $right): ?string
    {}
}
