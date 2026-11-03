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

namespace League\CommonMark\Reference;

final class MemoryLimitedReferenceMap implements ReferenceMapInterface
{
    private ReferenceMapInterface $decorated;

    private const MINIMUM_SIZE = 100_000;

    private int $remaining;

    public function __construct(ReferenceMapInterface $decorated, int $maxSize)
    {}

    public function add(ReferenceInterface $reference): void
    {}

    public function contains(string $label): bool
    {}

    public function get(string $label): ?ReferenceInterface
    {}

    /**
     * @return \Traversable<string, ReferenceInterface>
     */
    public function getIterator(): \Traversable
    {}

    public function count(): int
    {}
}
