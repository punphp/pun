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

use League\CommonMark\Parser\Block\AbstractBlockContinueParser;
use League\CommonMark\Parser\Block\BlockContinue;
use League\CommonMark\Parser\Block\BlockContinueParserInterface;
use League\CommonMark\Parser\Block\BlockContinueParserWithInlinesInterface;
use League\CommonMark\Parser\Cursor;
use League\CommonMark\Parser\InlineParserEngineInterface;
use League\CommonMark\Util\ArrayCollection;

final class TableParser extends AbstractBlockContinueParser implements BlockContinueParserWithInlinesInterface
{
    /**
     * @internal
     */
    public const DEFAULT_MAX_AUTOCOMPLETED_CELLS = 10_000;

    /** @psalm-readonly */
    private Table $block;

    /**
     * @var ArrayCollection<string>
     *
     * @psalm-readonly-allow-private-mutation
     */
    private ArrayCollection $bodyLines;

    /**
     * @var array<int, string|null>
     * @psalm-var array<int, TableCell::ALIGN_*|null>
     * @phpstan-var array<int, TableCell::ALIGN_*|null>
     *
     * @psalm-readonly
     */
    private array $columns;

    /**
     * @var array<int, string>
     *
     * @psalm-readonly-allow-private-mutation
     */
    private array $headerCells;

    /** @psalm-readonly-allow-private-mutation */
    private bool $nextIsSeparatorLine = true;

    private int $remainingAutocompletedCells;

    /**
     * Whether the current line may be added to the table as a lazy continuation line
     *
     * @psalm-readonly-allow-private-mutation
     */
    private bool $canHaveLazyContinuationLines = false;

    /**
     * @param array<int, string|null> $columns
     * @param array<int, string>      $headerCells
     *
     * @psalm-param array<int, TableCell::ALIGN_*|null> $columns
     *
     * @phpstan-param array<int, TableCell::ALIGN_*|null> $columns
     */
    public function __construct(array $columns, array $headerCells, int $remainingAutocompletedCells = self::DEFAULT_MAX_AUTOCOMPLETED_CELLS)
    {}

    public function canHaveLazyContinuationLines(): bool
    {}

    public function getBlock(): Table
    {}

    public function tryContinue(Cursor $cursor, BlockContinueParserInterface $activeBlockParser): ?BlockContinue
    {}

    public function addLine(string $line): void
    {}

    public function parseInlines(InlineParserEngineInterface $inlineParser): void
    {}

    private function parseCell(string $cell, int $column, InlineParserEngineInterface $inlineParser): TableCell
    {}

    /**
     * @internal
     *
     * @return array<int, string>
     */
    public static function split(string $line): array
    {}
}
