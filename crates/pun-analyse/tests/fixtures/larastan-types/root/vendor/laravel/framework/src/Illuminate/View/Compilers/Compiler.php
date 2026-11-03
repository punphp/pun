<?php

namespace Illuminate\View\Compilers;

use ErrorException;
use Illuminate\Filesystem\Filesystem;
use Illuminate\Support\Str;
use InvalidArgumentException;

abstract class Compiler
{
    /**
     * The filesystem instance.
     *
     * @var \Illuminate\Filesystem\Filesystem
     */
    protected $files;

    /**
     * The cache path for the compiled views.
     *
     * @var string
     */
    protected $cachePath;

    /**
     * The base path that should be removed from paths before hashing.
     *
     * @var string
     */
    protected $basePath;

    /**
     * Determines if compiled views should be cached.
     *
     * @var bool
     */
    protected $shouldCache;

    /**
     * The compiled view file extension.
     *
     * @var string
     */
    protected $compiledExtension = 'php';

    /**
     * Indicates if view cache timestamps should be checked.
     *
     * @var bool
     */
    protected $shouldCheckTimestamps;

    /**
     * Create a new compiler instance.
     *
     * @param  \Illuminate\Filesystem\Filesystem  $files
     * @param  string  $cachePath
     * @param  string  $basePath
     * @param  bool  $shouldCache
     * @param  string  $compiledExtension
     * @param  bool  $shouldCheckTimestamps
     *
     * @throws \InvalidArgumentException
     */
    public function __construct(
        Filesystem $files,
        $cachePath,
        $basePath = '',
        $shouldCache = true,
        $compiledExtension = 'php',
        $shouldCheckTimestamps = true,
    ) {}

    /**
     * Get the path to the compiled version of a view.
     *
     * @param  string  $path
     * @return string
     */
    public function getCompiledPath($path)
    {}

    /**
     * Determine if the view at the given path is expired.
     *
     * @param  string  $path
     * @return bool
     *
     * @throws \ErrorException
     */
    public function isExpired($path)
    {}

    /**
     * Create the compiled file directory if necessary.
     *
     * @param  string  $path
     * @return void
     */
    protected function ensureCompiledDirectoryExists($path)
    {}
}
