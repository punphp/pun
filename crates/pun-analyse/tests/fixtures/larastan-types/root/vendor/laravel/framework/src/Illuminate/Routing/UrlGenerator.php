<?php

namespace Illuminate\Routing;

use BackedEnum;
use Closure;
use Illuminate\Contracts\Routing\UrlGenerator as UrlGeneratorContract;
use Illuminate\Contracts\Routing\UrlRoutable;
use Illuminate\Http\Request;
use Illuminate\Support\Arr;
use Illuminate\Support\Carbon;
use Illuminate\Support\InteractsWithTime;
use Illuminate\Support\Str;
use Illuminate\Support\Stringable;
use Illuminate\Support\Traits\Macroable;
use InvalidArgumentException;
use Symfony\Component\Routing\Exception\RouteNotFoundException;

class UrlGenerator implements UrlGeneratorContract
{
    use InteractsWithTime, Macroable;

    /**
     * The route collection.
     *
     * @var \Illuminate\Routing\RouteCollectionInterface
     */
    protected $routes;

    /**
     * The request instance.
     *
     * @var \Illuminate\Http\Request
     */
    protected $request;

    /**
     * The asset root URL.
     *
     * @var string|null
     */
    protected $assetRoot;

    /**
     * The forced URL root.
     *
     * @var string|null
     */
    protected $forcedRoot;

    /**
     * The forced scheme for URLs.
     *
     * @var string|null
     */
    protected $forceScheme;

    /**
     * A cached copy of the URL root for the current request.
     *
     * @var string|null
     */
    protected $cachedRoot;

    /**
     * A cached copy of the URL scheme for the current request.
     *
     * @var string|null
     */
    protected $cachedScheme;

    /**
     * The root namespace being applied to controller actions.
     *
     * @var string
     */
    protected $rootNamespace;

    /**
     * The session resolver callable.
     *
     * @var callable
     */
    protected $sessionResolver;

    /**
     * The encryption key resolver callable.
     *
     * @var callable
     */
    protected $keyResolver;

    /**
     * The missing named route resolver callable.
     *
     * @var callable
     */
    protected $missingNamedRouteResolver;

    /**
     * The callback to use to format hosts.
     *
     * @var \Closure
     */
    protected $formatHostUsing;

    /**
     * The callback to use to format paths.
     *
     * @var \Closure
     */
    protected $formatPathUsing;

    /**
     * The route URL generator instance.
     *
     * @var \Illuminate\Routing\RouteUrlGenerator|null
     */
    protected $routeGenerator;

    /**
     * Create a new URL Generator instance.
     *
     * @param  \Illuminate\Routing\RouteCollectionInterface  $routes
     * @param  \Illuminate\Http\Request  $request
     * @param  string|null  $assetRoot
     */
    public function __construct(RouteCollectionInterface $routes, Request $request, $assetRoot = null)
    {}

    /**
     * Get the full URL for the current request.
     *
     * @return string
     */
    public function full()
    {}

    /**
     * Get the current URL for the request.
     *
     * @return string
     */
    public function current()
    {}

    /**
     * Get the URL for the previous request.
     *
     * @param  mixed  $fallback
     * @return string
     */
    public function previous($fallback = false)
    {}

    /**
     * Get the previous path info for the request.
     *
     * @param  mixed  $fallback
     * @return string
     */
    public function previousPath($fallback = false)
    {}

    /**
     * Get the previous URL from the session if possible.
     *
     * @return string|null
     */
    protected function getPreviousUrlFromSession()
    {}

    /**
     * Generate an absolute URL to the given path.
     *
     * @param  string  $path
     * @param  mixed  $extra
     * @param  bool|null  $secure
     * @return string
     */
    public function to($path, $extra = [], $secure = null)
    {}

    /**
     * Generate an absolute URL with the given query parameters.
     *
     * @param  string  $path
     * @param  array  $query
     * @param  mixed  $extra
     * @param  bool|null  $secure
     * @return string
     */
    public function query($path, $query = [], $extra = [], $secure = null)
    {}

    /**
     * Generate a secure, absolute URL to the given path.
     *
     * @param  string  $path
     * @param  array  $parameters
     * @return string
     */
    public function secure($path, $parameters = [])
    {}

    /**
     * Generate the URL to an application asset.
     *
     * @param  string  $path
     * @param  bool|null  $secure
     * @return string
     */
    public function asset($path, $secure = null)
    {}

    /**
     * Generate the URL to a secure asset.
     *
     * @param  string  $path
     * @return string
     */
    public function secureAsset($path)
    {}

    /**
     * Generate the URL to an asset from a custom root domain such as CDN, etc.
     *
     * @param  string  $root
     * @param  string  $path
     * @param  bool|null  $secure
     * @return string
     */
    public function assetFrom($root, $path, $secure = null)
    {}

    /**
     * Remove the index.php file from a path.
     *
     * @param  string  $root
     * @return string
     */
    protected function removeIndex($root)
    {}

    /**
     * Get the default scheme for a raw URL.
     *
     * @param  bool|null  $secure
     * @return string
     */
    public function formatScheme($secure = null)
    {}

    /**
     * Create a signed route URL for a named route.
     *
     * @param  \BackedEnum|string  $name
     * @param  mixed  $parameters
     * @param  \DateTimeInterface|\DateInterval|int|null  $expiration
     * @param  bool  $absolute
     * @return string
     *
     * @throws \InvalidArgumentException
     */
    public function signedRoute($name, $parameters = [], $expiration = null, $absolute = true)
    {}

    /**
     * Ensure the given signed route parameters are not reserved.
     *
     * @param  mixed  $parameters
     * @return void
     *
     * @throws \InvalidArgumentException
     */
    protected function ensureSignedRouteParametersAreNotReserved($parameters)
    {}

    /**
     * Create a temporary signed route URL for a named route.
     *
     * @param  \BackedEnum|string  $name
     * @param  \DateTimeInterface|\DateInterval|int  $expiration
     * @param  array  $parameters
     * @param  bool  $absolute
     * @return string
     */
    public function temporarySignedRoute($name, $expiration, $parameters = [], $absolute = true)
    {}

    /**
     * Determine if the given request has a valid signature.
     *
     * @param  \Illuminate\Http\Request  $request
     * @param  bool  $absolute
     * @param  \Closure|array  $ignoreQuery
     * @return bool
     */
    public function hasValidSignature(Request $request, $absolute = true, Closure|array $ignoreQuery = [])
    {}

    /**
     * Determine if the given request has a valid signature for a relative URL.
     *
     * @param  \Illuminate\Http\Request  $request
     * @param  \Closure|array  $ignoreQuery
     * @return bool
     */
    public function hasValidRelativeSignature(Request $request, Closure|array $ignoreQuery = [])
    {}

    /**
     * Determine if the signature from the given request matches the URL.
     *
     * @param  \Illuminate\Http\Request  $request
     * @param  bool  $absolute
     * @param  \Closure|array  $ignoreQuery
     * @return bool
     */
    public function hasCorrectSignature(Request $request, $absolute = true, Closure|array $ignoreQuery = [])
    {}

    /**
     * Determine if the expires timestamp from the given request is not from the past.
     *
     * @param  \Illuminate\Http\Request  $request
     * @return bool
     */
    public function signatureHasNotExpired(Request $request)
    {}

    /**
     * Get the URL to a named route.
     *
     * @param  \BackedEnum|string  $name
     * @param  mixed  $parameters
     * @param  bool  $absolute
     * @return string
     *
     * @throws \Symfony\Component\Routing\Exception\RouteNotFoundException|\InvalidArgumentException
     */
    public function route($name, $parameters = [], $absolute = true)
    {}

    /**
     * Get the URL for a given route instance.
     *
     * @param  \Illuminate\Routing\Route  $route
     * @param  mixed  $parameters
     * @param  bool  $absolute
     * @return string
     *
     * @throws \Illuminate\Routing\Exceptions\UrlGenerationException
     */
    public function toRoute($route, $parameters, $absolute)
    {}

    /**
     * Get the URL to a controller action.
     *
     * @param  string|array  $action
     * @param  mixed  $parameters
     * @param  bool  $absolute
     * @return string
     *
     * @throws \InvalidArgumentException
     */
    public function action($action, $parameters = [], $absolute = true)
    {}

    /**
     * Format the given controller action.
     *
     * @param  string|array  $action
     * @return string
     */
    protected function formatAction($action)
    {}

    /**
     * Format the array of URL parameters.
     *
     * @param  mixed  $parameters
     * @return array
     */
    public function formatParameters($parameters)
    {}

    /**
     * Extract the query string from the given path.
     *
     * @param  string  $path
     * @return array
     */
    protected function extractQueryString($path)
    {}

    /**
     * Get the base URL for the request.
     *
     * @param  string  $scheme
     * @param  string|null  $root
     * @return string
     */
    public function formatRoot($scheme, $root = null)
    {}

    /**
     * Format the given URL segments into a single URL.
     *
     * @param  string  $root
     * @param  string  $path
     * @param  \Illuminate\Routing\Route|null  $route
     * @return string
     */
    public function format($root, $path, $route = null)
    {}

    /**
     * Determine if the given path is a valid URL.
     *
     * @param  string  $path
     * @return bool
     */
    public function isValidUrl($path)
    {}

    /**
     * Get the Route URL generator instance.
     *
     * @return \Illuminate\Routing\RouteUrlGenerator
     */
    protected function routeUrl()
    {}

    /**
     * Set the default named parameters used by the URL generator.
     *
     * @param  array  $defaults
     * @return void
     */
    public function defaults(array $defaults)
    {}

    /**
     * Get the default named parameters used by the URL generator.
     *
     * @return array
     */
    public function getDefaultParameters()
    {}

    /**
     * Force the scheme for URLs.
     *
     * @param  string|null  $scheme
     * @return void
     */
    public function forceScheme($scheme)
    {}

    /**
     * Force the use of the HTTPS scheme for all generated URLs.
     *
     * @param  bool  $force
     * @return void
     */
    public function forceHttps($force = true)
    {}

    /**
     * Set the URL origin for all generated URLs.
     *
     * @param  string|null  $root
     * @return void
     */
    public function useOrigin(?string $root)
    {}

    /**
     * Set the forced root URL.
     *
     * @param  string|null  $root
     * @return void
     *
     * @deprecated Use useOrigin
     */
    public function forceRootUrl($root)
    {}

    /**
     * Set the URL origin for all generated asset URLs.
     *
     * @param  string|null  $root
     * @return void
     */
    public function useAssetOrigin(?string $root)
    {}

    /**
     * Set a callback to be used to format the host of generated URLs.
     *
     * @param  \Closure  $callback
     * @return $this
     */
    public function formatHostUsing(Closure $callback)
    {}

    /**
     * Set a callback to be used to format the path of generated URLs.
     *
     * @param  \Closure  $callback
     * @return $this
     */
    public function formatPathUsing(Closure $callback)
    {}

    /**
     * Get the path formatter being used by the URL generator.
     *
     * @return \Closure
     */
    public function pathFormatter()
    {}

    /**
     * Get the request instance.
     *
     * @return \Illuminate\Http\Request
     */
    public function getRequest()
    {}

    /**
     * Set the current request instance.
     *
     * @param  \Illuminate\Http\Request  $request
     * @return void
     */
    public function setRequest(Request $request)
    {}

    /**
     * Set the route collection.
     *
     * @param  \Illuminate\Routing\RouteCollectionInterface  $routes
     * @return $this
     */
    public function setRoutes(RouteCollectionInterface $routes)
    {}

    /**
     * Get the session implementation from the resolver.
     *
     * @return \Illuminate\Session\Store|null
     */
    protected function getSession()
    {}

    /**
     * Set the session resolver for the generator.
     *
     * @param  callable  $sessionResolver
     * @return $this
     */
    public function setSessionResolver(callable $sessionResolver)
    {}

    /**
     * Set the encryption key resolver.
     *
     * @param  callable  $keyResolver
     * @return $this
     */
    public function setKeyResolver(callable $keyResolver)
    {}

    /**
     * Clone a new instance of the URL generator with a different encryption key resolver.
     *
     * @param  callable  $keyResolver
     * @return \Illuminate\Routing\UrlGenerator
     */
    public function withKeyResolver(callable $keyResolver)
    {}

    /**
     * Set the callback that should be used to attempt to resolve missing named routes.
     *
     * @param  callable  $missingNamedRouteResolver
     * @return $this
     */
    public function resolveMissingNamedRoutesUsing(callable $missingNamedRouteResolver)
    {}

    /**
     * Get the root controller namespace.
     *
     * @return string
     */
    public function getRootControllerNamespace()
    {}

    /**
     * Set the root controller namespace.
     *
     * @param  string  $rootNamespace
     * @return $this
     */
    public function setRootControllerNamespace($rootNamespace)
    {}
}
