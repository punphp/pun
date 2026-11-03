<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Console\Helper;

/**
 * @implements \IteratorAggregate<TreeNode>
 *
 * @author Simon André <smn.andre@gmail.com>
 */
final class TreeNode implements \Countable, \IteratorAggregate
{
    /**
     * @var array<TreeNode|callable(): \Generator>
     */
    private array $children = [];

    public function __construct(
        private readonly string $value = '',
        iterable $children = [],
    ) {}

    public static function fromValues(iterable $nodes, ?self $node = null): self
    {}

    public function getValue(): string
    {}

    public function addChild(self|string|callable $node): self
    {}

    /**
     * @return \Traversable<int, TreeNode>
     */
    public function getChildren(): \Traversable
    { yield; }

    /**
     * @return \Traversable<int, TreeNode>
     */
    public function getIterator(): \Traversable
    {}

    public function count(): int
    {}

    public function __toString(): string
    {}
}
