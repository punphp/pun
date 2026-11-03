<?php

namespace Illuminate\Support\NodePackageManagers;

use Illuminate\Support\Contracts\NodePackageManager;

class Yarn implements NodePackageManager
{
    /**
     * Determine if the Yarn package manager is in use.
     *
     * @return bool
     */
    public static function matches(): bool
    {}

    /**
     * Get the command to run a script using Yarn.
     *
     * @param  string  $command
     * @return string
     */
    public function getRunCommand(string $command): string
    {}

    /**
     * Get the command to execute a package using Yarn.
     *
     * @param  string  $command
     * @return string
     */
    public function getExecCommand(string $command): string
    {}
}
