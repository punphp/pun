<?php

namespace Illuminate\Foundation;

use Exception;
use Illuminate\Filesystem\Filesystem;
use Illuminate\Support\Collection;
use Illuminate\Support\Env;

class PackageManifest
{
    /**
     * The filesystem instance.
     *
     * @var \Illuminate\Filesystem\Filesystem
     */
    public $files;

    /**
     * The base path.
     *
     * @var string
     */
    public $basePath;

    /**
     * The vendor path.
     *
     * @var string
     */
    public $vendorPath;

    /**
     * The manifest path.
     *
     * @var string|null
     */
    public $manifestPath;

    /**
     * The loaded manifest array.
     *
     * @var array|null
     */
    public $manifest;

    /**
     * Create a new package manifest instance.
     *
     * @param  \Illuminate\Filesystem\Filesystem  $files
     * @param  string  $basePath
     * @param  string  $manifestPath
     */
    public function __construct(Filesystem $files, $basePath, $manifestPath)
    {}

    /**
     * Get all of the service provider class names for all packages.
     *
     * @return array
     */
    public function providers()
    {}

    /**
     * Get all of the aliases for all packages.
     *
     * @return array
     */
    public function aliases()
    {}

    /**
     * Get all of the values for all packages for the given configuration name.
     *
     * @param  string  $key
     * @return array
     */
    public function config($key)
    {}

    /**
     * Get the current package manifest.
     *
     * @return array
     */
    protected function getManifest()
    {}

    /**
     * Build the manifest and write it to disk.
     *
     * @return void
     */
    public function build()
    {}

    /**
     * Format the given package name.
     *
     * @param  string  $package
     * @return string
     */
    protected function format($package)
    {}

    /**
     * Get all of the package names that should be ignored.
     *
     * @return array
     */
    protected function packagesToIgnore()
    {}

    /**
     * Write the given manifest array to disk.
     *
     * @param  array  $manifest
     * @return void
     *
     * @throws \Exception
     */
    protected function write(array $manifest)
    {}
}
