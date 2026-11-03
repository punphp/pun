<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\HttpFoundation\Session\Attribute;

/**
 * This class relates to session attribute storage.
 *
 * @implements \IteratorAggregate<string, mixed>
 */
class AttributeBag implements AttributeBagInterface, \IteratorAggregate, \Countable
{
    protected array $attributes = [];

    private string $name = 'attributes';

    /**
     * @param string $storageKey The key used to store attributes in the session
     */
    public function __construct(
        private string $storageKey = '_sf2_attributes',
    ) {}

    public function getName(): string
    {}

    public function setName(string $name): void
    {}

    public function initialize(array &$attributes): void
    {}

    public function getStorageKey(): string
    {}

    public function has(string $name): bool
    {}

    public function get(string $name, mixed $default = null): mixed
    {}

    public function set(string $name, mixed $value): void
    {}

    public function all(): array
    {}

    public function replace(array $attributes): void
    {}

    public function remove(string $name): mixed
    {}

    public function clear(): mixed
    {}

    /**
     * Returns an iterator for attributes.
     *
     * @return \ArrayIterator<string, mixed>
     */
    public function getIterator(): \ArrayIterator
    {}

    /**
     * Returns the number of attributes.
     */
    public function count(): int
    {}
}
