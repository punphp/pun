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

namespace League\CommonMark\Extension\CommonMark\Parser\Block;

use League\CommonMark\Extension\CommonMark\Node\Block\ListBlock;
use League\CommonMark\Extension\CommonMark\Node\Block\ListData;
use League\CommonMark\Extension\CommonMark\Node\Block\ListItem;
use League\CommonMark\Node\Block\AbstractBlock;
use League\CommonMark\Parser\Block\AbstractBlockContinueParser;
use League\CommonMark\Parser\Block\BlockContinue;
use League\CommonMark\Parser\Block\BlockContinueParserInterface;
use League\CommonMark\Parser\Cursor;

final class ListBlockParser extends AbstractBlockContinueParser
{
    /** @psalm-readonly */
    private ListBlock $block;

    public function __construct(ListData $listData)
    {}

    public function getBlock(): ListBlock
    {}

    public function isContainer(): bool
    {}

    public function canContain(AbstractBlock $childBlock): bool
    {}

    public function tryContinue(Cursor $cursor, BlockContinueParserInterface $activeBlockParser): ?BlockContinue
    {}

    public function closeBlock(): void
    {}

    private static function endsWithBlankLine(AbstractBlock $block): bool
    {}
}
