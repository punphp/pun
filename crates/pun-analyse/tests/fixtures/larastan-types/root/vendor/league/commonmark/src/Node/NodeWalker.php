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

namespace League\CommonMark\Node;

use League\CommonMark\Node\Block\AbstractBlock;

final class NodeWalker
{
    /** @psalm-readonly */
    private Node $root;

    /** @psalm-readonly-allow-private-mutation */
    private ?Node $current = null;

    /** @psalm-readonly-allow-private-mutation */
    private bool $entering;

    public function __construct(Node $root)
    {}

    /**
     * Returns an event which contains node and entering flag
     * (entering is true when we enter a Node from a parent or sibling,
     * and false when we reenter it from child)
     */
    public function next(): ?NodeWalkerEvent
    {}

    /**
     * Resets the iterator to resume at the specified node
     */
    public function resumeAt(Node $node, bool $entering = true): void
    {}
}
