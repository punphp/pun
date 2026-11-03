<?php

namespace Illuminate\Foundation\Console;

use Symfony\Component\Process\Process;

use function Illuminate\Support\php_binary;

trait InteractsWithComposerPackages
{
    /**
     * Installs the given Composer Packages into the application.
     *
     * @param  string  $composer
     * @param  array  $packages
     * @param  bool  $pretend
     * @return bool
     */
    protected function requireComposerPackages(string $composer, array $packages, bool $pretend = false)
    {}

    /**
     * Get the path to the appropriate PHP binary.
     *
     * @return string
     */
    protected function phpBinary()
    {}
}
