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
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace League\CommonMark\Parser;

use League\CommonMark\Environment\EnvironmentInterface;
use League\CommonMark\Node\Block\AbstractBlock;
use League\CommonMark\Node\Inline\AdjacentTextMerger;
use League\CommonMark\Node\Inline\Text;
use League\CommonMark\Parser\Inline\InlineParserInterface;
use League\CommonMark\Reference\ReferenceMapInterface;

/**
 * @internal
 */
final class InlineParserEngine implements InlineParserEngineInterface
{
    /** @psalm-readonly */
    private EnvironmentInterface $environment;

    /** @psalm-readonly */
    private ReferenceMapInterface $referenceMap;

    /**
     * @var array<int, InlineParserInterface|string|bool>
     * @psalm-var list<array{0: InlineParserInterface, 1: non-empty-string, 2: bool}>
     * @phpstan-var array<int, array{0: InlineParserInterface, 1: non-empty-string, 2: bool}>
     */
    private array $parsers = [];

    public function __construct(EnvironmentInterface $environment, ReferenceMapInterface $referenceMap)
    {}

    public function parse(string $contents, AbstractBlock $block): void
    {}

    private function addPlainText(string $text, AbstractBlock $container): void
    {}

    /**
     * Given the current line, ask all the parsers which parts of the text they would be interested in parsing.
     *
     * The resulting array provides a list of character positions, which parsers are interested in trying to parse
     * the text at those points, and (for convenience/optimization) what the matching text happened to be.
     *
     * @return array<array<int, InlineParserInterface|string>>
     *
     * @psalm-return array<int, list<array{0: InlineParserInterface, 1: non-empty-array<string>}>>
     *
     * @phpstan-return array<int, array<int, array{0: InlineParserInterface, 1: non-empty-array<string>}>>
     */
    private function matchParsers(string $contents): array
    {}
}
