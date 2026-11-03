<?php

namespace Illuminate\Http\Client;

use ArrayAccess;
use GuzzleHttp\Psr7\StreamWrapper;
use Illuminate\Support\Collection;
use Illuminate\Support\Fluent;
use Illuminate\Support\Traits\Macroable;
use Illuminate\Support\Traits\Tappable;
use LogicException;
use Stringable;

/**
 * @mixin \Psr\Http\Message\ResponseInterface
 */
class Response implements ArrayAccess, Stringable
{
    use Concerns\DeterminesStatusCode, Tappable, Macroable {
        __call as macroCall;
    }

    /**
     * The underlying PSR response.
     *
     * @var \Psr\Http\Message\ResponseInterface
     */
    protected $response;

    /**
     * The decoded JSON response.
     *
     * @var array
     */
    protected $decoded;

    /**
     * Indicates if the JSON response has been decoded.
     *
     * @var bool
     */
    protected bool $decodedJson = false;

    /**
     * The flags that were used when decoding the JSON response.
     *
     * @var int-mask<JSON_BIGINT_AS_STRING, JSON_INVALID_UTF8_IGNORE, JSON_INVALID_UTF8_SUBSTITUTE, JSON_OBJECT_AS_ARRAY, JSON_THROW_ON_ERROR>
     */
    protected int $decodingFlags;

    /**
     * The request cookies.
     *
     * @var \GuzzleHttp\Cookie\CookieJar
     */
    public $cookies;

    /**
     * The transfer stats for the request.
     *
     * @var \GuzzleHttp\TransferStats|null
     */
    public $transferStats;

    /**
     * The length at which request exceptions will be truncated.
     *
     * @var int<1, max>|false|null
     */
    protected $truncateExceptionsAt = null;

    /**
     * The flags passed to `json_decode` by default.
     *
     * @var int-mask<JSON_BIGINT_AS_STRING, JSON_INVALID_UTF8_IGNORE, JSON_INVALID_UTF8_SUBSTITUTE, JSON_OBJECT_AS_ARRAY, JSON_THROW_ON_ERROR>
     */
    public static int $defaultJsonDecodingFlags = 0;

    /**
     * Create a new response instance.
     *
     * @param  \Psr\Http\Message\MessageInterface  $response
     */
    public function __construct($response)
    {}

    /**
     * Get the body of the response.
     *
     * @return string
     */
    public function body()
    {}

    /**
     * Get the decoded JSON body of the response as an array or scalar value.
     *
     * @param  string|null  $key
     * @param  mixed  $default
     * @param  int-mask<JSON_BIGINT_AS_STRING, JSON_INVALID_UTF8_IGNORE, JSON_INVALID_UTF8_SUBSTITUTE, JSON_OBJECT_AS_ARRAY, JSON_THROW_ON_ERROR>|null  $flags
     * @return mixed
     */
    public function json($key = null, $default = null, $flags = null)
    {}

    /**
     * Get the decoded JSON body of the response as an object.
     *
     * @param  int-mask<JSON_BIGINT_AS_STRING, JSON_INVALID_UTF8_IGNORE, JSON_INVALID_UTF8_SUBSTITUTE, JSON_OBJECT_AS_ARRAY, JSON_THROW_ON_ERROR>|null  $flags
     * @return object|null
     */
    public function object($flags = null)
    {}

    /**
     * Get the decoded JSON body of the response as a collection.
     *
     * @param  string|null  $key
     * @param  int-mask<JSON_BIGINT_AS_STRING, JSON_INVALID_UTF8_IGNORE, JSON_INVALID_UTF8_SUBSTITUTE, JSON_OBJECT_AS_ARRAY, JSON_THROW_ON_ERROR>|null  $flags
     * @return \Illuminate\Support\Collection
     */
    public function collect($key = null, $flags = null)
    {}

    /**
     * Get the decoded JSON body of the response as a fluent object.
     *
     * @param  string|null  $key
     * @param  int-mask<JSON_BIGINT_AS_STRING, JSON_INVALID_UTF8_IGNORE, JSON_INVALID_UTF8_SUBSTITUTE, JSON_OBJECT_AS_ARRAY, JSON_THROW_ON_ERROR>|null  $flags
     * @return \Illuminate\Support\Fluent
     */
    public function fluent($key = null, $flags = null)
    {}

    /**
     * Get the body of the response as a PHP resource.
     *
     * @return resource
     *
     * @throws \InvalidArgumentException
     */
    public function resource()
    {}

    /**
     * Get a header from the response.
     *
     * @param  string  $header
     * @return string
     */
    public function header(string $header)
    {}

    /**
     * Get the headers from the response.
     *
     * @return array
     */
    public function headers()
    {}

    /**
     * Get the status code of the response.
     *
     * @return int
     */
    public function status()
    {}

    /**
     * Get the reason phrase of the response.
     *
     * @return string
     */
    public function reason()
    {}

    /**
     * Get the effective URI of the response.
     *
     * @return \Psr\Http\Message\UriInterface|null
     */
    public function effectiveUri()
    {}

    /**
     * Determine if the request was successful.
     *
     * @return bool
     */
    public function successful()
    {}

    /**
     * Determine if the response was a redirect.
     *
     * @return bool
     */
    public function redirect()
    {}

    /**
     * Determine if the response indicates a client or server error occurred.
     *
     * @return bool
     */
    public function failed()
    {}

    /**
     * Determine if the response indicates a client error occurred.
     *
     * @return bool
     */
    public function clientError()
    {}

    /**
     * Determine if the response indicates a server error occurred.
     *
     * @return bool
     */
    public function serverError()
    {}

    /**
     * Execute the given callback if there was a server or client error.
     *
     * @param  callable|(\Closure(\Illuminate\Http\Client\Response): mixed)  $callback
     * @return $this
     */
    public function onError(callable $callback)
    {}

    /**
     * Get the response cookies.
     *
     * @return \GuzzleHttp\Cookie\CookieJar
     */
    public function cookies()
    {}

    /**
     * Get the handler stats of the response.
     *
     * @return array
     */
    public function handlerStats()
    {}

    /**
     * Close the stream and any underlying resources.
     *
     * @return $this
     */
    public function close()
    {}

    /**
     * Get the underlying PSR response for the response.
     *
     * @return \Psr\Http\Message\ResponseInterface
     */
    public function toPsrResponse()
    {}

    /**
     * Create an exception if a server or client error occurred.
     *
     * @return \Illuminate\Http\Client\RequestException|null
     */
    public function toException()
    {}

    /**
     * Throw an exception if a server or client error occurred.
     *
     * @param  null|(\Closure(\Illuminate\Http\Client\Response, \Illuminate\Http\Client\RequestException): mixed)  $callback
     * @return $this
     *
     * @throws \Illuminate\Http\Client\RequestException
     */
    public function throw($callback = null)
    {}

    /**
     * Throw an exception if a server or client error occurred and the given condition evaluates to true.
     *
     * @param  \Closure|bool  $condition
     * @param  null|(\Closure(\Illuminate\Http\Client\Response, \Illuminate\Http\Client\RequestException): mixed)  $callback
     * @return $this
     *
     * @throws \Illuminate\Http\Client\RequestException
     */
    public function throwIf($condition, $callback = null)
    {}

    /**
     * Throw an exception if a server or client error occurred and the given condition evaluates to false.
     *
     * @param  \Closure|bool  $condition
     * @return $this
     *
     * @throws \Illuminate\Http\Client\RequestException
     */
    public function throwUnless($condition)
    {}

    /**
     * Throw an exception if the response status code matches the given code.
     *
     * @param  int|(\Closure(int, \Illuminate\Http\Client\Response): bool)|callable  $statusCode
     * @return $this
     *
     * @throws \Illuminate\Http\Client\RequestException
     */
    public function throwIfStatus($statusCode)
    {}

    /**
     * Throw an exception unless the response status code matches the given code.
     *
     * @param  int|(\Closure(int, \Illuminate\Http\Client\Response): bool)|callable  $statusCode
     * @return $this
     *
     * @throws \Illuminate\Http\Client\RequestException
     */
    public function throwUnlessStatus($statusCode)
    {}

    /**
     * Throw an exception if the response status code is a 4xx level code.
     *
     * @return $this
     *
     * @throws \Illuminate\Http\Client\RequestException
     */
    public function throwIfClientError()
    {}

    /**
     * Throw an exception if the response status code is a 5xx level code.
     *
     * @return $this
     *
     * @throws \Illuminate\Http\Client\RequestException
     */
    public function throwIfServerError()
    {}

    /**
     * Indicate that request exceptions should be truncated to the given length.
     *
     * @param  int<1, max>  $length
     * @return $this
     */
    public function truncateExceptionsAt(int $length)
    {}

    /**
     * Indicate that request exceptions should not be truncated.
     *
     * @return $this
     */
    public function dontTruncateExceptions()
    {}

    /**
     * Dump the content from the response.
     *
     * @param  string|null  $key
     * @return $this
     */
    public function dump($key = null)
    {}

    /**
     * Dump the content from the response and end the script.
     *
     * @param  string|null  $key
     * @return never
     */
    public function dd($key = null)
    {}

    /**
     * Dump the headers from the response.
     *
     * @return $this
     */
    public function dumpHeaders()
    {}

    /**
     * Dump the headers from the response and end the script.
     *
     * @return never
     */
    public function ddHeaders()
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
     * Get the value for a given offset.
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
     *
     * @throws \LogicException
     */
    public function offsetSet($offset, $value): void
    {}

    /**
     * Unset the value at the given offset.
     *
     * @param  string  $offset
     * @return void
     *
     * @throws \LogicException
     */
    public function offsetUnset($offset): void
    {}

    /**
     * Get the body of the response.
     *
     * @return string
     */
    public function __toString()
    {}

    /**
     * Dynamically proxy other methods to the underlying response.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return mixed
     */
    public function __call($method, $parameters)
    {}

    /**
     * Flush the global state of the Response.
     */
    public static function flushState(): void
    {}
}
