<?php

namespace Illuminate\Support;

use Illuminate\Support\Contracts\NodePackageManager as NodePackageManagerContract;

class NodePackageManager
{
    /**
     * Create a new NodePackageManager manager instance.
     *
     * @param  NodePackageManagerContract|null  $packageManager
     */
    public function __construct(protected ?NodePackageManagerContract $packageManager = null)
    {}

    /**
     * Get the command to execute a package using the detected package manager.
     *
     * @param  string  $command
     * @return string
     */
    public function getExecCommand(string $command): string
    {}

    /**
     * Get the command to run a script using the detected package manager.
     *
     * @param  string  $command
     * @return string
     */
    public function getRunCommand(string $command): string
    {}

    /**
     * Get the Node package manager in use.
     *
     * @return NodePackageManagerContract
     */
    public function packageManager(): NodePackageManagerContract
    {}

    /**
     * Detect the current package manager.
     *
     * @return NodePackageManagerContract
     */
    protected function detect(): NodePackageManagerContract
    {}
}
