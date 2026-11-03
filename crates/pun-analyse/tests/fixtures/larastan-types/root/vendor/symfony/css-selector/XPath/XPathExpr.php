<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\CssSelector\XPath;

/**
 * XPath expression translator interface.
 *
 * This component is a port of the Python cssselect library,
 * which is copyright Ian Bicking, @see https://github.com/SimonSapin/cssselect.
 *
 * @author Jean-François Simon <jeanfrancois.simon@sensiolabs.com>
 *
 * @internal
 */
class XPathExpr
{
    public function __construct(
        private string $path = '',
        private string $element = '*',
        private string $condition = '',
        bool $starPrefix = false,
    ) {}

    public function getElement(): string
    {}

    /**
     * @return $this
     */
    public function addCondition(string $condition, string $operator = 'and'): static
    {}

    public function getCondition(): string
    {}

    /**
     * @return $this
     */
    public function addNameTest(): static
    {}

    /**
     * @return $this
     */
    public function addStarPrefix(): static
    {}

    /**
     * Joins another XPathExpr with a combiner.
     *
     * @return $this
     */
    public function join(string $combiner, self $expr): static
    {}

    /**
     * Replaces position() and last() by tests on the siblings of the context node.
     */
    public static function positionsToSiblings(string $condition, string $element): string
    {}

    public function __toString(): string
    {}
}
