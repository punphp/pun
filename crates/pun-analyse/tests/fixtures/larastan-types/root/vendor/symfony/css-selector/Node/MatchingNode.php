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
 * Represents a "<selector>:is(<subSelectorList>)" node.
 *
 * This component is a port of the Python cssselect library,
 * which is copyright Ian Bicking, @see https://github.com/SimonSapin/cssselect.
 *
 * @author Hubert Lenoir <lenoir.hubert@gmail.com>
 *
 * @internal
 */
class MatchingNode extends AbstractNode
{
    /**
     * @param array<NodeInterface> $arguments
     */
    public function __construct(
        public readonly NodeInterface $selector,
        public readonly array $arguments = [],
    ) {}

    public function getSpecificity(): Specificity
    {}

    public function __toString(): string
    {}
}
