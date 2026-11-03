<?php

namespace Illuminate\Redis;

use Illuminate\Contracts\Support\DeferrableProvider;
use Illuminate\Support\Arr;
use Illuminate\Support\ServiceProvider;

class RedisServiceProvider extends ServiceProvider implements DeferrableProvider
{
    /**
     * Register the service provider.
     *
     * @return void
     */
    public function register()
    {}

    /**
     * Get the services provided by the provider.
     *
     * @return array
     */
    public function provides()
    {}
}
