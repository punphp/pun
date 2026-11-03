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
 * Represents a "<selector>:<identifier>" node.
 *
 * This component is a port of the Python cssselect library,
 * which is copyright Ian Bicking, @see https://github.com/SimonSapin/cssselect.
 *
 * @author Jean-François Simon <jeanfrancois.simon@sensiolabs.com>
 *
 * @internal
 */
class PseudoNode extends AbstractNode
{
    private string $identifier;

    public function __construct(
        private NodeInterface $selector,
        string $identifier,
    ) {}

    public function getSelector(): NodeInterface
    {}

    public function getIdentifier(): string
    {}

    public function getSpecificity(): Specificity
    {}

    public function __toString(): string
    {}
}
