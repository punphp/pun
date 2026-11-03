<?php

namespace Illuminate\Routing;

use Illuminate\Http\RedirectResponse;
use Illuminate\Session\Store as SessionStore;
use Illuminate\Support\Traits\Macroable;

class Redirector
{
    use Macroable;

    /**
     * The URL generator instance.
     *
     * @var \Illuminate\Routing\UrlGenerator
     */
    protected $generator;

    /**
     * The session store instance.
     *
     * @var \Illuminate\Session\Store
     */
    protected $session;

    /**
     * Create a new Redirector instance.
     *
     * @param  \Illuminate\Routing\UrlGenerator  $generator
     */
    public function __construct(UrlGenerator $generator)
    {}

    /**
     * Create a new redirect response to the previous location.
     *
     * @param  int  $status
     * @param  array  $headers
     * @param  mixed  $fallback
     * @return \Illuminate\Http\RedirectResponse
     */
    public function back($status = 302, $headers = [], $fallback = false)
    {}

    /**
     * Create a new redirect response to the current URI.
     *
     * @param  int  $status
     * @param  array  $headers
     * @return \Illuminate\Http\RedirectResponse
     */
    public function refresh($status = 302, $headers = [])
    {}

    /**
     * Create a new redirect response, while putting the current URL in the session.
     *
     * @param  string  $path
     * @param  int  $status
     * @param  array  $headers
     * @param  bool|null  $secure
     * @return \Illuminate\Http\RedirectResponse
     */
    public function guest($path, $status = 302, $headers = [], $secure = null)
    {}

    /**
     * Create a new redirect response to the previously intended location.
     *
     * @param  mixed  $default
     * @param  int  $status
     * @param  array  $headers
     * @param  bool|null  $secure
     * @return \Illuminate\Http\RedirectResponse
     */
    public function intended($default = '/', $status = 302, $headers = [], $secure = null)
    {}

    /**
     * Create a new redirect response to the given path.
     *
     * @param  string  $path
     * @param  int  $status
     * @param  array  $headers
     * @param  bool|null  $secure
     * @return \Illuminate\Http\RedirectResponse
     */
    public function to($path, $status = 302, $headers = [], $secure = null)
    {}

    /**
     * Create a new redirect response to an external URL (no validation).
     *
     * @param  string  $path
     * @param  int  $status
     * @param  array  $headers
     * @return \Illuminate\Http\RedirectResponse
     */
    public function away($path, $status = 302, $headers = [])
    {}

    /**
     * Create a new redirect response to the given HTTPS path.
     *
     * @param  string  $path
     * @param  int  $status
     * @param  array  $headers
     * @return \Illuminate\Http\RedirectResponse
     */
    public function secure($path, $status = 302, $headers = [])
    {}

    /**
     * Create a new redirect response to a named route.
     *
     * @param  \BackedEnum|string  $route
     * @param  mixed  $parameters
     * @param  int  $status
     * @param  array  $headers
     * @return \Illuminate\Http\RedirectResponse
     */
    public function route($route, $parameters = [], $status = 302, $headers = [])
    {}

    /**
     * Create a new redirect response to a signed named route.
     *
     * @param  \BackedEnum|string  $route
     * @param  mixed  $parameters
     * @param  \DateTimeInterface|\DateInterval|int|null  $expiration
     * @param  int  $status
     * @param  array  $headers
     * @return \Illuminate\Http\RedirectResponse
     */
    public function signedRoute($route, $parameters = [], $expiration = null, $status = 302, $headers = [])
    {}

    /**
     * Create a new redirect response to a signed named route.
     *
     * @param  \BackedEnum|string  $route
     * @param  \DateTimeInterface|\DateInterval|int|null  $expiration
     * @param  mixed  $parameters
     * @param  int  $status
     * @param  array  $headers
     * @return \Illuminate\Http\RedirectResponse
     */
    public function temporarySignedRoute($route, $expiration, $parameters = [], $status = 302, $headers = [])
    {}

    /**
     * Create a new redirect response to a controller action.
     *
     * @param  string|array  $action
     * @param  mixed  $parameters
     * @param  int  $status
     * @param  array  $headers
     * @return \Illuminate\Http\RedirectResponse
     */
    public function action($action, $parameters = [], $status = 302, $headers = [])
    {}

    /**
     * Create a new redirect response.
     *
     * @param  string  $path
     * @param  int  $status
     * @param  array  $headers
     * @return \Illuminate\Http\RedirectResponse
     */
    protected function createRedirect($path, $status, $headers)
    {}

    /**
     * Get the URL generator instance.
     *
     * @return \Illuminate\Routing\UrlGenerator
     */
    public function getUrlGenerator()
    {}

    /**
     * Set the active session store.
     *
     * @param  \Illuminate\Session\Store  $session
     * @return void
     */
    public function setSession(SessionStore $session)
    {}

    /**
     * Get the "intended" URL from the session.
     *
     * @return string|null
     */
    public function getIntendedUrl()
    {}

    /**
     * Set the "intended" URL in the session.
     *
     * @param  string  $url
     * @return $this
     */
    public function setIntendedUrl($url)
    {}
}
