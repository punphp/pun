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

use Symfony\Component\Finder\Comparator\NumberComparator;

/**
 * SizeRangeFilterIterator filters out files that are not in the given size range.
 *
 * @author Fabien Potencier <fabien@symfony.com>
 *
 * @extends \FilterIterator<string, \SplFileInfo>
 */
class SizeRangeFilterIterator extends \FilterIterator
{
    private array $comparators = [];

    /**
     * @param \Iterator<string, \SplFileInfo> $iterator
     * @param NumberComparator[]              $comparators
     */
    public function __construct(\Iterator $iterator, array $comparators)
    {}

    /**
     * Filters the iterator values.
     */
    public function accept(): bool
    {}
}
