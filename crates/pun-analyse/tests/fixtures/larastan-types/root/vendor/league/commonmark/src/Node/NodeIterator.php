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

namespace League\CommonMark\Node;

use League\CommonMark\Node\Block\AbstractBlock;

/**
 * @implements \IteratorAggregate<int, Node>
 */
final class NodeIterator implements \IteratorAggregate
{
    public const FLAG_BLOCKS_ONLY = 1;

    private Node $node;
    private bool $blocksOnly;

    public function __construct(Node $node, int $flags = 0)
    {}

    /**
     * @return \Generator<int, Node>
     */
    public function getIterator(): \Generator
    { yield; }
}
