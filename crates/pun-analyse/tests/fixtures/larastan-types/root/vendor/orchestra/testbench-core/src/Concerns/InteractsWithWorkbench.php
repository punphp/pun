<?php

namespace Orchestra\Testbench\Concerns;

use Illuminate\Support\Arr;
use Orchestra\Testbench\Workbench\Workbench;

/**
 * @internal
 */
trait InteractsWithWorkbench
{
    use InteractsWithPest;
    use InteractsWithPHPUnit;
    use InteractsWithTestCase;

    /**
     * Get Application's base path.
     *
     * @internal
     *
     * @return string|null
     */
    public static function applicationBasePathUsingWorkbench(): ?string
    {}

    /**
     * Ignore package discovery from.
     *
     * @internal
     *
     * @return array<int, string>|null
     */
    public function ignorePackageDiscoveriesFromUsingWorkbench(): ?array
    {}

    /**
     * Get package bootstrapper.
     *
     * @internal
     *
     * @param  \Illuminate\Foundation\Application  $app
     * @return array<int, class-string>|null
     */
    protected function getPackageBootstrappersUsingWorkbench($app): ?array
    {}

    /**
     * Get package providers.
     *
     * @internal
     *
     * @param  \Illuminate\Foundation\Application  $app
     * @return array<int, class-string<\Illuminate\Support\ServiceProvider>>|null
     */
    protected function getPackageProvidersUsingWorkbench($app): ?array
    {}

    /**
     * Resolve application Console Kernel implementation.
     *
     * @internal
     *
     * @param  \Illuminate\Foundation\Application  $app
     * @return string
     */
    protected function applicationConsoleKernelUsingWorkbench($app): string
    {}

    /**
     * Get application HTTP Kernel implementation using Workbench.
     *
     * @internal
     *
     * @param  \Illuminate\Foundation\Application  $app
     * @return string
     */
    protected function applicationHttpKernelUsingWorkbench($app): string
    {}

    /**
     * Get application HTTP exception handler using Workbench.
     *
     * @internal
     *
     * @param  \Illuminate\Foundation\Application  $app
     * @return string
     */
    protected function applicationExceptionHandlerUsingWorkbench($app): string
    {}

    /**
     * Define or get the cached uses for test case.
     *
     * @return \Orchestra\Testbench\Contracts\Config|null
     */
    public static function cachedConfigurationForWorkbench()
    {}

    /**
     * Prepare the testing environment before the running the test case.
     *
     * @internal
     *
     * @return void
     *
     * @codeCoverageIgnore
     */
    public static function setUpBeforeClassUsingWorkbench(): void
    {}

    /**
     * Clean up the testing environment before the next test case.
     *
     * @internal
     *
     * @return void
     *
     * @codeCoverageIgnore
     */
    public static function tearDownAfterClassUsingWorkbench(): void
    {}
}
