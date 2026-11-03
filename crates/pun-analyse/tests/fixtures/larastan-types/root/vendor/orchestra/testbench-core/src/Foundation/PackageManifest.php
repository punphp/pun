<?php

namespace Orchestra\Testbench\Foundation;

use Illuminate\Filesystem\Filesystem;
use Illuminate\Foundation\PackageManifest as IlluminatePackageManifest;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Illuminate\Support\Facades\ParallelTesting;
use Orchestra\Testbench\Contracts\TestCase as TestCaseContract;

use function Orchestra\Sidekick\Filesystem\join_paths;
use function Orchestra\Sidekick\is_testbench_cli;
use function Orchestra\Testbench\in_parallel_testing;
use function Orchestra\Testbench\package_path;

/**
 * @api
 */
class PackageManifest extends IlluminatePackageManifest
{
    /**
     * Testbench Class.
     *
     * @var \Orchestra\Testbench\Contracts\TestCase|object|null
     */
    protected $testbench;

    /**
     * List of required packages.
     *
     * @var array<int, string>
     */
    protected array $requiredPackages = [
        'laravel/dusk',
        'spatie/laravel-ray',
    ];

    /**
     * {@inheritDoc}
     *
     * @param  \Orchestra\Testbench\Contracts\TestCase|object|null  $testbench
     */
    public function __construct(Filesystem $files, $basePath, $manifestPath, $testbench = null)
    {}

    /**
     * Create a new package manifest instance from base.
     *
     * @param  \Illuminate\Foundation\Application  $app
     * @param  object|null  $testbench
     * @return void
     */
    public static function swap($app, $testbench = null): void
    {}

    /**
     * Set Testbench instance.
     *
     * @param  object|null  $testbench
     * @return void
     */
    public function setTestbench($testbench): void
    {}

    /**
     * Requires packages.
     *
     * @param  string[]  $packages
     * @return $this
     */
    public function requires(...$packages)
    {}

    /**
     * Get the manifest path.
     *
     * @return string|null
     */
    public function getManifestPath()
    {}

    /** {@inheritDoc} */
    #[\Override]
    protected function getManifest()
    {}

    /** {@inheritDoc} */
    #[\Override]
    protected function packagesToIgnore()
    {}

    /**
     * Get all of the package names from root.
     *
     * @return array
     */
    protected function providersFromRoot(): array
    {}

    /**
     * Get testbench root composer file.
     *
     * @return array{name: string, extra?: array{laravel?: array}}|null
     */
    protected function providersFromTestbench(): ?array
    {}

    /** {@inheritDoc} */
    #[\Override]
    protected function write(array $manifest)
    {}
}
