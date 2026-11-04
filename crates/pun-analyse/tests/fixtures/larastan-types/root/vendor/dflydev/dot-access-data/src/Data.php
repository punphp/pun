<?php

declare(strict_types=1);

/*
 * This file is a part of dflydev/dot-access-data.
 *
 * (c) Dragonfly Development Inc.
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Dflydev\DotAccessData;

use ArrayAccess;
use Dflydev\DotAccessData\Exception\DataException;
use Dflydev\DotAccessData\Exception\InvalidPathException;
use Dflydev\DotAccessData\Exception\MissingPathException;

/**
 * @implements ArrayAccess<string, mixed>
 */
class Data implements DataInterface, ArrayAccess
{
    private const DELIMITERS = ['.', '/'];

    /**
     * Internal representation of data data
     *
     * @var array<string, mixed>
     */
    protected $data;

    /**
     * Constructor
     *
     * @param array<string, mixed> $data
     */
    public function __construct(array $data = [])
    {}

    /**
     * {@inheritdoc}
     */
    public function append(string $key, $value = null): void
    {}

    /**
     * {@inheritdoc}
     */
    public function set(string $key, $value = null): void
    {}

    /**
     * {@inheritdoc}
     */
    public function remove(string $key): void
    {}

    /**
     * {@inheritdoc}
     *
     * @psalm-mutation-free
     */
    public function get(string $key, $default = null)
    { func_get_args(); }

    /**
     * {@inheritdoc}
     *
     * @psalm-mutation-free
     */
    public function has(string $key): bool
    {}

    /**
     * {@inheritdoc}
     *
     * @psalm-mutation-free
     */
    public function getData(string $key): DataInterface
    {}

    /**
     * {@inheritdoc}
     */
    public function import(array $data, int $mode = self::REPLACE): void
    {}

    /**
     * {@inheritdoc}
     */
    public function importData(DataInterface $data, int $mode = self::REPLACE): void
    {}

    /**
     * {@inheritdoc}
     *
     * @psalm-mutation-free
     */
    public function export(): array
    {}

    /**
     * {@inheritdoc}
     *
     * @return bool
     */
    #[\ReturnTypeWillChange]
    public function offsetExists($key)
    {}

    /**
     * {@inheritdoc}
     *
     * @return mixed
     */
    #[\ReturnTypeWillChange]
    public function offsetGet($key)
    {}

    /**
     * {@inheritdoc}
     *
     * @param string $key
     * @param mixed $value
     *
     * @return void
     */
    #[\ReturnTypeWillChange]
    public function offsetSet($key, $value)
    {}

    /**
     * {@inheritdoc}
     *
     * @return void
     */
    #[\ReturnTypeWillChange]
    public function offsetUnset($key)
    {}

    /**
     * @param string $path
     *
     * @return string[]
     *
     * @psalm-return non-empty-list<string>
     *
     * @psalm-pure
     */
    protected static function keyToPathArray(string $path): array
    {}

    /**
     * @param string|string[] $path
     *
     * @return string
     *
     * @psalm-pure
     */
    protected static function formatPath($path): string
    {}
}
