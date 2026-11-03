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

use League\CommonMark\Node\Block\AbstractBlock;
use League\CommonMark\Node\Block\Document;
use League\CommonMark\Node\Block\Paragraph;
use League\CommonMark\Parser\Cursor;
use League\CommonMark\Reference\ReferenceMapInterface;

/**
 * Parser implementation which ensures everything is added to the root-level Document
 */
final class DocumentBlockParser extends AbstractBlockContinueParser
{
    /** @psalm-readonly */
    private Document $document;

    public function __construct(ReferenceMapInterface $referenceMap)
    {}

    public function getBlock(): Document
    {}

    public function isContainer(): bool
    {}

    public function canContain(AbstractBlock $childBlock): bool
    {}

    public function tryContinue(Cursor $cursor, BlockContinueParserInterface $activeBlockParser): ?BlockContinue
    {}

    public function closeBlock(): void
    {}

    private function removeLinkReferenceDefinitions(): void
    {}
}
