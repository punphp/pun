<?php

namespace Illuminate\Support;

use Closure;
use Illuminate\Filesystem\Filesystem;
use RuntimeException;
use Symfony\Component\Console\Output\OutputInterface;
use Symfony\Component\Process\Process;

class Composer
{
    /**
     * The filesystem instance.
     *
     * @var \Illuminate\Filesystem\Filesystem
     */
    protected $files;

    /**
     * The working path to regenerate from.
     *
     * @var string|null
     */
    protected $workingPath;

    /**
     * Create a new Composer manager instance.
     *
     * @param  \Illuminate\Filesystem\Filesystem  $files
     * @param  string|null  $workingPath
     */
    public function __construct(Filesystem $files, $workingPath = null)
    {}

    /**
     * Determine if the given Composer package is installed.
     *
     * @param  string  $package
     * @return bool
     *
     * @throws \RuntimeException
     */
    public function hasPackage($package)
    {}

    /**
     * Install the given Composer packages into the application.
     *
     * @param  array<int, string>  $packages
     * @param  bool  $dev
     * @param  \Closure|\Symfony\Component\Console\Output\OutputInterface|null  $output
     * @param  string|null  $composerBinary
     * @return bool
     */
    public function requirePackages(array $packages, bool $dev = false, Closure|OutputInterface|null $output = null, $composerBinary = null)
    {}

    /**
     * Remove the given Composer packages from the application.
     *
     * @param  array<int, string>  $packages
     * @param  bool  $dev
     * @param  \Closure|\Symfony\Component\Console\Output\OutputInterface|null  $output
     * @param  string|null  $composerBinary
     * @return bool
     */
    public function removePackages(array $packages, bool $dev = false, Closure|OutputInterface|null $output = null, $composerBinary = null)
    {}

    /**
     * Modify the "composer.json" file contents using the given callback.
     *
     * @param  callable(array<string, mixed>):array<string, mixed>  $callback
     * @return void
     *
     * @throws \JsonException
     * @throws \RuntimeException
     */
    public function modify(callable $callback)
    {}

    /**
     * Regenerate the Composer autoloader files.
     *
     * @param  string|array<string>  $extra
     * @param  string|null  $composerBinary
     * @return int
     */
    public function dumpAutoloads($extra = '', $composerBinary = null)
    {}

    /**
     * Regenerate the optimized Composer autoloader files.
     *
     * @param  string|null  $composerBinary
     * @return int
     */
    public function dumpOptimized($composerBinary = null)
    {}

    /**
     * Get the Composer binary / command for the environment.
     *
     * @param  string|null  $composerBinary
     * @return array<string>
     */
    public function findComposer($composerBinary = null)
    {}

    /**
     * Get the path to the "composer.json" file.
     *
     * @return string
     *
     * @throws \RuntimeException
     */
    protected function findComposerFile()
    {}

    /**
     * Get the PHP binary.
     *
     * @return string
     */
    protected function phpBinary()
    {}

    /**
     * Get a new Symfony process instance.
     *
     * @param  array<string>  $command
     * @param  array<string, string>  $env
     * @return \Symfony\Component\Process\Process
     */
    protected function getProcess(array $command, array $env = [])
    {}

    /**
     * Set the working path used by the class.
     *
     * @param  string  $path
     * @return $this
     */
    public function setWorkingPath($path)
    {}

    /**
     * Get the version of Composer.
     *
     * @return string|null
     */
    public function getVersion()
    {}
}
