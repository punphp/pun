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
 * Represents a "<selector>[<namespace>|<attribute> <operator> <value>]" node.
 *
 * This component is a port of the Python cssselect library,
 * which is copyright Ian Bicking, @see https://github.com/SimonSapin/cssselect.
 *
 * @author Jean-François Simon <jeanfrancois.simon@sensiolabs.com>
 *
 * @internal
 */
class AttributeNode extends AbstractNode
{
    public function __construct(
        private NodeInterface $selector,
        private ?string $namespace,
        private string $attribute,
        private string $operator,
        private ?string $value,
    ) {}

    public function getSelector(): NodeInterface
    {}

    public function getNamespace(): ?string
    {}

    public function getAttribute(): string
    {}

    public function getOperator(): string
    {}

    public function getValue(): ?string
    {}

    public function getSpecificity(): Specificity
    {}

    public function __toString(): string
    {}
}
