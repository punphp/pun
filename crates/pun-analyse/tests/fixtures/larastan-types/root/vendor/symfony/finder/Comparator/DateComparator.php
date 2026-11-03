<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Finder\Comparator;

/**
 * DateCompare compiles date comparisons.
 *
 * @author Fabien Potencier <fabien@symfony.com>
 */
class DateComparator extends Comparator
{
    /**
     * @param string $test A comparison string
     *
     * @throws \InvalidArgumentException If the test is not understood
     */
    public function __construct(string $test)
    {}
}
