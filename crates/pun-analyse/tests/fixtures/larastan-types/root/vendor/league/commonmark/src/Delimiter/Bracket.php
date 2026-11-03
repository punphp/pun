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

namespace League\CommonMark\Delimiter;

use League\CommonMark\Node\Node;

final class Bracket
{
    private Node $node;
    private ?Bracket $previous;
    private bool $hasNext = false;
    private int $position;
    private bool $image;
    private bool $active = true;

    public function __construct(Node $node, ?Bracket $previous, int $position, bool $image)
    {}

    public function getNode(): Node
    {}

    public function getPrevious(): ?Bracket
    {}

    public function hasNext(): bool
    {}

    public function getPosition(): int
    {}

    public function isImage(): bool
    {}

    /**
     * Only valid in the context of non-images (links)
     */
    public function isActive(): bool
    {}

    /**
     * @internal
     */
    public function setHasNext(bool $hasNext): void
    {}

    /**
     * @internal
     */
    public function setActive(bool $active): void
    {}
}
