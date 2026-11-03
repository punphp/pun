<?php

namespace Orchestra\Testbench\Concerns;

use Orchestra\Testbench\Foundation\Application as Testbench;

use function Orchestra\Sidekick\Filesystem\join_paths;
use function Orchestra\Testbench\uses_default_skeleton;
use function Orchestra\Testbench\workbench_path;

trait WithLaravelBootstrapFile
{
    use InteractsWithTestCase;

    /**
     * Get application bootstrap file path (if exists).
     *
     * @internal
     *
     * @param  string  $filename
     * @return string|false
     */
    protected function getApplicationBootstrapFile(string $filename): string|false
    {}

    /**
     * Determine if application is using a custom application kernels.
     *
     * @internal
     *
     * @return bool
     */
    protected function hasCustomApplicationKernels(): bool
    {}

    /**
     * Determine if application is bootstrapped using Testbench's default skeleton.
     *
     * @return bool
     */
    protected function usesTestbenchDefaultSkeleton(): bool
    {}

    /**
     * Get the application's base path.
     *
     * @api
     *
     * @return string
     */
    abstract protected function getApplicationBasePath();
}
