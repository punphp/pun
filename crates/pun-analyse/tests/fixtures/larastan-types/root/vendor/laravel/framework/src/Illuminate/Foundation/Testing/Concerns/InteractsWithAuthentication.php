<?php

namespace Illuminate\Foundation\Testing\Concerns;

use Illuminate\Contracts\Auth\Authenticatable as UserContract;

trait InteractsWithAuthentication
{
    /**
     * Set the currently logged in user for the application.
     *
     * @param  \Illuminate\Contracts\Auth\Authenticatable  $user
     * @param  string|null  $guard
     * @return $this
     */
    public function actingAs(UserContract $user, $guard = null)
    {}

    /**
     * Clear the currently logged in user for the application.
     *
     * @param  string|null  $guard
     * @return $this
     */
    public function actingAsGuest($guard = null)
    {}

    /**
     * Set the currently logged in user for the application.
     *
     * @param  \Illuminate\Contracts\Auth\Authenticatable  $user
     * @param  string|null  $guard
     * @return $this
     */
    public function be(UserContract $user, $guard = null)
    {}

    /**
     * Assert that the user is authenticated.
     *
     * @param  string|null  $guard
     * @return $this
     */
    public function assertAuthenticated($guard = null)
    {}

    /**
     * Assert that the user is not authenticated.
     *
     * @param  string|null  $guard
     * @return $this
     */
    public function assertGuest($guard = null)
    {}

    /**
     * Return true if the user is authenticated, false otherwise.
     *
     * @param  string|null  $guard
     * @return bool
     */
    protected function isAuthenticated($guard = null)
    {}

    /**
     * Assert that the user is authenticated as the given user.
     *
     * @param  \Illuminate\Contracts\Auth\Authenticatable  $user
     * @param  string|null  $guard
     * @return $this
     */
    public function assertAuthenticatedAs($user, $guard = null)
    {}

    /**
     * Assert that the given credentials are valid.
     *
     * @param  array  $credentials
     * @param  string|null  $guard
     * @return $this
     */
    public function assertCredentials(array $credentials, $guard = null)
    {}

    /**
     * Assert that the given credentials are invalid.
     *
     * @param  array  $credentials
     * @param  string|null  $guard
     * @return $this
     */
    public function assertInvalidCredentials(array $credentials, $guard = null)
    {}

    /**
     * Return true if the credentials are valid, false otherwise.
     *
     * @param  array  $credentials
     * @param  string|null  $guard
     * @return bool
     */
    protected function hasCredentials(array $credentials, $guard = null)
    {}
}
