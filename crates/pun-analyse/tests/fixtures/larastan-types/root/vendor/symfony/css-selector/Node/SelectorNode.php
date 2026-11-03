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
 * Represents a "<selector>(::|:)<pseudoElement>" node.
 *
 * This component is a port of the Python cssselect library,
 * which is copyright Ian Bicking, @see https://github.com/SimonSapin/cssselect.
 *
 * @author Jean-François Simon <jeanfrancois.simon@sensiolabs.com>
 *
 * @internal
 */
class SelectorNode extends AbstractNode
{
    private ?string $pseudoElement;

    public function __construct(
        private NodeInterface $tree,
        ?string $pseudoElement = null,
    ) {}

    public function getTree(): NodeInterface
    {}

    public function getPseudoElement(): ?string
    {}

    public function getSpecificity(): Specificity
    {}

    public function __toString(): string
    {}
}
