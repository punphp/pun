<?php

declare(strict_types=1);

namespace Dotenv\Store;

use Dotenv\Store\File\Paths;

final class StoreBuilder
{
    /**
     * The of default name.
     */
    private const DEFAULT_NAME = '.env';

    /**
     * The paths to search within.
     *
     * @var string[]
     */
    private $paths;

    /**
     * The file names to search for.
     *
     * @var string[]
     */
    private $names;

    /**
     * Should file loading short circuit?
     *
     * @var bool
     */
    private $shortCircuit;

    /**
     * The file encoding.
     *
     * @var string|null
     */
    private $fileEncoding;

    /**
     * Create a new store builder instance.
     *
     * @param string[]    $paths
     * @param string[]    $names
     * @param bool        $shortCircuit
     * @param string|null $fileEncoding
     *
     * @return void
     */
    private function __construct(array $paths = [], array $names = [], bool $shortCircuit = false, ?string $fileEncoding = null)
    {}

    /**
     * Create a new store builder instance with no names.
     *
     * @return \Dotenv\Store\StoreBuilder
     */
    public static function createWithNoNames()
    {}

    /**
     * Create a new store builder instance with the default name.
     *
     * @return \Dotenv\Store\StoreBuilder
     */
    public static function createWithDefaultName()
    {}

    /**
     * Creates a store builder with the given path added.
     *
     * @param string $path
     *
     * @return \Dotenv\Store\StoreBuilder
     */
    public function addPath(string $path)
    {}

    /**
     * Creates a store builder with the given name added.
     *
     * @param string $name
     *
     * @return \Dotenv\Store\StoreBuilder
     */
    public function addName(string $name)
    {}

    /**
     * Creates a store builder with short circuit mode enabled.
     *
     * @return \Dotenv\Store\StoreBuilder
     */
    public function shortCircuit()
    {}

    /**
     * Creates a store builder with the specified file encoding.
     *
     * @param string|null $fileEncoding
     *
     * @return \Dotenv\Store\StoreBuilder
     */
    public function fileEncoding(?string $fileEncoding = null)
    {}

    /**
     * Creates a new store instance.
     *
     * @return \Dotenv\Store\StoreInterface
     */
    public function make()
    {}
}
