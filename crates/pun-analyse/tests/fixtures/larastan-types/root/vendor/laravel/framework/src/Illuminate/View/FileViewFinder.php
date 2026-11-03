<?php

namespace Illuminate\View;

use Illuminate\Filesystem\Filesystem;
use InvalidArgumentException;

class FileViewFinder implements ViewFinderInterface
{
    /**
     * The filesystem instance.
     *
     * @var \Illuminate\Filesystem\Filesystem
     */
    protected $files;

    /**
     * The array of active view paths.
     *
     * @var string[]
     */
    protected $paths;

    /**
     * The array of views that have been located.
     *
     * @var array<string, string>
     */
    protected $views = [];

    /**
     * The namespace to file path hints.
     *
     * @var array<string, array>
     */
    protected $hints = [];

    /**
     * Register a view extension with the finder.
     *
     * @var string[]
     */
    protected $extensions = ['blade.php', 'php', 'css', 'html'];

    /**
     * Create a new file view loader instance.
     *
     * @param  \Illuminate\Filesystem\Filesystem  $files
     * @param  string[]  $paths
     * @param  string[]|null  $extensions
     */
    public function __construct(Filesystem $files, array $paths, ?array $extensions = null)
    {}

    /**
     * Get the fully-qualified location of the view.
     *
     * @param  string  $view
     * @return string
     */
    public function find($view)
    {}

    /**
     * Get the path to a template with a named path.
     *
     * @param  string  $name
     * @return string
     */
    protected function findNamespacedView($name)
    {}

    /**
     * Get the segments of a template with a named path.
     *
     * @param  string  $name
     * @return string[]
     *
     * @throws \InvalidArgumentException
     */
    protected function parseNamespaceSegments($name)
    {}

    /**
     * Find the given view in the list of paths.
     *
     * @param  string  $name
     * @param  string|string[]  $paths
     * @return string
     *
     * @throws \InvalidArgumentException
     */
    protected function findInPaths($name, $paths)
    {}

    /**
     * Get an array of possible view files.
     *
     * @param  string  $name
     * @return string[]
     */
    protected function getPossibleViewFiles($name)
    {}

    /**
     * Add a location to the finder.
     *
     * @param  string  $location
     * @return void
     */
    public function addLocation($location)
    {}

    /**
     * Prepend a location to the finder.
     *
     * @param  string  $location
     * @return void
     */
    public function prependLocation($location)
    {}

    /**
     * Resolve the path.
     *
     * @param  string  $path
     * @return string
     */
    protected function resolvePath($path)
    {}

    /**
     * Add a namespace hint to the finder.
     *
     * @param  string  $namespace
     * @param  string|string[]  $hints
     * @return void
     */
    public function addNamespace($namespace, $hints)
    {}

    /**
     * Prepend a namespace hint to the finder.
     *
     * @param  string  $namespace
     * @param  string|string[]  $hints
     * @return void
     */
    public function prependNamespace($namespace, $hints)
    {}

    /**
     * Replace the namespace hints for the given namespace.
     *
     * @param  string  $namespace
     * @param  string|string[]  $hints
     * @return void
     */
    public function replaceNamespace($namespace, $hints)
    {}

    /**
     * Register an extension with the view finder.
     *
     * @param  string  $extension
     * @return void
     */
    public function addExtension($extension)
    {}

    /**
     * Returns whether or not the view name has any hint information.
     *
     * @param  string  $name
     * @return bool
     */
    public function hasHintInformation($name)
    {}

    /**
     * Flush the cache of located views.
     *
     * @return void
     */
    public function flush()
    {}

    /**
     * Get the filesystem instance.
     *
     * @return \Illuminate\Filesystem\Filesystem
     */
    public function getFilesystem()
    {}

    /**
     * Set the active view paths.
     *
     * @param  string[]  $paths
     * @return $this
     */
    public function setPaths($paths)
    {}

    /**
     * Get the active view paths.
     *
     * @return string[]
     */
    public function getPaths()
    {}

    /**
     * Get the views that have been located.
     *
     * @return array<string, string>
     */
    public function getViews()
    {}

    /**
     * Get the namespace to file path hints.
     *
     * @return array<string, array>
     */
    public function getHints()
    {}

    /**
     * Get registered extensions.
     *
     * @return string[]
     */
    public function getExtensions()
    {}
}
