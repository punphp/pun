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

namespace League\CommonMark\Parser;

use League\CommonMark\Parser\Block\BlockContinueParserInterface;
use League\CommonMark\Parser\Block\ParagraphParser;

/**
 * @internal You should rely on the interface instead
 */
final class MarkdownParserState implements MarkdownParserStateInterface
{
    /** @psalm-readonly */
    private BlockContinueParserInterface $activeBlockParser;

    /** @psalm-readonly */
    private BlockContinueParserInterface $lastMatchedBlockParser;

    public function __construct(BlockContinueParserInterface $activeBlockParser, BlockContinueParserInterface $lastMatchedBlockParser)
    {}

    public function getActiveBlockParser(): BlockContinueParserInterface
    {}

    public function getLastMatchedBlockParser(): BlockContinueParserInterface
    {}

    public function getParagraphContent(): ?string
    {}
}
