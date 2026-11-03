<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\CssSelector\Node;

/**
 * Represents a node specificity.
 *
 * This component is a port of the Python cssselect library,
 * which is copyright Ian Bicking, @see https://github.com/SimonSapin/cssselect.
 *
 * @see http://www.w3.org/TR/selectors/#specificity
 *
 * @author Jean-François Simon <jeanfrancois.simon@sensiolabs.com>
 *
 * @internal
 */
class Specificity
{
    public const A_FACTOR = 100;
    public const B_FACTOR = 10;
    public const C_FACTOR = 1;

    public function __construct(
        private int $a,
        private int $b,
        private int $c,
    ) {}

    public function plus(self $specificity): self
    {}

    public function getValue(): int
    {}

    /**
     * Returns -1 if the object specificity is lower than the argument,
     * 0 if they are equal, and 1 if the argument is lower.
     */
    public function compareTo(self $specificity): int
    {}
}
