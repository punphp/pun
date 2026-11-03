<?php

namespace Illuminate\Translation;

use Illuminate\Contracts\Translation\Loader;
use Illuminate\Filesystem\Filesystem;
use Illuminate\Support\Collection;
use RuntimeException;

class FileLoader implements Loader
{
    /**
     * The filesystem instance.
     *
     * @var \Illuminate\Filesystem\Filesystem
     */
    protected $files;

    /**
     * The default paths for the loader.
     *
     * @var array
     */
    protected $paths;

    /**
     * All of the registered paths to JSON translation files.
     *
     * @var array
     */
    protected $jsonPaths = [];

    /**
     * All of the namespace hints.
     *
     * @var array<string, string>
     */
    protected $hints = [];

    /**
     * Create a new file loader instance.
     *
     * @param  \Illuminate\Filesystem\Filesystem  $files
     * @param  array|string  $path
     */
    public function __construct(Filesystem $files, array|string $path)
    {}

    /**
     * Load the messages for the given locale.
     *
     * @param  string  $locale
     * @param  string  $group
     * @param  string|null  $namespace
     * @return array
     */
    public function load($locale, $group, $namespace = null)
    {}

    /**
     * Load a namespaced translation group.
     *
     * @param  string  $locale
     * @param  string  $group
     * @param  string  $namespace
     * @return array
     */
    protected function loadNamespaced($locale, $group, $namespace)
    {}

    /**
     * Load a local namespaced translation group for overrides.
     *
     * @param  array  $lines
     * @param  string  $locale
     * @param  string  $group
     * @param  string  $namespace
     * @return array
     */
    protected function loadNamespaceOverrides(array $lines, $locale, $group, $namespace)
    {}

    /**
     * Load a locale from a given path.
     *
     * @param  array  $paths
     * @param  string  $locale
     * @param  string  $group
     * @return array
     */
    protected function loadPaths(array $paths, $locale, $group)
    {}

    /**
     * Load a locale from the given JSON file path.
     *
     * @param  string  $locale
     * @return array
     *
     * @throws \RuntimeException
     */
    protected function loadJsonPaths($locale)
    {}

    /**
     * Determine if the given value is unsafe to use as part of a translation file path.
     *
     * @param  mixed  $value
     * @param  bool  $allowSlashes
     * @return bool
     */
    protected function isUnsafePathSegment($value, $allowSlashes = false)
    {}

    /**
     * Add a new namespace to the loader.
     *
     * @param  string  $namespace
     * @param  string  $hint
     * @return void
     */
    public function addNamespace($namespace, $hint)
    {}

    /**
     * Get an array of all the registered namespaces.
     *
     * @return array<string, string>
     */
    public function namespaces()
    {}

    /**
     * Add a new path to the loader.
     *
     * @param  string  $path
     * @return void
     */
    public function addPath($path)
    {}

    /**
     * Add a new JSON path to the loader.
     *
     * @param  string  $path
     * @return void
     */
    public function addJsonPath($path)
    {}

    /**
     * Get an array of all the registered paths to translation files.
     *
     * @return array
     */
    public function paths()
    {}

    /**
     * Get an array of all the registered paths to JSON translation files.
     *
     * @return array
     */
    public function jsonPaths()
    {}
}
