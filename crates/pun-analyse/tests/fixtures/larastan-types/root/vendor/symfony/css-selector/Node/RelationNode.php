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
 * Represents a "<selector>:has(<subselector>)" node.
 *
 * This component is a port of the Python cssselect library,
 * which is copyright Ian Bicking, @see https://github.com/scrapy/cssselect.
 *
 * @author Franck Ranaivo-Harisoa <franckranaivo@gmail.com>
 *
 * @internal
 */
class RelationNode extends AbstractNode
{
    /**
     * @param list<array{0: string, 1: NodeInterface}> $arguments
     */
    public function __construct(
        private NodeInterface $selector,
        private array $arguments,
    ) {}

    public function getSelector(): NodeInterface
    {}

    /**
     * @return list<array{0: string, 1: NodeInterface}>
     */
    public function getArguments(): array
    {}

    public function getSpecificity(): Specificity
    {}

    public function __toString(): string
    {}
}
