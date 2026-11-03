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

use Dflydev\DotAccessData\Data;
use League\CommonMark\Exception\InvalidArgumentException;

abstract class Node
{
    /** @psalm-readonly-allow-private-mutation */
    public Data $data;

    /** @psalm-readonly-allow-private-mutation */
    protected int $depth = 0;

    /** @psalm-readonly-allow-private-mutation */
    protected ?Node $parent = null;

    /** @psalm-readonly-allow-private-mutation */
    protected ?Node $previous = null;

    /** @psalm-readonly-allow-private-mutation */
    protected ?Node $next = null;

    /** @psalm-readonly-allow-private-mutation */
    protected ?Node $firstChild = null;

    /** @psalm-readonly-allow-private-mutation */
    protected ?Node $lastChild = null;

    public function __construct()
    {}

    public function previous(): ?Node
    {}

    public function next(): ?Node
    {}

    public function parent(): ?Node
    {}

    protected function setParent(?Node $node = null): void
    {}

    /**
     * Inserts the $sibling node after $this
     */
    public function insertAfter(Node $sibling): void
    {}

    /**
     * Inserts the $sibling node before $this
     */
    public function insertBefore(Node $sibling): void
    {}

    public function replaceWith(Node $replacement): void
    {}

    public function detach(): void
    {}

    public function hasChildren(): bool
    {}

    public function firstChild(): ?Node
    {}

    public function lastChild(): ?Node
    {}

    /**
     * @return Node[]
     */
    public function children(): iterable
    {}

    public function appendChild(Node $child): void
    {}

    /**
     * Adds $child as the very first child of $this
     */
    public function prependChild(Node $child): void
    {}

    /**
     * Detaches all child nodes of given node
     */
    public function detachChildren(): void
    {}

    /**
     * Replace all children of given node with collection of another
     *
     * @param iterable<Node> $children
     */
    public function replaceChildren(iterable $children): void
    {}

    public function getDepth(): int
    {}

    public function walker(): NodeWalker
    {}

    public function iterator(int $flags = 0): NodeIterator
    {}

    /**
     * Clone the current node and its children
     *
     * WARNING: This is a recursive function and should not be called on deeply-nested node trees!
     */
    public function __clone()
    {}

    public static function assertInstanceOf(Node $node): void
    {}
}
