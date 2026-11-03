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

namespace League\CommonMark\Parser\Block;

use League\CommonMark\Node\Block\Paragraph;
use League\CommonMark\Parser\Cursor;
use League\CommonMark\Parser\InlineParserEngineInterface;
use League\CommonMark\Reference\ReferenceInterface;
use League\CommonMark\Reference\ReferenceParser;

final class ParagraphParser extends AbstractBlockContinueParser implements BlockContinueParserWithInlinesInterface
{
    /** @psalm-readonly */
    private Paragraph $block;

    /** @psalm-readonly */
    private ReferenceParser $referenceParser;

    public function __construct()
    {}

    public function canHaveLazyContinuationLines(): bool
    {}

    public function getBlock(): Paragraph
    {}

    public function tryContinue(Cursor $cursor, BlockContinueParserInterface $activeBlockParser): ?BlockContinue
    {}

    public function addLine(string $line): void
    {}

    public function closeBlock(): void
    {}

    public function parseInlines(InlineParserEngineInterface $inlineParser): void
    {}

    public function getContentString(): string
    {}

    /**
     * @return ReferenceInterface[]
     */
    public function getReferences(): iterable
    {}
}
