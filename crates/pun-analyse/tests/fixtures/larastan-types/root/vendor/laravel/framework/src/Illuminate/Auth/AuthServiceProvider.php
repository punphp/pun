<?php

namespace Illuminate\Auth;

use Illuminate\Auth\Access\Gate;
use Illuminate\Auth\Middleware\RequirePassword;
use Illuminate\Contracts\Auth\Access\Gate as GateContract;
use Illuminate\Contracts\Auth\Authenticatable as AuthenticatableContract;
use Illuminate\Contracts\Routing\ResponseFactory;
use Illuminate\Contracts\Routing\UrlGenerator;
use Illuminate\Support\ServiceProvider;

class AuthServiceProvider extends ServiceProvider
{
    /**
     * Register the service provider.
     *
     * @return void
     */
    public function register()
    {}

    /**
     * Register the authenticator services.
     *
     * @return void
     */
    protected function registerAuthenticator()
    {}

    /**
     * Register a resolver for the authenticated user.
     *
     * @return void
     */
    protected function registerUserResolver()
    {}

    /**
     * Register the access gate service.
     *
     * @return void
     */
    protected function registerAccessGate()
    {}

    /**
     * Register a resolver for the authenticated user.
     *
     * @return void
     */
    protected function registerRequirePassword()
    {}

    /**
     * Handle the re-binding of the request binding.
     *
     * @return void
     */
    protected function registerRequestRebindHandler()
    {}

    /**
     * Handle the re-binding of the event dispatcher binding.
     *
     * @return void
     */
    protected function registerEventRebindHandler()
    {}
}
