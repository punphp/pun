<?php

namespace Illuminate\Support;

class AggregateServiceProvider extends ServiceProvider
{
    /**
     * The provider class names.
     *
     * @var array<int, class-string<\Illuminate\Support\ServiceProvider>>
     */
    protected $providers = [];

    /**
     * An array of the service provider instances.
     *
     * @var array<int, \Illuminate\Support\ServiceProvider>
     */
    protected $instances = [];

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
     * @return array<int, string>
     */
    public function provides()
    {}
}
