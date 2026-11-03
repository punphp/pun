<?php

declare(strict_types=1);

/*
 * This file is part of the league/commonmark package.
 *
 * (c) Colin O'Dell <colinodell@gmail.com>
 *
 * Original code based on the CommonMark JS reference parser (https://bitly.com/commonmark-js)
 *  - (c) John MacFarlane
 *
 * Additional code based on commonmark-java (https://github.com/commonmark/commonmark-java)
 *  - (c) Atlassian Pty Ltd
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace League\CommonMark\Parser;

use League\CommonMark\Environment\EnvironmentInterface;
use League\CommonMark\Event\DocumentParsedEvent;
use League\CommonMark\Event\DocumentPreParsedEvent;
use League\CommonMark\Exception\CommonMarkException;
use League\CommonMark\Input\MarkdownInput;
use League\CommonMark\Node\Block\Document;
use League\CommonMark\Node\Block\Paragraph;
use League\CommonMark\Parser\Block\BlockContinueParserInterface;
use League\CommonMark\Parser\Block\BlockContinueParserWithInlinesInterface;
use League\CommonMark\Parser\Block\BlockStart;
use League\CommonMark\Parser\Block\BlockStartParserInterface;
use League\CommonMark\Parser\Block\DocumentBlockParser;
use League\CommonMark\Parser\Block\ParagraphParser;
use League\CommonMark\Reference\MemoryLimitedReferenceMap;
use League\CommonMark\Reference\ReferenceInterface;
use League\CommonMark\Reference\ReferenceMap;

final class MarkdownParser implements MarkdownParserInterface
{
    /** @psalm-readonly */
    private EnvironmentInterface $environment;

    /** @psalm-readonly-allow-private-mutation */
    private int $maxNestingLevel;

    /** @psalm-readonly-allow-private-mutation */
    private ReferenceMap $referenceMap;

    /** @psalm-readonly-allow-private-mutation */
    private int $lineNumber = 0;

    /** @psalm-readonly-allow-private-mutation */
    private Cursor $cursor;

    /**
     * @var array<int, BlockContinueParserInterface>
     *
     * @psalm-readonly-allow-private-mutation
     */
    private array $activeBlockParsers = [];

    /**
     * @var array<int, BlockContinueParserWithInlinesInterface>
     *
     * @psalm-readonly-allow-private-mutation
     */
    private array $closedBlockParsers = [];

    public function __construct(EnvironmentInterface $environment)
    {}

    private function initialize(): void
    {}

    /**
     * @throws CommonMarkException
     */
    public function parse(string $input): Document
    {}

    /**
     * Analyze a line of text and update the document appropriately. We parse markdown text by calling this on each
     * line of input, then finalizing the document.
     */
    private function parseLine(string $line): void
    {}

    private function parseBlockContinuation(): ?int
    {}

    private function findBlockStart(BlockContinueParserInterface $lastMatchedBlockParser): ?BlockStart
    {}

    private function closeBlockParsers(int $count, int $endLineNumber): void
    {}

    /**
     * Finalize a block. Close it and do any necessary postprocessing, e.g. creating string_content from strings,
     * setting the 'tight' or 'loose' status of a list, and parsing the beginnings of paragraphs for reference
     * definitions.
     */
    private function finalize(BlockContinueParserInterface $blockParser, int $endLineNumber): void
    {}

    /**
     * Walk through a block & children recursively, parsing string content into inline content where appropriate.
     */
    private function processInlines(int $inputSize): void
    {}

    /**
     * Add block of type tag as a child of the tip. If the tip can't accept children, close and finalize it and try
     * its parent, and so on til we find a block that can accept children.
     */
    private function addChild(BlockContinueParserInterface $blockParser, ?int $startLineNumber = null): BlockContinueParserInterface
    {}

    private function activateBlockParser(BlockContinueParserInterface $blockParser): void
    {}

    /**
     * @throws ParserLogicException
     */
    private function deactivateBlockParser(): BlockContinueParserInterface
    {}

    /**
     * @return int|null The line number where the old block started
     */
    private function prepareActiveBlockParserForReplacement(): ?int
    {}

    /**
     * @param ReferenceInterface[] $references
     */
    private function updateReferenceMap(iterable $references): void
    {}

    /**
     * @throws ParserLogicException
     */
    public function getActiveBlockParser(): BlockContinueParserInterface
    {}
}
