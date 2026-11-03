<?php

declare(strict_types=1);

namespace League\Flysystem;

use ArrayIterator;
use Generator;
use IteratorAggregate;
use Traversable;

/**
 * @template T
 */
class DirectoryListing implements IteratorAggregate
{
    /**
     * @param iterable<T> $listing
     */
    public function __construct(private iterable $listing)
    {}

    /**
     * @param callable(T): bool $filter
     *
     * @return DirectoryListing<T>
     */
    public function filter(callable $filter): DirectoryListing
    {}

    /**
     * @template R
     *
     * @param callable(T): R $mapper
     *
     * @return DirectoryListing<R>
     */
    public function map(callable $mapper): DirectoryListing
    {}

    /**
     * @return DirectoryListing<T>
     */
    public function sortByPath(): DirectoryListing
    {}

    /**
     * @return Traversable<T>
     */
    public function getIterator(): Traversable
    {}

    /**
     * @return T[]
     */
    public function toArray(): array
    {}
}
