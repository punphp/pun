<?php

namespace Illuminate\Foundation\Configuration;

use Closure;
use Illuminate\Auth\AuthenticationException;
use Illuminate\Auth\Middleware\Authenticate;
use Illuminate\Auth\Middleware\RedirectIfAuthenticated;
use Illuminate\Cookie\Middleware\EncryptCookies;
use Illuminate\Foundation\Http\Middleware\ConvertEmptyStringsToNull;
use Illuminate\Foundation\Http\Middleware\PreventRequestForgery;
use Illuminate\Foundation\Http\Middleware\PreventRequestsDuringMaintenance;
use Illuminate\Foundation\Http\Middleware\TrimStrings;
use Illuminate\Http\Middleware\TrustHosts;
use Illuminate\Http\Middleware\TrustProxies;
use Illuminate\Routing\Middleware\ValidateSignature;
use Illuminate\Session\Middleware\AuthenticateSession;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;

class Middleware
{
    /**
     * The user defined global middleware stack.
     *
     * @var array
     */
    protected $global = [];

    /**
     * The middleware that should be prepended to the global middleware stack.
     *
     * @var array
     */
    protected $prepends = [];

    /**
     * The middleware that should be appended to the global middleware stack.
     *
     * @var array
     */
    protected $appends = [];

    /**
     * The middleware that should be removed from the global middleware stack.
     *
     * @var array
     */
    protected $removals = [];

    /**
     * The middleware that should be replaced in the global middleware stack.
     *
     * @var array
     */
    protected $replacements = [];

    /**
     * The user defined middleware groups.
     *
     * @var array
     */
    protected $groups = [];

    /**
     * The middleware that should be prepended to the specified groups.
     *
     * @var array
     */
    protected $groupPrepends = [];

    /**
     * The middleware that should be appended to the specified groups.
     *
     * @var array
     */
    protected $groupAppends = [];

    /**
     * The middleware that should be removed from the specified groups.
     *
     * @var array
     */
    protected $groupRemovals = [];

    /**
     * The middleware that should be replaced in the specified groups.
     *
     * @var array
     */
    protected $groupReplacements = [];

    /**
     * The Folio / page middleware for the application.
     *
     * @var array
     */
    protected $pageMiddleware = [];

    /**
     * Indicates if the "trust hosts" middleware is enabled.
     *
     * @var bool
     */
    protected $trustHosts = false;

    /**
     * Indicates if Sanctum's frontend state middleware is enabled.
     *
     * @var bool
     */
    protected $statefulApi = false;

    /**
     * Indicates the API middleware group's rate limiter.
     *
     * @var string
     */
    protected $apiLimiter;

    /**
     * Indicates if Redis throttling should be applied.
     *
     * @var bool
     */
    protected $throttleWithRedis = false;

    /**
     * Indicates if sessions should be authenticated for the "web" middleware group.
     *
     * @var bool
     */
    protected $authenticatedSessions = false;

    /**
     * The custom middleware aliases.
     *
     * @var array
     */
    protected $customAliases = [];

    /**
     * The custom middleware priority definition.
     *
     * @var array
     */
    protected $priority = [];

    /**
     * The middleware to prepend to the middleware priority definition.
     *
     * @var array
     */
    protected $prependPriority = [];

    /**
     * The middleware to append to the middleware priority definition.
     *
     * @var array
     */
    protected $appendPriority = [];

    /**
     * Prepend middleware to the application's global middleware stack.
     *
     * @param  array|string  $middleware
     * @return $this
     */
    public function prepend(array|string $middleware)
    {}

    /**
     * Append middleware to the application's global middleware stack.
     *
     * @param  array|string  $middleware
     * @return $this
     */
    public function append(array|string $middleware)
    {}

    /**
     * Remove middleware from the application's global middleware stack.
     *
     * @param  array|string  $middleware
     * @return $this
     */
    public function remove(array|string $middleware)
    {}

    /**
     * Specify a middleware that should be replaced with another middleware.
     *
     * @param  string  $search
     * @param  string  $replace
     * @return $this
     */
    public function replace(string $search, string $replace)
    {}

    /**
     * Define the global middleware for the application.
     *
     * @param  array  $middleware
     * @return $this
     */
    public function use(array $middleware)
    {}

    /**
     * Define a middleware group.
     *
     * @param  string  $group
     * @param  array  $middleware
     * @return $this
     */
    public function group(string $group, array $middleware)
    {}

    /**
     * Prepend the given middleware to the specified group.
     *
     * @param  string  $group
     * @param  array|string  $middleware
     * @return $this
     */
    public function prependToGroup(string $group, array|string $middleware)
    {}

    /**
     * Append the given middleware to the specified group.
     *
     * @param  string  $group
     * @param  array|string  $middleware
     * @return $this
     */
    public function appendToGroup(string $group, array|string $middleware)
    {}

    /**
     * Remove the given middleware from the specified group.
     *
     * @param  string  $group
     * @param  array|string  $middleware
     * @return $this
     */
    public function removeFromGroup(string $group, array|string $middleware)
    {}

    /**
     * Replace the given middleware in the specified group with another middleware.
     *
     * @param  string  $group
     * @param  string  $search
     * @param  string  $replace
     * @return $this
     */
    public function replaceInGroup(string $group, string $search, string $replace)
    {}

    /**
     * Modify the middleware in the "web" group.
     *
     * @param  array|string  $append
     * @param  array|string  $prepend
     * @param  array|string  $remove
     * @param  array  $replace
     * @return $this
     */
    public function web(array|string $append = [], array|string $prepend = [], array|string $remove = [], array $replace = [])
    {}

    /**
     * Modify the middleware in the "api" group.
     *
     * @param  array|string  $append
     * @param  array|string  $prepend
     * @param  array|string  $remove
     * @param  array  $replace
     * @return $this
     */
    public function api(array|string $append = [], array|string $prepend = [], array|string $remove = [], array $replace = [])
    {}

    /**
     * Modify the middleware in the given group.
     *
     * @param  string  $group
     * @param  array|string  $append
     * @param  array|string  $prepend
     * @param  array|string  $remove
     * @param  array  $replace
     * @return $this
     */
    protected function modifyGroup(string $group, array|string $append, array|string $prepend, array|string $remove, array $replace)
    {}

    /**
     * Register the Folio / page middleware for the application.
     *
     * @param  array  $middleware
     * @return $this
     */
    public function pages(array $middleware)
    {}

    /**
     * Register additional middleware aliases.
     *
     * @param  array  $aliases
     * @return $this
     */
    public function alias(array $aliases)
    {}

    /**
     * Define the middleware priority for the application.
     *
     * @param  array  $priority
     * @return $this
     */
    public function priority(array $priority)
    {}

    /**
     * Prepend middleware to the priority middleware.
     *
     * @param  array|string  $before
     * @param  string  $prepend
     * @return $this
     */
    public function prependToPriorityList($before, $prepend)
    {}

    /**
     * Append middleware to the priority middleware.
     *
     * @param  array|string  $after
     * @param  string  $append
     * @return $this
     */
    public function appendToPriorityList($after, $append)
    {}

    /**
     * Get the global middleware.
     *
     * @return array
     */
    public function getGlobalMiddleware()
    {}

    /**
     * Get the middleware groups.
     *
     * @return array
     */
    public function getMiddlewareGroups()
    {}

    /**
     * Configure where guests are redirected by the "auth" middleware.
     *
     * @param  callable|string|null  $redirect
     * @return $this
     */
    public function redirectGuestsTo(callable|string|null $redirect)
    {}

    /**
     * Configure where users are redirected by the "guest" middleware.
     *
     * @param  callable|string  $redirect
     * @return $this
     */
    public function redirectUsersTo(callable|string $redirect)
    {}

    /**
     * Configure where users are redirected by the authentication and guest middleware.
     *
     * @param  callable|string|null  $guests
     * @param  callable|string|null  $users
     * @return $this
     */
    public function redirectTo(callable|string|null $guests = null, callable|string|null $users = null)
    {}

    /**
     * Configure the cookie encryption middleware.
     *
     * @param  array<int, string>  $except
     * @return $this
     */
    public function encryptCookies(array $except = [])
    {}

    /**
     * Configure the request forgery prevention middleware.
     *
     * @param  array  $except
     * @param  bool  $originOnly
     * @param  bool  $allowSameSite
     * @return $this
     */
    public function preventRequestForgery(array $except = [], bool $originOnly = false, bool $allowSameSite = false)
    {}

    /**
     * Configure the CSRF token validation middleware.
     *
     * @deprecated Use preventRequestForgery() instead.
     *
     * @param  array  $except
     * @return $this
     */
    public function validateCsrfTokens(array $except = [])
    {}

    /**
     * Configure the URL signature validation middleware.
     *
     * @param  array  $except
     * @return $this
     */
    public function validateSignatures(array $except = [])
    {}

    /**
     * Configure the empty string conversion middleware.
     *
     * @param  array<int, (\Closure(\Illuminate\Http\Request): bool)>  $except
     * @return $this
     */
    public function convertEmptyStringsToNull(array $except = [])
    {}

    /**
     * Configure the string trimming middleware.
     *
     * @param  array<int, (\Closure(\Illuminate\Http\Request): bool)|string>  $except
     * @return $this
     */
    public function trimStrings(array $except = [])
    {}

    /**
     * Indicate that the trusted host middleware should be enabled.
     *
     * @param  array<int, string>|(callable(): array<int, string>)|null  $at
     * @param  bool  $subdomains
     * @return $this
     */
    public function trustHosts(array|callable|null $at = null, bool $subdomains = true)
    {}

    /**
     * Configure the trusted proxies for the application.
     *
     * @param  array<int, string>|string|null  $at
     * @param  int|null  $headers
     * @return $this
     */
    public function trustProxies(array|string|null $at = null, ?int $headers = null)
    {}

    /**
     * Configure the middleware that prevents requests during maintenance mode.
     *
     * @param  array<int, string>  $except
     * @return $this
     */
    public function preventRequestsDuringMaintenance(array $except = [])
    {}

    /**
     * Indicate that Sanctum's frontend state middleware should be enabled.
     *
     * @return $this
     */
    public function statefulApi()
    {}

    /**
     * Indicate that the API middleware group's throttling middleware should be enabled.
     *
     * @param  string  $limiter
     * @param  bool  $redis
     * @return $this
     */
    public function throttleApi($limiter = 'api', $redis = false)
    {}

    /**
     * Indicate that Laravel's throttling middleware should use Redis.
     *
     * @return $this
     */
    public function throttleWithRedis()
    {}

    /**
     * Indicate that sessions should be authenticated for the "web" middleware group.
     *
     * @return $this
     */
    public function authenticateSessions()
    {}

    /**
     * Get the Folio / page middleware for the application.
     *
     * @return array
     */
    public function getPageMiddleware()
    {}

    /**
     * Get the middleware aliases.
     *
     * @return array
     */
    public function getMiddlewareAliases()
    {}

    /**
     * Get the default middleware aliases.
     *
     * @return array
     */
    protected function defaultAliases()
    {}

    /**
     * Get the middleware priority for the application.
     *
     * @return array
     */
    public function getMiddlewarePriority()
    {}

    /**
     * Get the middleware to prepend to the middleware priority definition.
     *
     * @return array
     */
    public function getMiddlewarePriorityPrepends()
    {}

    /**
     * Get the middleware to append to the middleware priority definition.
     *
     * @return array
     */
    public function getMiddlewarePriorityAppends()
    {}
}
