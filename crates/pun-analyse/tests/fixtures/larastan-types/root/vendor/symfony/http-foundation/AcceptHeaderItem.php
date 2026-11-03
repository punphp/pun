<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\HttpFoundation;

/**
 * Represents an Accept-* header item.
 *
 * @author Jean-François Simon <contact@jfsimon.fr>
 */
class AcceptHeaderItem
{
    private float $quality = 1.0;
    private int $index = 0;
    private array $attributes = [];

    public function __construct(
        private string $value,
        array $attributes = [],
    ) {}

    /**
     * Builds an AcceptHeaderInstance instance from a string.
     */
    public static function fromString(?string $itemValue): self
    {}

    /**
     * Returns header value's string representation.
     */
    public function __toString(): string
    {}

    /**
     * Set the item value.
     *
     * @return $this
     */
    public function setValue(string $value): static
    {}

    /**
     * Returns the item value.
     */
    public function getValue(): string
    {}

    /**
     * Set the item quality.
     *
     * @return $this
     */
    public function setQuality(float $quality): static
    {}

    /**
     * Returns the item quality.
     */
    public function getQuality(): float
    {}

    /**
     * Set the item index.
     *
     * @return $this
     */
    public function setIndex(int $index): static
    {}

    /**
     * Returns the item index.
     */
    public function getIndex(): int
    {}

    /**
     * Tests if an attribute exists.
     */
    public function hasAttribute(string $name): bool
    {}

    /**
     * Returns an attribute by its name.
     */
    public function getAttribute(string $name, mixed $default = null): mixed
    {}

    /**
     * Returns all attributes.
     */
    public function getAttributes(): array
    {}

    /**
     * Set an attribute.
     *
     * @return $this
     */
    public function setAttribute(string $name, string $value): static
    {}
}
