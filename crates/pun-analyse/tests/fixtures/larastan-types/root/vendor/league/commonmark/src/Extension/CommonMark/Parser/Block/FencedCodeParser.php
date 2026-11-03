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

use League\CommonMark\Extension\CommonMark\Node\Block\FencedCode;
use League\CommonMark\Parser\Block\AbstractBlockContinueParser;
use League\CommonMark\Parser\Block\BlockContinue;
use League\CommonMark\Parser\Block\BlockContinueParserInterface;
use League\CommonMark\Parser\Cursor;
use League\CommonMark\Util\ArrayCollection;
use League\CommonMark\Util\RegexHelper;

final class FencedCodeParser extends AbstractBlockContinueParser
{
    /** @psalm-readonly */
    private FencedCode $block;

    /** @var ArrayCollection<string> */
    private ArrayCollection $strings;

    public function __construct(int $fenceLength, string $fenceChar, int $fenceOffset)
    {}

    public function getBlock(): FencedCode
    {}

    public function tryContinue(Cursor $cursor, BlockContinueParserInterface $activeBlockParser): ?BlockContinue
    {}

    public function addLine(string $line): void
    {}

    public function closeBlock(): void
    {}
}
