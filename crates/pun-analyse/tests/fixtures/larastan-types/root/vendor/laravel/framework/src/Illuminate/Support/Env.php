<?php

namespace Illuminate\Support;

use Closure;
use Dotenv\Repository\Adapter\PutenvAdapter;
use Dotenv\Repository\RepositoryBuilder;
use Illuminate\Filesystem\Filesystem;
use PhpOption\Option;
use RuntimeException;

class Env
{
    /**
     * Indicates if the putenv adapter is enabled.
     *
     * @var bool
     */
    protected static $putenv = true;

    /**
     * The environment repository instance.
     *
     * @var \Dotenv\Repository\RepositoryInterface|null
     */
    protected static $repository;

    /**
     * The list of custom adapters for loading environment variables.
     *
     * @var array<Closure>
     */
    protected static $customAdapters = [];

    /**
     * Enable the putenv adapter.
     *
     * @return void
     */
    public static function enablePutenv()
    {}

    /**
     * Disable the putenv adapter.
     *
     * @return void
     */
    public static function disablePutenv()
    {}

    /**
     * Register a custom adapter creator Closure.
     */
    public static function extend(Closure $callback, ?string $name = null): void
    {}

    /**
     * Get the environment repository instance.
     *
     * @return \Dotenv\Repository\RepositoryInterface
     */
    public static function getRepository()
    {}

    /**
     * Get the value of an environment variable.
     *
     * @param  string  $key
     * @param  mixed  $default
     * @return mixed
     */
    public static function get($key, $default = null)
    {}

    /**
     * Get the value of a required environment variable.
     *
     * @param  string  $key
     * @return mixed
     *
     * @throws \RuntimeException
     */
    public static function getOrFail($key)
    {}

    /**
     * Write an array of key-value pairs to the environment file.
     *
     * @param  array<string, mixed>  $variables
     * @param  string  $pathToFile
     * @param  bool  $overwrite
     * @return void
     *
     * @throws \RuntimeException
     * @throws \Illuminate\Contracts\Filesystem\FileNotFoundException
     */
    public static function writeVariables(array $variables, string $pathToFile, bool $overwrite = false): void
    {}

    /**
     * Write a single key-value pair to the environment file.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @param  string  $pathToFile
     * @param  bool  $overwrite
     * @return void
     *
     * @throws \RuntimeException
     * @throws \Illuminate\Contracts\Filesystem\FileNotFoundException
     */
    public static function writeVariable(string $key, mixed $value, string $pathToFile, bool $overwrite = false): void
    {}

    /**
     * Add a variable to the environment file contents.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @param  array<int, string>  $envLines
     * @param  bool  $overwrite
     * @return array<int, string>
     */
    protected static function addVariableToEnvContents(string $key, mixed $value, array $envLines, bool $overwrite): array
    {}

    /**
     * Get the possible option for this environment variable.
     *
     * @param  string  $key
     * @return \PhpOption\Option|\PhpOption\Some
     */
    protected static function getOption($key)
    {}

    /**
     * Wrap a string in quotes, choosing double or single quotes.
     *
     * @param  string  $input
     * @return string
     */
    protected static function prepareQuotedValue(string $input)
    {}

    /**
     * Escape a string using addslashes, excluding the specified characters from being escaped.
     *
     * @param  string  $value
     * @param  array<string>  $except
     * @return string
     */
    protected static function addSlashesExceptFor(string $value, array $except = [])
    {}
}
