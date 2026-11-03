<?php

namespace Illuminate\Http\Client;

use Closure;
use GuzzleHttp\Exception\ConnectException;
use GuzzleHttp\Middleware;
use GuzzleHttp\Promise\Create;
use GuzzleHttp\Promise\PromiseInterface;
use GuzzleHttp\Psr7\Response as Psr7Response;
use GuzzleHttp\TransferStats;
use Illuminate\Contracts\Events\Dispatcher;
use Illuminate\Support\Collection;
use Illuminate\Support\Str;
use Illuminate\Support\Stringable;
use Illuminate\Support\Traits\Macroable;
use InvalidArgumentException;
use JsonException;
use PHPUnit\Framework\Assert as PHPUnit;
use Psr\Http\Message\StreamInterface;

/**
 * @mixin \Illuminate\Http\Client\PendingRequest
 */
class Factory
{
    use Macroable {
        __call as macroCall;
    }

    /**
     * The event dispatcher implementation.
     *
     * @var \Illuminate\Contracts\Events\Dispatcher|null
     */
    protected $dispatcher;

    /**
     * The middleware to apply to every request.
     *
     * @var array
     */
    protected $globalMiddleware = [];

    /**
     * The options to apply to every request.
     *
     * @var \Closure|array
     */
    protected $globalOptions = [];

    /**
     * The stub callables that will handle requests.
     *
     * @var \Illuminate\Support\Collection
     */
    protected $stubCallbacks;

    /**
     * Indicates if the factory is recording requests and responses.
     *
     * @var bool
     */
    protected $recording = false;

    /**
     * The recorded response array.
     *
     * @var list<array{0: \Illuminate\Http\Client\Request, 1: \Illuminate\Http\Client\Response|null}>
     */
    protected $recorded = [];

    /**
     * All created response sequences.
     *
     * @var list<\Illuminate\Http\Client\ResponseSequence>
     */
    protected $responseSequences = [];

    /**
     * Indicates that an exception should be thrown if any request is not faked.
     *
     * @var bool
     */
    protected $preventStrayRequests = false;

    /**
     * A list of URL patterns that are allowed to bypass the stray request guard.
     *
     * @var array<int, string>
     */
    protected $allowedStrayRequestUrls = [];

    /**
     * Create a new factory instance.
     *
     * @param  \Illuminate\Contracts\Events\Dispatcher|null  $dispatcher
     */
    public function __construct(?Dispatcher $dispatcher = null)
    {}

    /**
     * Add middleware to apply to every request.
     *
     * @param  callable  $middleware
     * @return $this
     */
    public function globalMiddleware($middleware)
    {}

    /**
     * Add request middleware to apply to every request.
     *
     * @param  callable  $middleware
     * @return $this
     */
    public function globalRequestMiddleware($middleware)
    {}

    /**
     * Add response middleware to apply to every request.
     *
     * @param  callable  $middleware
     * @return $this
     */
    public function globalResponseMiddleware($middleware)
    {}

    /**
     * Set the options to apply to every request.
     *
     * @param  \Closure|array  $options
     * @return $this
     */
    public function globalOptions($options)
    {}

    /**
     * Execute a callback while requests are created without global middleware or global options.
     *
     * @template TReturn
     *
     * @param  (\Closure(): TReturn)  $callback
     * @return TReturn
     */
    public function withoutGlobalConfiguration(Closure $callback)
    {}

    /**
     * Create a new response instance for use during stubbing.
     *
     * @param  \Psr\Http\Message\StreamInterface|array|string|resource|null  $body
     * @param  int  $status
     * @param  array  $headers
     * @return \GuzzleHttp\Promise\PromiseInterface
     */
    public static function response($body = null, $status = 200, $headers = [])
    {}

    /**
     * Create a new PSR-7 response instance for use during stubbing.
     *
     * @param  \Psr\Http\Message\StreamInterface|array|string|resource|null  $body
     * @param  int  $status
     * @param  array  $headers
     * @return \GuzzleHttp\Psr7\Response
     *
     * @throws \InvalidArgumentException
     */
    public static function psr7Response($body = null, $status = 200, $headers = [])
    {}

    /**
     * Normalize the given fake response headers.
     *
     * @param  array  $headers
     * @return array
     *
     * @throws \InvalidArgumentException
     */
    protected static function normalizeResponseHeaders(array $headers): array
    {}

    /**
     * Normalize a scalar to a string without triggering PHP 8.5 non-finite float warnings.
     *
     * @param  scalar  $value
     * @return string
     */
    protected static function normalizeScalarString($value): string
    {}

    /**
     * Create a new RequestException instance for use during stubbing.
     *
     * @param  \Psr\Http\Message\StreamInterface|array|string|resource|null  $body
     * @param  int  $status
     * @param  array  $headers
     * @return \Illuminate\Http\Client\RequestException
     */
    public static function failedRequest($body = null, $status = 200, $headers = [])
    {}

    /**
     * Create a new connection exception for use during stubbing.
     *
     * @param  string|null  $message
     * @return \Closure(\Illuminate\Http\Client\Request): \GuzzleHttp\Promise\PromiseInterface
     */
    public static function failedConnection($message = null)
    {}

    /**
     * Get an invokable object that returns a sequence of responses in order for use during stubbing.
     *
     * @param  array  $responses
     * @return \Illuminate\Http\Client\ResponseSequence
     */
    public function sequence(array $responses = [])
    {}

    /**
     * Register a stub callable that will intercept requests and be able to return stub responses.
     *
     * @param  callable|array<string, mixed>|null  $callback
     * @return $this
     */
    public function fake($callback = null)
    {}

    /**
     * Register a response sequence for the given URL pattern.
     *
     * @param  string  $url
     * @return \Illuminate\Http\Client\ResponseSequence
     */
    public function fakeSequence($url = '*')
    {}

    /**
     * Stub the given URL using the given callback.
     *
     * @param  string  $url
     * @param  \Illuminate\Http\Client\Response|\GuzzleHttp\Promise\PromiseInterface|callable|int|string|array|\Illuminate\Http\Client\ResponseSequence  $callback
     * @return $this
     *
     * @throws \InvalidArgumentException
     */
    public function stubUrl($url, $callback)
    {}

    /**
     * Indicate that an exception should be thrown if any request is not faked.
     *
     * @param  bool  $prevent
     * @return $this
     */
    public function preventStrayRequests($prevent = true)
    {}

    /**
     * Determine if stray requests are being prevented.
     *
     * @return bool
     */
    public function preventingStrayRequests()
    {}

    /**
     * Allow stray, unfaked requests entirely, or optionally allow only specific URLs.
     *
     * @param  array<int, string>|null  $only
     * @return $this
     */
    public function allowStrayRequests(?array $only = null)
    {}

    /**
     * Begin recording request / response pairs.
     *
     * @return $this
     */
    public function record()
    {}

    /**
     * Record a request response pair.
     *
     * @param  \Illuminate\Http\Client\Request  $request
     * @param  \Illuminate\Http\Client\Response|null  $response
     * @return void
     */
    public function recordRequestResponsePair($request, $response)
    {}

    /**
     * Assert that a request / response pair was recorded matching a given truth test.
     *
     * @param  callable|(\Closure(\Illuminate\Http\Client\Request, \Illuminate\Http\Client\Response|null): bool)  $callback
     * @return void
     */
    public function assertSent($callback)
    {}

    /**
     * Assert that the given request was sent in the given order.
     *
     * @param  list<string|(\Closure(\Illuminate\Http\Client\Request, \Illuminate\Http\Client\Response|null): bool)|callable>  $callbacks
     * @return void
     */
    public function assertSentInOrder($callbacks)
    {}

    /**
     * Assert that a request / response pair was not recorded matching a given truth test.
     *
     * @param  callable|(\Closure(\Illuminate\Http\Client\Request, \Illuminate\Http\Client\Response|null): bool)  $callback
     * @return void
     */
    public function assertNotSent($callback)
    {}

    /**
     * Assert that no request / response pair was recorded.
     *
     * @return void
     */
    public function assertNothingSent()
    {}

    /**
     * Assert how many requests have been recorded.
     *
     * @param  int  $count
     * @return void
     */
    public function assertSentCount($count)
    {}

    /**
     * Assert that every created response sequence is empty.
     *
     * @return void
     */
    public function assertSequencesAreEmpty()
    {}

    /**
     * Get a collection of the request / response pairs matching the given truth test.
     *
     * @param  (\Closure(\Illuminate\Http\Client\Request, \Illuminate\Http\Client\Response|null): bool)|callable|null  $callback
     * @return \Illuminate\Support\Collection<int, array{0: \Illuminate\Http\Client\Request, 1: \Illuminate\Http\Client\Response|null}>
     */
    public function recorded($callback = null)
    {}

    /**
     * Create a new pending request instance for this factory.
     *
     * @return \Illuminate\Http\Client\PendingRequest
     */
    public function createPendingRequest()
    {}

    /**
     * Instantiate a new pending request instance for this factory.
     *
     * @return \Illuminate\Http\Client\PendingRequest
     */
    protected function newPendingRequest()
    {}

    /**
     * Get the current event dispatcher implementation.
     *
     * @return \Illuminate\Contracts\Events\Dispatcher|null
     */
    public function getDispatcher()
    {}

    /**
     * Get the array of global middleware.
     *
     * @return array
     */
    public function getGlobalMiddleware()
    {}

    /**
     * Execute a method against a new pending request instance.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return mixed
     */
    public function __call($method, $parameters)
    {}
}
