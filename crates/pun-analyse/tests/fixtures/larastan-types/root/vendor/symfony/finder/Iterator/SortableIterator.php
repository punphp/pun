<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Finder\Iterator;

/**
 * SortableIterator applies a sort on a given Iterator.
 *
 * @author Fabien Potencier <fabien@symfony.com>
 *
 * @implements \IteratorAggregate<string, \SplFileInfo>
 */
class SortableIterator implements \IteratorAggregate
{
    public const SORT_BY_NONE = 0;
    public const SORT_BY_NAME = 1;
    public const SORT_BY_TYPE = 2;
    public const SORT_BY_ACCESSED_TIME = 3;
    public const SORT_BY_CHANGED_TIME = 4;
    public const SORT_BY_MODIFIED_TIME = 5;
    public const SORT_BY_NAME_NATURAL = 6;
    public const SORT_BY_NAME_CASE_INSENSITIVE = 7;
    public const SORT_BY_NAME_NATURAL_CASE_INSENSITIVE = 8;
    public const SORT_BY_EXTENSION = 9;
    public const SORT_BY_SIZE = 10;

    /** @var \Traversable<string, \SplFileInfo> */
    private \Traversable $iterator;
    private \Closure|int $sort;

    /**
     * @param \Traversable<string, \SplFileInfo> $iterator
     * @param int|callable                       $sort     The sort type (SORT_BY_NAME, SORT_BY_TYPE, or a PHP callback)
     *
     * @throws \InvalidArgumentException
     */
    public function __construct(\Traversable $iterator, int|callable $sort, bool $reverseOrder = false)
    {}

    public function getIterator(): \Traversable
    { yield; }
}
