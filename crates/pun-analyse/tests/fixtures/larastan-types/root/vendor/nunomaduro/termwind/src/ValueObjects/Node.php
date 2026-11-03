<?php

declare(strict_types=1);

namespace Termwind\ValueObjects;

use Generator;

/**
 * @internal
 */
final class Node
{
    /**
     * A value object with helper methods for working with DOM node.
     */
    public function __construct(private \DOMNode $node) {}

    /**
     * Gets the value of the node.
     */
    public function getValue(): string
    {}

    /**
     * Gets child nodes of the node.
     *
     * @return Generator<Node>
     */
    public function getChildNodes(): Generator
    { yield; }

    /**
     * Checks if the node is a text.
     */
    public function isText(): bool
    {}

    /**
     * Checks if the node is a comment.
     */
    public function isComment(): bool
    {}

    /**
     * Compares the current node name with a given name.
     */
    public function isName(string $name): bool
    {}

    /**
     * Returns the current node type name.
     */
    public function getName(): string
    {}

    /**
     * Returns value of [class] attribute.
     */
    public function getClassAttribute(): string
    {}

    /**
     * Returns value of attribute with a given name.
     */
    public function getAttribute(string $name): string
    {}

    /**
     * Checks if the node is empty.
     */
    public function isEmpty(): bool
    {}

    /**
     * Gets the previous sibling from the node.
     */
    public function getPreviousSibling(): ?static
    {}

    /**
     * Gets the next sibling from the node.
     */
    public function getNextSibling(): ?static
    {}

    /**
     * Checks if the node is the first child.
     */
    public function isFirstChild(): bool
    {}

    /**
     * Gets the inner HTML representation of the node including child nodes.
     */
    public function getHtml(): string
    {}

    /**
     * Converts the node to a string.
     */
    public function __toString(): string
    {}
}
