<?php

declare(strict_types=1);

/*
 * This file is part of the league/commonmark package.
 *
 * (c) Colin O'Dell <colinodell@gmail.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace League\CommonMark\Util;

/**
 * Array collection
 *
 * Provides a wrapper around a standard PHP array.
 *
 * @internal
 *
 * @phpstan-template T
 * @phpstan-implements \IteratorAggregate<int, T>
 * @phpstan-implements \ArrayAccess<int, T>
 */
final class ArrayCollection implements \IteratorAggregate, \Countable, \ArrayAccess
{
    /**
     * @var array<int, mixed>
     * @phpstan-var array<int, T>
     */
    private array $elements;

    /**
     * Constructor
     *
     * @param array<int|string, mixed> $elements
     *
     * @phpstan-param array<int, T> $elements
     */
    public function __construct(array $elements = [])
    {}

    /**
     * @return mixed|false
     *
     * @phpstan-return T|false
     */
    public function first()
    {}

    /**
     * @return mixed|false
     *
     * @phpstan-return T|false
     */
    public function last()
    {}

    /**
     * Retrieve an external iterator
     *
     * @return \ArrayIterator<int, mixed>
     *
     * @phpstan-return \ArrayIterator<int, T>
     */
    #[\ReturnTypeWillChange]
    public function getIterator(): \ArrayIterator
    {}

    /**
     * Count elements of an object
     *
     * @return int The count as an integer.
     */
    public function count(): int
    {}

    /**
     * Whether an offset exists
     *
     * {@inheritDoc}
     *
     * @phpstan-param int $offset
     */
    public function offsetExists($offset): bool
    {}

    /**
     * Offset to retrieve
     *
     * {@inheritDoc}
     *
     * @phpstan-param int $offset
     *
     * @return mixed|null
     *
     * @phpstan-return T|null
     */
    #[\ReturnTypeWillChange]
    public function offsetGet($offset)
    {}

    /**
     * Offset to set
     *
     * {@inheritDoc}
     *
     * @phpstan-param int|null $offset
     * @phpstan-param T        $value
     */
    #[\ReturnTypeWillChange]
    public function offsetSet($offset, $value): void
    {}

    /**
     * Offset to unset
     *
     * {@inheritDoc}
     *
     * @phpstan-param int $offset
     */
    #[\ReturnTypeWillChange]
    public function offsetUnset($offset): void
    {}

    /**
     * Returns a subset of the array
     *
     * @return array<int, mixed>
     *
     * @phpstan-return array<int, T>
     */
    public function slice(int $offset, ?int $length = null): array
    {}

    /**
     * @return array<int, mixed>
     *
     * @phpstan-return array<int, T>
     */
    public function toArray(): array
    {}
}
