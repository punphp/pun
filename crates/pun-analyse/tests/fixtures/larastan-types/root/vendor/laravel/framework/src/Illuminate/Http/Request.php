<?php

namespace Illuminate\Http;

use ArrayAccess;
use Closure;
use Illuminate\Contracts\Support\Arrayable;
use Illuminate\Session\SymfonySessionDecorator;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Illuminate\Support\Str;
use Illuminate\Support\Traits\Conditionable;
use Illuminate\Support\Traits\Macroable;
use Illuminate\Support\Uri;
use RuntimeException;
use Symfony\Component\HttpFoundation\Exception\SessionNotFoundException;
use Symfony\Component\HttpFoundation\InputBag;
use Symfony\Component\HttpFoundation\Request as SymfonyRequest;
use Symfony\Component\HttpFoundation\Session\SessionInterface;

/**
 * @method array validate(array $rules, ...$params)
 * @method array validateWithBag(string $errorBag, array $rules, ...$params)
 * @method bool hasValidSignature(bool $absolute = true)
 * @method bool hasValidRelativeSignature()
 * @method bool hasValidSignatureWhileIgnoring($ignoreQuery = [], $absolute = true)
 * @method bool hasValidRelativeSignatureWhileIgnoring($ignoreQuery = [])
 */
class Request extends SymfonyRequest implements Arrayable, ArrayAccess
{
    use Concerns\CanBePrecognitive,
        Concerns\InteractsWithContentTypes,
        Concerns\InteractsWithFlashData,
        Concerns\InteractsWithInput,
        Conditionable,
        Macroable;

    /**
     * The decoded JSON content for the request.
     *
     * @var \Symfony\Component\HttpFoundation\InputBag|null
     */
    protected $json;

    /**
     * All of the converted files for the request.
     *
     * @var array<string, \Illuminate\Http\UploadedFile|\Illuminate\Http\UploadedFile[]>|null
     */
    protected $convertedFiles;

    /**
     * The user resolver callback.
     *
     * @var \Closure
     */
    protected $userResolver;

    /**
     * The route resolver callback.
     *
     * @var \Closure
     */
    protected $routeResolver;

    /**
     * The cached "Accept" header value.
     *
     * @var string|null
     */
    protected $cachedAcceptHeader;

    /**
     * Create a new Illuminate HTTP request from server variables.
     *
     * @return static
     */
    public static function capture()
    {}

    /**
     * Return the Request instance.
     *
     * @return $this
     */
    public function instance()
    {}

    /**
     * Get the request method.
     *
     * @return string
     */
    public function method()
    {}

    /**
     * Get a URI instance for the request.
     *
     * @return \Illuminate\Support\Uri
     */
    public function uri()
    {}

    /**
     * Get the root URL for the application.
     *
     * @return string
     */
    public function root()
    {}

    /**
     * Get the URL (no query string) for the request.
     *
     * @return string
     */
    public function url()
    {}

    /**
     * Get the full URL for the request.
     *
     * @return string
     */
    public function fullUrl()
    {}

    /**
     * Get the full URL for the request with the added query string parameters.
     *
     * @param  array  $query
     * @return string
     */
    public function fullUrlWithQuery(array $query)
    {}

    /**
     * Get the full URL for the request without the given query string parameters.
     *
     * @param  array|string  $keys
     * @return string
     */
    public function fullUrlWithoutQuery($keys)
    {}

    /**
     * Get the current path info for the request.
     *
     * @return string
     */
    public function path()
    {}

    /**
     * Get the current decoded path info for the request.
     *
     * @return string
     */
    public function decodedPath()
    {}

    /**
     * Get a segment from the URI (1 based index).
     *
     * @param  int  $index
     * @param  string|null  $default
     * @return string|null
     */
    public function segment($index, $default = null)
    {}

    /**
     * Get all of the segments for the request path.
     *
     * @return array
     */
    public function segments()
    {}

    /**
     * Determine if the current request URI matches a pattern.
     *
     * @param  mixed  ...$patterns
     * @return bool
     */
    public function is(...$patterns)
    {}

    /**
     * Determine if the route name matches a given pattern.
     *
     * @param  mixed  ...$patterns
     * @return bool
     */
    public function routeIs(...$patterns)
    {}

    /**
     * Determine if the current request URL and query string match a pattern.
     *
     * @param  mixed  ...$patterns
     * @return bool
     */
    public function fullUrlIs(...$patterns)
    {}

    /**
     * Get the host name.
     *
     * @return string
     */
    public function host()
    {}

    /**
     * Get the HTTP host being requested.
     *
     * @return string
     */
    public function httpHost()
    {}

    /**
     * Get the scheme and HTTP host.
     *
     * @return string
     */
    public function schemeAndHttpHost()
    {}

    /**
     * Determine if the request is the result of an AJAX call.
     *
     * @return bool
     */
    public function ajax()
    {}

    /**
     * Determine if the request is the result of a PJAX call.
     *
     * @return bool
     */
    public function pjax()
    {}

    /**
     * Determine if the request is the result of a prefetch call.
     *
     * @return bool
     */
    public function prefetch()
    {}

    /**
     * Determine if the request is over HTTPS.
     *
     * @return bool
     */
    public function secure()
    {}

    /**
     * Get the client IP address.
     *
     * @return string|null
     */
    public function ip()
    {}

    /**
     * Get the client IP addresses.
     *
     * @return array
     */
    public function ips()
    {}

    /**
     * Get the client user agent.
     *
     * @return string|null
     */
    public function userAgent()
    {}

    /**
     * {@inheritdoc}
     */
    #[\Override]
    public function getAcceptableContentTypes(): array
    {}

    /**
     * Merge new input into the current request's input array.
     *
     * @param  array  $input
     * @return $this
     */
    public function merge(array $input)
    {}

    /**
     * Merge new input into the request's input, but only when that key is missing from the request.
     *
     * @param  array  $input
     * @return $this
     */
    public function mergeIfMissing(array $input)
    {}

    /**
     * Replace the input values for the current request.
     *
     * @param  array  $input
     * @return $this
     */
    public function replace(array $input)
    {}

    /**
     * This method belongs to Symfony HttpFoundation and is not usually needed when using Laravel.
     *
     * Instead, you may use the "input" method.
     *
     * @param  string  $key
     * @param  mixed  $default
     * @return mixed
     *
     * @deprecated use ->input() instead
     */
    public function get(string $key, mixed $default = null): mixed
    {}

    /**
     * Get the JSON payload for the request.
     *
     * @param  string|null  $key
     * @param  mixed  $default
     * @return ($key is null ? \Symfony\Component\HttpFoundation\InputBag : mixed)
     */
    public function json($key = null, $default = null)
    {}

    /**
     * Get the input source for the request.
     *
     * @return \Symfony\Component\HttpFoundation\InputBag
     */
    protected function getInputSource()
    {}

    /**
     * Create a new request instance from the given Laravel request.
     *
     * @param  \Illuminate\Http\Request  $from
     * @param  \Illuminate\Http\Request|null  $to
     * @return static
     */
    public static function createFrom(self $from, $to = null)
    {}

    /**
     * Create an Illuminate request from a Symfony instance.
     *
     * @param  \Symfony\Component\HttpFoundation\Request  $request
     * @return static
     */
    public static function createFromBase(SymfonyRequest $request)
    {}

    /**
     * {@inheritdoc}
     *
     * @return static
     */
    #[\Override]
    public function duplicate(?array $query = null, ?array $request = null, ?array $attributes = null, ?array $cookies = null, ?array $files = null, ?array $server = null): static
    {}

    /**
     * Filter the given array of files, removing any empty values.
     *
     * @param  mixed  $files
     * @return mixed
     */
    protected function filterFiles($files)
    {}

    /**
     * {@inheritdoc}
     */
    #[\Override]
    public function hasSession(bool $skipIfUninitialized = false): bool
    {}

    /**
     * {@inheritdoc}
     *
     * @throws \Symfony\Component\HttpFoundation\Exception\SessionNotFoundException
     */
    #[\Override]
    public function getSession(): SessionInterface
    {}

    /**
     * Get the session associated with the request.
     *
     * @return \Illuminate\Contracts\Session\Session
     *
     * @throws \RuntimeException
     */
    public function session()
    {}

    /**
     * Set the session instance on the request.
     *
     * @param  \Illuminate\Contracts\Session\Session  $session
     * @return void
     */
    public function setLaravelSession($session)
    {}

    /**
     * Set the locale for the request instance.
     *
     * @param  string  $locale
     * @return void
     */
    public function setRequestLocale(string $locale)
    {}

    /**
     * Set the default locale for the request instance.
     *
     * @param  string  $locale
     * @return void
     */
    public function setDefaultRequestLocale(string $locale)
    {}

    /**
     * Get the user making the request.
     *
     * @param  string|null  $guard
     * @return mixed
     */
    public function user($guard = null)
    {}

    /**
     * Get the route handling the request.
     *
     * @param  string|null  $param
     * @param  mixed  $default
     * @return ($param is null ? \Illuminate\Routing\Route|null : object|string|null)
     */
    public function route($param = null, $default = null)
    {}

    /**
     * Get a unique fingerprint for the request / route / IP address.
     *
     * @return string
     *
     * @throws \RuntimeException
     */
    public function fingerprint()
    {}

    /**
     * Set the JSON payload for the request.
     *
     * @param  \Symfony\Component\HttpFoundation\InputBag  $json
     * @return $this
     */
    public function setJson($json)
    {}

    /**
     * Get the user resolver callback.
     *
     * @return \Closure
     */
    public function getUserResolver()
    {}

    /**
     * Set the user resolver callback.
     *
     * @param  \Closure  $callback
     * @return $this
     */
    public function setUserResolver(Closure $callback)
    {}

    /**
     * Get the route resolver callback.
     *
     * @return \Closure
     */
    public function getRouteResolver()
    {}

    /**
     * Set the route resolver callback.
     *
     * @param  \Closure  $callback
     * @return $this
     */
    public function setRouteResolver(Closure $callback)
    {}

    /**
     * Get all of the input and files for the request.
     *
     * @return array
     */
    public function toArray(): array
    {}

    /**
     * Determine if the given offset exists.
     *
     * @param  string  $offset
     * @return bool
     */
    public function offsetExists($offset): bool
    {}

    /**
     * Get the value at the given offset.
     *
     * @param  string  $offset
     * @return mixed
     */
    public function offsetGet($offset): mixed
    {}

    /**
     * Set the value at the given offset.
     *
     * @param  string  $offset
     * @param  mixed  $value
     * @return void
     */
    public function offsetSet($offset, $value): void
    {}

    /**
     * Remove the value at the given offset.
     *
     * @param  string  $offset
     * @return void
     */
    public function offsetUnset($offset): void
    {}

    /**
     * Check if an input element is set on the request.
     *
     * @param  string  $key
     * @return bool
     */
    public function __isset($key)
    {}

    /**
     * Get an input element from the request.
     *
     * @param  string  $key
     * @return mixed
     */
    public function __get($key)
    {}
}
