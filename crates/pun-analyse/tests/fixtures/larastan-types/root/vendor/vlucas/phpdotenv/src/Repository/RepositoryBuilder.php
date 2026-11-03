<?php

declare(strict_types=1);

namespace Dotenv\Repository;

use Dotenv\Repository\Adapter\AdapterInterface;
use Dotenv\Repository\Adapter\EnvConstAdapter;
use Dotenv\Repository\Adapter\GuardedWriter;
use Dotenv\Repository\Adapter\ImmutableWriter;
use Dotenv\Repository\Adapter\MultiReader;
use Dotenv\Repository\Adapter\MultiWriter;
use Dotenv\Repository\Adapter\ReaderInterface;
use Dotenv\Repository\Adapter\ServerConstAdapter;
use Dotenv\Repository\Adapter\WriterInterface;
use InvalidArgumentException;
use PhpOption\Some;
use ReflectionClass;

final class RepositoryBuilder
{
    /**
     * The set of default adapters.
     */
    private const DEFAULT_ADAPTERS = [
        ServerConstAdapter::class,
        EnvConstAdapter::class,
    ];

    /**
     * The set of readers to use.
     *
     * @var \Dotenv\Repository\Adapter\ReaderInterface[]
     */
    private $readers;

    /**
     * The set of writers to use.
     *
     * @var \Dotenv\Repository\Adapter\WriterInterface[]
     */
    private $writers;

    /**
     * Are we immutable?
     *
     * @var bool
     */
    private $immutable;

    /**
     * The variable name allow list.
     *
     * @var string[]|null
     */
    private $allowList;

    /**
     * Create a new repository builder instance.
     *
     * @param \Dotenv\Repository\Adapter\ReaderInterface[] $readers
     * @param \Dotenv\Repository\Adapter\WriterInterface[] $writers
     * @param bool                                         $immutable
     * @param string[]|null                                $allowList
     *
     * @return void
     */
    private function __construct(array $readers = [], array $writers = [], bool $immutable = false, ?array $allowList = null)
    {}

    /**
     * Create a new repository builder instance with no adapters added.
     *
     * @return \Dotenv\Repository\RepositoryBuilder
     */
    public static function createWithNoAdapters()
    {}

    /**
     * Create a new repository builder instance with the default adapters added.
     *
     * @return \Dotenv\Repository\RepositoryBuilder
     */
    public static function createWithDefaultAdapters()
    {}

    /**
     * Return the array of default adapters.
     *
     * @return \Generator<\Dotenv\Repository\Adapter\AdapterInterface>
     */
    private static function defaultAdapters()
    { yield; }

    /**
     * Determine if the given name if of an adapterclass.
     *
     * @param string $name
     *
     * @return bool
     */
    private static function isAnAdapterClass(string $name)
    {}

    /**
     * Creates a repository builder with the given reader added.
     *
     * Accepts either a reader instance, or a class-string for an adapter. If
     * the adapter is not supported, then we silently skip adding it.
     *
     * @param \Dotenv\Repository\Adapter\ReaderInterface|class-string<\Dotenv\Repository\Adapter\AdapterInterface> $reader
     *
     * @throws \InvalidArgumentException
     *
     * @return \Dotenv\Repository\RepositoryBuilder
     */
    public function addReader($reader)
    {}

    /**
     * Creates a repository builder with the given writer added.
     *
     * Accepts either a writer instance, or a class-string for an adapter. If
     * the adapter is not supported, then we silently skip adding it.
     *
     * @param \Dotenv\Repository\Adapter\WriterInterface|class-string<\Dotenv\Repository\Adapter\AdapterInterface> $writer
     *
     * @throws \InvalidArgumentException
     *
     * @return \Dotenv\Repository\RepositoryBuilder
     */
    public function addWriter($writer)
    {}

    /**
     * Creates a repository builder with the given adapter added.
     *
     * Accepts either an adapter instance, or a class-string for an adapter. If
     * the adapter is not supported, then we silently skip adding it. We will
     * add the adapter as both a reader and a writer.
     *
     * @param \Dotenv\Repository\Adapter\AdapterInterface|class-string<\Dotenv\Repository\Adapter\AdapterInterface> $adapter
     *
     * @throws \InvalidArgumentException
     *
     * @return \Dotenv\Repository\RepositoryBuilder
     */
    public function addAdapter($adapter)
    {}

    /**
     * Creates a repository builder with immutability enabled.
     *
     * @return \Dotenv\Repository\RepositoryBuilder
     */
    public function immutable()
    {}

    /**
     * Creates a repository builder with the given allow list.
     *
     * @param string[]|null $allowList
     *
     * @return \Dotenv\Repository\RepositoryBuilder
     */
    public function allowList(?array $allowList = null)
    {}

    /**
     * Creates a new repository instance.
     *
     * @return \Dotenv\Repository\RepositoryInterface
     */
    public function make()
    {}
}
