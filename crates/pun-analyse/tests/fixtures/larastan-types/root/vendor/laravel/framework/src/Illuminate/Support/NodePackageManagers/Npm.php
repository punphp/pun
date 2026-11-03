<?php

namespace Illuminate\Support\NodePackageManagers;

use Illuminate\Support\Contracts\NodePackageManager;

class Npm implements NodePackageManager
{
    /**
     * Determine if the npm package manager is in use.
     *
     * @return bool
     */
    public static function matches(): bool
    {}

    /**
     * Get the command to run a script using npm.
     *
     * @param  string  $command
     * @return string
     */
    public function getRunCommand(string $command): string
    {}

    /**
     * Get the command to execute a package using npm.
     *
     * @param  string  $command
     * @return string
     */
    public function getExecCommand(string $command): string
    {}
}
