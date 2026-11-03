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

use Symfony\Component\Console\Output\OutputInterface;

/**
 * The TreeHelper class provides methods to display tree-like structures.
 *
 * @author Simon André <smn.andre@gmail.com>
 *
 * @implements \RecursiveIterator<int, TreeNode>
 */
final class TreeHelper implements \RecursiveIterator
{
    /**
     * @var \Iterator<int, TreeNode>
     */
    private \Iterator $children;

    private function __construct(
        private readonly OutputInterface $output,
        private readonly TreeNode $node,
        private readonly TreeStyle $style,
    ) {}

    public static function createTree(OutputInterface $output, string|TreeNode|null $root = null, iterable $values = [], ?TreeStyle $style = null): self
    {}

    public function current(): TreeNode
    {}

    public function key(): int
    {}

    public function next(): void
    {}

    public function rewind(): void
    {}

    public function valid(): bool
    {}

    public function hasChildren(): bool
    {}

    public function getChildren(): \RecursiveIterator
    {}

    /**
     * Recursively renders the tree to the output, applying the tree style.
     */
    public function render(): void
    {}
}
