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

// Help opcache.preload discover always-needed symbols
class_exists(AcceptHeaderItem::class);

/**
 * Represents an Accept-* header.
 *
 * An accept header is compound with a list of items,
 * sorted by descending quality.
 *
 * @author Jean-François Simon <contact@jfsimon.fr>
 */
class AcceptHeader
{
    /**
     * @var array<string, AcceptHeaderItem>
     */
    private array $items = [];

    private bool $sorted = true;

    /**
     * @param AcceptHeaderItem[] $items
     */
    public function __construct(array $items)
    {}

    /**
     * Builds an AcceptHeader instance from a string.
     */
    public static function fromString(?string $headerValue): self
    {}

    /**
     * Returns header value's string representation.
     */
    public function __toString(): string
    {}

    /**
     * Tests if header has given value.
     */
    public function has(string $value): bool
    {}

    /**
     * Returns given value's item, if exists.
     */
    public function get(string $value): ?AcceptHeaderItem
    {}

    /**
     * Adds an item.
     *
     * @return $this
     */
    public function add(AcceptHeaderItem $item): static
    {}

    /**
     * Returns all items.
     *
     * @return AcceptHeaderItem[]
     */
    public function all(): array
    {}

    /**
     * Filters items on their value using given regex.
     */
    public function filter(string $pattern): self
    {}

    /**
     * Returns first item.
     */
    public function first(): ?AcceptHeaderItem
    {}

    /**
     * Sorts items by descending quality.
     */
    private function sort(): void
    {}

    /**
     * Generates the canonical key for storing/retrieving an item.
     */
    private function getCanonicalKey(AcceptHeaderItem $item): string
    {}

    /**
     * Checks if a given header item (range) matches a queried item (value).
     *
     * @param AcceptHeaderItem $rangeItem The item from the Accept header (e.g., text/*;format=flowed)
     * @param AcceptHeaderItem $queryItem The item being queried (e.g., text/plain;format=flowed;charset=utf-8)
     */
    private function matches(AcceptHeaderItem $rangeItem, AcceptHeaderItem $queryItem): bool
    {}

    /**
     * Checks if the parameters of a range item are satisfied by the query item.
     *
     * Parameters are case-insensitive; range params must be a subset of query params.
     */
    private function rangeParametersMatch(AcceptHeaderItem $rangeItem, AcceptHeaderItem $queryItem): bool
    {}

    /**
     * Calculates a specificity score for sorting: media precision + param count.
     */
    private function getSpecificity(AcceptHeaderItem $item, AcceptHeaderItem $queryItem): int
    {}

    /**
     * Returns normalized attributes: keys lowercased, excluding 'q'.
     */
    private function getMediaParams(AcceptHeaderItem $item): array
    {}
}
