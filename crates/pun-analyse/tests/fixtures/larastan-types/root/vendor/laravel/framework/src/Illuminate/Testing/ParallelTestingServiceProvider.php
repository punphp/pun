<?php

namespace Illuminate\Testing;

use Illuminate\Contracts\Support\DeferrableProvider;
use Illuminate\Support\ServiceProvider;
use Illuminate\Testing\Concerns\TestCaches;
use Illuminate\Testing\Concerns\TestDatabases;
use Illuminate\Testing\Concerns\TestViews;

class ParallelTestingServiceProvider extends ServiceProvider implements DeferrableProvider
{
    use TestCaches, TestDatabases, TestViews;

    /**
     * Boot the application's service providers.
     *
     * @return void
     */
    public function boot()
    {}

    /**
     * Register the service provider.
     *
     * @return void
     */
    public function register()
    {}
}
