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

use Symfony\Component\CssSelector\Parser\Token;

/**
 * Represents a "<selector>:<name>(<arguments>)" node.
 *
 * This component is a port of the Python cssselect library,
 * which is copyright Ian Bicking, @see https://github.com/SimonSapin/cssselect.
 *
 * @author Jean-François Simon <jeanfrancois.simon@sensiolabs.com>
 *
 * @internal
 */
class FunctionNode extends AbstractNode
{
    private string $name;

    /**
     * @param Token[] $arguments
     */
    public function __construct(
        private NodeInterface $selector,
        string $name,
        private array $arguments = [],
    ) {}

    public function getSelector(): NodeInterface
    {}

    public function getName(): string
    {}

    /**
     * @return Token[]
     */
    public function getArguments(): array
    {}

    public function getSpecificity(): Specificity
    {}

    public function __toString(): string
    {}
}
