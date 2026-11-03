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
 * DepthRangeFilterIterator limits the directory depth.
 *
 * @author Fabien Potencier <fabien@symfony.com>
 *
 * @template-covariant TKey
 * @template-covariant TValue
 *
 * @extends \FilterIterator<TKey, TValue>
 */
class DepthRangeFilterIterator extends \FilterIterator
{
    private int $minDepth = 0;

    /**
     * @param \RecursiveIteratorIterator<\RecursiveIterator<TKey, TValue>> $iterator The Iterator to filter
     * @param int                                                          $minDepth The min depth
     * @param int                                                          $maxDepth The max depth
     */
    public function __construct(\RecursiveIteratorIterator $iterator, int $minDepth = 0, int $maxDepth = \PHP_INT_MAX)
    {}

    /**
     * Filters the iterator values.
     */
    public function accept(): bool
    {}
}
