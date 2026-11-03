<?php

namespace Illuminate\Foundation\Testing;

use Illuminate\Foundation\Application;
use Illuminate\Foundation\Bootstrap\LoadConfiguration;

trait WithCachedConfig
{
    /**
     * After resolving the configuration once, we can cache it for the remaining tests.
     */
    protected function setUpWithCachedConfig(): void
    {}

    /**
     * Reset the cached configuration.
     *
     * This is helpful if some of the tests in the suite apply this trait while others do not.
     */
    protected function tearDownWithCachedConfig(): void
    {}

    /**
     * Inform the container that the configuration is cached.
     */
    protected function markConfigCached(Application $app): void
    {}
}
