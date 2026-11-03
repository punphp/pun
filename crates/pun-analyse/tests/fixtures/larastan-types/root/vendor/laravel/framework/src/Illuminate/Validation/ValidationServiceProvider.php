<?php

namespace Illuminate\Validation;

use Illuminate\Contracts\Support\DeferrableProvider;
use Illuminate\Contracts\Validation\UncompromisedVerifier;
use Illuminate\Http\Client\Factory as HttpFactory;
use Illuminate\Support\ServiceProvider;

class ValidationServiceProvider extends ServiceProvider implements DeferrableProvider
{
    /**
     * Register the service provider.
     *
     * @return void
     */
    public function register()
    {}

    /**
     * Register the validation factory.
     *
     * @return void
     */
    protected function registerValidationFactory()
    {}

    /**
     * Register the database presence verifier.
     *
     * @return void
     */
    protected function registerPresenceVerifier()
    {}

    /**
     * Register the uncompromised password verifier.
     *
     * @return void
     */
    protected function registerUncompromisedVerifier()
    {}

    /**
     * Get the services provided by the provider.
     *
     * @return array
     */
    public function provides()
    {}
}
