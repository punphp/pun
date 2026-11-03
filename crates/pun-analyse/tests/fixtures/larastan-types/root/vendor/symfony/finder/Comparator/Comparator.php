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
 * @author Fabien Potencier <fabien@symfony.com>
 */
class Comparator
{
    private string $operator;

    public function __construct(
        private string $target,
        string $operator = '==',
    ) {}

    /**
     * Gets the target value.
     */
    public function getTarget(): string
    {}

    /**
     * Gets the comparison operator.
     */
    public function getOperator(): string
    {}

    /**
     * Tests against the target.
     */
    public function test(mixed $test): bool
    {}
}
