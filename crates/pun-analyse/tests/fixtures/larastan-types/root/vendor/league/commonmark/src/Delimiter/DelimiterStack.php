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
 * Additional emphasis processing code based on commonmark-java (https://github.com/atlassian/commonmark-java)
 *  - (c) Atlassian Pty Ltd
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace League\CommonMark\Delimiter;

use League\CommonMark\Delimiter\Processor\CacheableDelimiterProcessorInterface;
use League\CommonMark\Delimiter\Processor\DelimiterProcessorCollection;
use League\CommonMark\Node\Inline\AdjacentTextMerger;
use League\CommonMark\Node\Node;

final class DelimiterStack
{
    /** @psalm-readonly-allow-private-mutation */
    private ?DelimiterInterface $top = null;

    /** @psalm-readonly-allow-private-mutation */
    private ?Bracket $brackets = null;

    /**
     * @deprecated This property will be removed in 3.0 once all delimiters MUST have an index/position
     *
     * @var \SplObjectStorage<DelimiterInterface, int>|\WeakMap<DelimiterInterface, int>
     */
    private $missingIndexCache;


    private int $remainingDelimiters = 0;

    public function __construct(int $maximumStackSize = PHP_INT_MAX)
    {}

    public function push(DelimiterInterface $newDelimiter): void
    {}

    /**
     * @internal
     */
    public function addBracket(Node $node, int $index, bool $image): void
    {}

    /**
     * @psalm-immutable
     */
    public function getLastBracket(): ?Bracket
    {}

    private function findEarliest(int $stackBottom): ?DelimiterInterface
    {}

    /**
     * @internal
     */
    public function removeBracket(): void
    {}

    public function removeDelimiter(DelimiterInterface $delimiter): void
    {}

    private function removeDelimiterAndNode(DelimiterInterface $delimiter): void
    {}

    private function removeDelimitersBetween(DelimiterInterface $opener, DelimiterInterface $closer): void
    {}

    /**
     * @param DelimiterInterface|int|null $stackBottom
     */
    public function removeAll($stackBottom = null): void
    {}

    /**
     * @deprecated This method is no longer used internally and will be removed in 3.0
     */
    public function removeEarlierMatches(string $character): void
    {}

    /**
     * @internal
     */
    public function deactivateLinkOpeners(): void
    {}

    /**
     * @deprecated This method is no longer used internally and will be removed in 3.0
     *
     * @param string|string[] $characters
     */
    public function searchByCharacter($characters): ?DelimiterInterface
    {}

    /**
     * @param DelimiterInterface|int|null $stackBottom
     *
     * @todo change $stackBottom to an int in 3.0
     */
    public function processDelimiters($stackBottom, DelimiterProcessorCollection $processors): void
    {}

    /**
     * @internal
     */
    public function __destruct()
    {}

    /**
     * @deprecated This method will be dropped in 3.0 once all delimiters MUST have an index/position
     */
    private function getIndex(?DelimiterInterface $delimiter): int
    {}
}
