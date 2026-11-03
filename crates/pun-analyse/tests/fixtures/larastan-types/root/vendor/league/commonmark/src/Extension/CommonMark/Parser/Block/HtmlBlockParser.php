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

use League\CommonMark\Extension\CommonMark\Node\Block\HtmlBlock;
use League\CommonMark\Parser\Block\AbstractBlockContinueParser;
use League\CommonMark\Parser\Block\BlockContinue;
use League\CommonMark\Parser\Block\BlockContinueParserInterface;
use League\CommonMark\Parser\Cursor;
use League\CommonMark\Util\RegexHelper;

final class HtmlBlockParser extends AbstractBlockContinueParser
{
    /** @psalm-readonly */
    private HtmlBlock $block;

    private string $content = '';

    private bool $finished = false;

    /**
     * @psalm-param HtmlBlock::TYPE_* $blockType
     *
     * @phpstan-param HtmlBlock::TYPE_* $blockType
     */
    public function __construct(int $blockType)
    {}

    public function getBlock(): HtmlBlock
    {}

    public function tryContinue(Cursor $cursor, BlockContinueParserInterface $activeBlockParser): ?BlockContinue
    {}

    public function addLine(string $line): void
    {}

    public function closeBlock(): void
    {}
}
