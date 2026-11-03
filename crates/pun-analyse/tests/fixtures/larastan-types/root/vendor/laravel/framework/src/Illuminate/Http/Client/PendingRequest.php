<?php

namespace Illuminate\Http\Client;

use Closure;
use Exception;
use GuzzleHttp\Client;
use GuzzleHttp\Cookie\CookieJar;
use GuzzleHttp\Exception\RequestException;
use GuzzleHttp\Exception\ResponseException;
use GuzzleHttp\Exception\TransferException;
use GuzzleHttp\HandlerStack;
use GuzzleHttp\Middleware;
use GuzzleHttp\Promise\EachPromise;
use GuzzleHttp\Promise\PromiseInterface;
use GuzzleHttp\Psr7\Utils;
use GuzzleHttp\UriTemplate\UriTemplate;
use Illuminate\Contracts\Support\Arrayable;
use Illuminate\Http\Client\Events\ConnectionFailed;
use Illuminate\Http\Client\Events\RequestSending;
use Illuminate\Http\Client\Events\ResponseReceived;
use Illuminate\Http\Client\Promises\FluentPromise;
use Illuminate\Http\Client\Promises\LazyPromise;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Illuminate\Support\Str;
use Illuminate\Support\Stringable;
use Illuminate\Support\Traits\Conditionable;
use Illuminate\Support\Traits\Macroable;
use InvalidArgumentException;
use JsonSerializable;
use Psr\Http\Message\MessageInterface;
use Psr\Http\Message\RequestInterface;
use Psr\Http\Message\ResponseInterface;
use Psr\Http\Message\StreamInterface;
use Symfony\Component\VarDumper\VarDumper;
use Throwable;

/**
 * @template TAsync of bool = false
 */
class PendingRequest
{
    use Conditionable, Macroable;

    /**
     * The factory instance.
     *
     * @var \Illuminate\Http\Client\Factory|null
     */
    protected $factory;

    /**
     * The Guzzle client instance.
     *
     * @var \GuzzleHttp\Client|null
     */
    protected $client;

    /**
     * The Guzzle HTTP handler.
     *
     * @var callable
     */
    protected $handler;

    /**
     * The base URL for the request.
     *
     * @var string
     */
    protected $baseUrl = '';

    /**
     * The parameters that can be substituted into the URL.
     *
     * @var array
     */
    protected $urlParameters = [];

    /**
     * The request body format.
     *
     * @var string
     */
    protected $bodyFormat;

    /**
     * The raw body for the request.
     *
     * @var \Psr\Http\Message\StreamInterface|string
     */
    protected $pendingBody;

    /**
     * The pending files for the request.
     *
     * @var array
     */
    protected $pendingFiles = [];

    /**
     * The request cookies.
     *
     * @var \GuzzleHttp\Cookie\CookieJar
     */
    protected $cookies;

    /**
     * The transfer stats for the request.
     *
     * @var \GuzzleHttp\TransferStats
     */
    protected $transferStats;

    /**
     * The request options.
     *
     * @var array
     */
    protected $options = [];

    /**
     * A callback to run when throwing if a server or client error occurs.
     *
     * @var \Closure
     */
    protected $throwCallback;

    /**
     * A callback to check if an exception should be thrown when a server or client error occurs.
     *
     * @var \Closure
     */
    protected $throwIfCallback;

    /**
     * The number of times to try the request.
     *
     * @var array<int, int>|int
     */
    protected $tries = 1;

    /**
     * The number of milliseconds to wait between retries.
     *
     * @var (Closure(int, mixed): int)|int
     */
    protected $retryDelay = 100;

    /**
     * Whether to throw an exception when all retries fail.
     *
     * @var bool
     */
    protected $retryThrow = true;

    /**
     * The callback that will determine if the request should be retried.
     *
     * @var (callable(\Throwable, static, string|null): bool)|null
     */
    protected $retryWhenCallback = null;

    /**
     * The callbacks that should execute before the request is sent.
     *
     * @var \Illuminate\Support\Collection
     */
    protected $beforeSendingCallbacks;

    /**
     * The callbacks that should execute after the Laravel Response is built.
     *
     * @var \Illuminate\Support\Collection<int, (callable(\Illuminate\Http\Client\Response, \Illuminate\Http\Client\Request): \Illuminate\Http\Client\Response|null)>
     */
    protected $afterResponseCallbacks;

    /**
     * The stub callables that will handle requests.
     *
     * @var \Illuminate\Support\Collection|null
     */
    protected $stubCallbacks;

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
     * The middleware callables added by users that will handle requests.
     *
     * @var \Illuminate\Support\Collection
     */
    protected $middleware;

    /**
     * Whether the requests should be asynchronous.
     *
     * @var TAsync
     */
    protected $async = false;

    /**
     * The attributes to track with the request.
     *
     * @var array<array-key, mixed>
     */
    protected $attributes = [];

    /**
     * The pending request promise.
     *
     * @var \GuzzleHttp\Promise\PromiseInterface
     */
    protected $promise;

    /**
     * The sent request object, if a request has been made.
     *
     * @var \Illuminate\Http\Client\Request|null
     */
    protected $request;

    /**
     * The Guzzle request options that are mergeable via array_merge_recursive.
     *
     * @var array
     */
    protected $mergeableOptions = [
        'cookies',
        'form_params',
        'headers',
        'json',
        'multipart',
        'query',
    ];

    /**
     * The length at which request exceptions will be truncated.
     *
     * @var int<1, max>|false|null
     */
    protected $truncateExceptionsAt = null;

    /**
     * Create a new HTTP Client instance.
     *
     * @param  \Illuminate\Http\Client\Factory|null  $factory
     * @param  array  $middleware
     */
    public function __construct(?Factory $factory = null, $middleware = [])
    {}

    /**
     * Set the base URL for the pending request.
     *
     * @param  string  $url
     * @return $this
     */
    public function baseUrl(string $url)
    {}

    /**
     * Attach a raw body to the request.
     *
     * @param  \Psr\Http\Message\StreamInterface|string  $content
     * @param  string  $contentType
     * @return $this
     */
    public function withBody($content, $contentType = 'application/json')
    {}

    /**
     * Indicate the request contains JSON.
     *
     * @return $this
     */
    public function asJson()
    {}

    /**
     * Indicate the request contains form parameters.
     *
     * @return $this
     */
    public function asForm()
    {}

    /**
     * Attach a file to the request.
     *
     * @param  string|array  $name
     * @param  string|resource  $contents
     * @param  string|null  $filename
     * @param  array  $headers
     * @return $this
     */
    public function attach($name, $contents = '', $filename = null, array $headers = [])
    {}

    /**
     * Indicate the request is a multi-part form request.
     *
     * @return $this
     */
    public function asMultipart()
    {}

    /**
     * Specify the body format of the request.
     *
     * @param  string  $format
     * @return $this
     */
    public function bodyFormat(string $format)
    {}

    /**
     * Set the given query parameters in the request URI.
     *
     * @param  array  $parameters
     * @return $this
     */
    public function withQueryParameters(array $parameters)
    {}

    /**
     * Specify the request's content type.
     *
     * @param  string  $contentType
     * @return $this
     */
    public function contentType(string $contentType)
    {}

    /**
     * Indicate that JSON should be returned by the server.
     *
     * @return $this
     */
    public function acceptJson()
    {}

    /**
     * Indicate the type of content that should be returned by the server.
     *
     * @param  string  $contentType
     * @return $this
     */
    public function accept($contentType)
    {}

    /**
     * Add the given headers to the request.
     *
     * @param  array  $headers
     * @return $this
     */
    public function withHeaders(array $headers)
    {}

    /**
     * Add the given header to the request.
     *
     * @param  string  $name
     * @param  mixed  $value
     * @return $this
     */
    public function withHeader($name, $value)
    {}

    /**
     * Replace the given headers on the request.
     *
     * @param  array  $headers
     * @return $this
     */
    public function replaceHeaders(array $headers)
    {}

    /**
     * Specify the basic authentication username and password for the request.
     *
     * @param  string  $username
     * @param  string  $password
     * @return $this
     */
    public function withBasicAuth(string $username, #[\SensitiveParameter] string $password)
    {}

    /**
     * Specify the digest authentication username and password for the request.
     *
     * @param  string  $username
     * @param  string  $password
     * @return $this
     */
    public function withDigestAuth($username, #[\SensitiveParameter] $password)
    {}

    /**
     * Specify the NTLM authentication username and password for the request.
     *
     * @param  string  $username
     * @param  string  $password
     * @return $this
     */
    public function withNtlmAuth($username, #[\SensitiveParameter] $password)
    {}

    /**
     * Specify an authorization token for the request.
     *
     * @param  string  $token
     * @param  string  $type
     * @return $this
     */
    public function withToken(#[\SensitiveParameter] $token, $type = 'Bearer')
    {}

    /**
     * Specify the user agent for the request.
     *
     * @param  string|bool  $userAgent
     * @return $this
     */
    public function withUserAgent($userAgent)
    {}

    /**
     * Specify the URL parameters that can be substituted into the request URL.
     *
     * @param  array  $parameters
     * @return $this
     */
    public function withUrlParameters(array $parameters = [])
    {}

    /**
     * Specify the cookies that should be included with the request.
     *
     * @param  array  $cookies
     * @param  string  $domain
     * @return $this
     */
    public function withCookies(array $cookies, string $domain)
    {}

    /**
     * Specify the maximum number of redirects to allow.
     *
     * @param  int  $max
     * @return $this
     */
    public function maxRedirects(int $max)
    {}

    /**
     * Indicate that redirects should not be followed.
     *
     * @return $this
     */
    public function withoutRedirecting()
    {}

    /**
     * Indicate that TLS certificates should not be verified.
     *
     * @return $this
     */
    public function withoutVerifying()
    {}

    /**
     * Specify the path where the body of the response should be stored.
     *
     * @param  string|resource  $to
     * @return $this
     */
    public function sink($to)
    {}

    /**
     * Specify the timeout (in seconds) for the request.
     *
     * @param  int|float  $seconds
     * @return $this
     */
    public function timeout(int|float $seconds)
    {}

    /**
     * Specify the connect timeout (in seconds) for the request.
     *
     * @param  int|float  $seconds
     * @return $this
     */
    public function connectTimeout(int|float $seconds)
    {}

    /**
     * Specify the number of times the request should be attempted.
     *
     * @param  array|int  $times
     * @param  (Closure(int, mixed): int)|int  $sleepMilliseconds
     * @param  (callable(\Throwable, static, string|null): bool)|null  $when
     * @param  bool  $throw
     * @return $this
     */
    public function retry(array|int $times, Closure|int $sleepMilliseconds = 0, ?callable $when = null, bool $throw = true)
    {}

    /**
     * Replace the specified options on the request.
     *
     * @param  array  $options
     * @return $this
     */
    public function withOptions(array $options)
    {}

    /**
     * Add new middleware the client handler stack.
     *
     * @param  callable  $middleware
     * @return $this
     */
    public function withMiddleware(callable $middleware)
    {}

    /**
     * Add new request middleware the client handler stack.
     *
     * @param  callable  $middleware
     * @return $this
     */
    public function withRequestMiddleware(callable $middleware)
    {}

    /**
     * Add new response middleware the client handler stack.
     *
     * @param  callable  $middleware
     * @return $this
     */
    public function withResponseMiddleware(callable $middleware)
    {}

    /**
     * Set arbitrary attributes to store with the request.
     *
     * @param  array<array-key, mixed>  $attributes
     * @return $this
     */
    public function withAttributes($attributes)
    {}

    /**
     * Add a new "before sending" callback to the request.
     *
     * @param  callable  $callback
     * @return $this
     */
    public function beforeSending($callback)
    {}

    /**
     * Add a new callback to execute after the response is built.
     *
     * @param  callable(\Illuminate\Http\Client\Response, \Illuminate\Http\Client\Request): (\Illuminate\Http\Client\Response|null)  $callback
     * @return $this
     */
    public function afterResponse(callable $callback)
    {}

    /**
     * Throw an exception if a server or client error occurs.
     *
     * @param  callable|null  $callback
     * @return $this
     */
    public function throw(?callable $callback = null)
    {}

    /**
     * Throw an exception if a server or client error occurred and the given condition evaluates to true.
     *
     * @param  callable|bool  $condition
     * @return $this
     */
    public function throwIf($condition)
    {}

    /**
     * Throw an exception if a server or client error occurred and the given condition evaluates to false.
     *
     * @param  callable|bool  $condition
     * @return $this
     */
    public function throwUnless($condition)
    {}

    /**
     * Dump the request before sending.
     *
     * @return $this
     */
    public function dump()
    {}

    /**
     * Dump the request before sending and end the script.
     *
     * @return $this
     */
    public function dd()
    {}

    /**
     * Issue a GET request to the given URL.
     *
     * @param  string  $url
     * @param  array|string|null  $query
     * @return \Illuminate\Http\Client\Response|\GuzzleHttp\Promise\PromiseInterface
     *
     * @phpstan-return (TAsync is false ?  \Illuminate\Http\Client\Response : \GuzzleHttp\Promise\PromiseInterface)
     *
     * @throws \Illuminate\Http\Client\ConnectionException
     */
    public function get(string $url, $query = null)
    {}

    /**
     * Issue a HEAD request to the given URL.
     *
     * @param  string  $url
     * @param  array|string|null  $query
     * @return \Illuminate\Http\Client\Response|\GuzzleHttp\Promise\PromiseInterface
     *
     * @phpstan-return (TAsync is false ?  \Illuminate\Http\Client\Response : \GuzzleHttp\Promise\PromiseInterface)
     *
     * @throws \Illuminate\Http\Client\ConnectionException
     */
    public function head(string $url, $query = null)
    {}

    /**
     * Issue a QUERY request to the given URL.
     *
     * @param  string  $url
     * @param  array|\JsonSerializable|\Illuminate\Contracts\Support\Arrayable  $data
     * @return \Illuminate\Http\Client\Response|\GuzzleHttp\Promise\PromiseInterface
     *
     * @phpstan-return (TAsync is false ?  \Illuminate\Http\Client\Response : \GuzzleHttp\Promise\PromiseInterface)
     *
     * @throws \Illuminate\Http\Client\ConnectionException
     */
    public function query(string $url, $data = [])
    {}

    /**
     * Issue a POST request to the given URL.
     *
     * @param  string  $url
     * @param  array|\JsonSerializable|\Illuminate\Contracts\Support\Arrayable  $data
     * @return \Illuminate\Http\Client\Response|\GuzzleHttp\Promise\PromiseInterface
     *
     * @phpstan-return (TAsync is false ?  \Illuminate\Http\Client\Response : \GuzzleHttp\Promise\PromiseInterface)
     *
     * @throws \Illuminate\Http\Client\ConnectionException
     */
    public function post(string $url, $data = [])
    {}

    /**
     * Issue a PATCH request to the given URL.
     *
     * @param  string  $url
     * @param  array|\JsonSerializable|\Illuminate\Contracts\Support\Arrayable  $data
     * @return \Illuminate\Http\Client\Response|\GuzzleHttp\Promise\PromiseInterface
     *
     * @phpstan-return (TAsync is false ?  \Illuminate\Http\Client\Response : \GuzzleHttp\Promise\PromiseInterface)
     *
     * @throws \Illuminate\Http\Client\ConnectionException
     */
    public function patch(string $url, $data = [])
    {}

    /**
     * Issue a PUT request to the given URL.
     *
     * @param  string  $url
     * @param  array|\JsonSerializable|\Illuminate\Contracts\Support\Arrayable  $data
     * @return \Illuminate\Http\Client\Response|\GuzzleHttp\Promise\PromiseInterface
     *
     * @phpstan-return (TAsync is false ?  \Illuminate\Http\Client\Response : \GuzzleHttp\Promise\PromiseInterface)
     *
     * @throws \Illuminate\Http\Client\ConnectionException
     */
    public function put(string $url, $data = [])
    {}

    /**
     * Issue a DELETE request to the given URL.
     *
     * @param  string  $url
     * @param  array|\JsonSerializable|\Illuminate\Contracts\Support\Arrayable  $data
     * @return \Illuminate\Http\Client\Response|\GuzzleHttp\Promise\PromiseInterface
     *
     * @phpstan-return (TAsync is false ?  \Illuminate\Http\Client\Response : \GuzzleHttp\Promise\PromiseInterface)
     *
     * @throws \Illuminate\Http\Client\ConnectionException
     */
    public function delete(string $url, $data = [])
    {}

    /**
     * Send a pool of asynchronous requests concurrently.
     *
     * @param  (callable(\Illuminate\Http\Client\Pool): mixed)  $callback
     * @param  non-negative-int|null  $concurrency
     * @return array<array-key, \Illuminate\Http\Client\Response|\Throwable>
     */
    public function pool(callable $callback, ?int $concurrency = 0)
    {}

    /**
     * Send a pool of asynchronous requests concurrently, with callbacks for introspection.
     *
     * @param  callable  $callback
     * @return \Illuminate\Http\Client\Batch
     */
    public function batch(callable $callback): Batch
    {}

    /**
     * Send the request to the given URL.
     *
     * @param  string  $method
     * @param  string  $url
     * @param  array  $options
     * @return \Illuminate\Http\Client\Response|\Illuminate\Http\Client\Promises\LazyPromise
     *
     * @phpstan-return (TAsync is false ? \Illuminate\Http\Client\Response : \Illuminate\Http\Client\Promises\LazyPromise)
     *
     * @throws \Exception
     * @throws \Illuminate\Http\Client\ConnectionException
     */
    public function send(string $method, string $url, array $options = [])
    {}

    /**
     * Substitute the URL parameters in the given URL.
     *
     * @param  string  $url
     * @return string
     */
    protected function expandUrlParameters(string $url)
    {}

    /**
     * Parse the given HTTP options and set the appropriate additional options.
     *
     * @param  array  $options
     * @return array
     */
    protected function parseHttpOptions(array $options)
    {}

    /**
     * Parse multi-part form data.
     *
     * @param  array  $data
     * @return array|array[]
     */
    protected function parseMultipartBodyFormat(array $data)
    {}

    /**
     * Wrap the multipart stream resources in PSR-7 streams so they are not closed between retry attempts.
     *
     * @param  array  $multipart
     * @return array
     */
    protected function wrapMultipartStreams(array $multipart)
    {}

    /**
     * Rewind the seekable multipart streams so retried requests contain the full part contents.
     *
     * @param  array  $options
     * @return void
     */
    protected function rewindMultipartStreams(array $options)
    {}

    /**
     * Send an asynchronous request to the given URL.
     *
     * @param  string  $method
     * @param  string  $url
     * @param  array  $options
     * @param  int  $attempt
     * @return \GuzzleHttp\Promise\PromiseInterface
     */
    protected function makePromise(string $method, string $url, array $options = [], int $attempt = 1)
    {}

    /**
     * Handle the response of an asynchronous request.
     *
     * @param  \Illuminate\Http\Client\Response|\Throwable  $response
     * @param  string  $method
     * @param  string  $url
     * @param  array  $options
     * @param  int  $attempt
     * @return mixed
     */
    protected function handlePromiseResponse(Response|Throwable $response, $method, $url, $options, $attempt)
    {}

    /**
     * Get the maximum number of attempts for the request.
     *
     * @return int
     */
    protected function getMaximumAttempts()
    {}

    /**
     * Get the delay in milliseconds before the next retry attempt.
     *
     * @param  int  $attempt
     * @param  mixed  $exception
     * @return int|float
     */
    protected function retryDelayInMilliseconds($attempt, $exception)
    {}

    /**
     * Send a request either synchronously or asynchronously.
     *
     * @param  string  $method
     * @param  string  $url
     * @param  array  $options
     * @return \Psr\Http\Message\MessageInterface|\GuzzleHttp\Promise\PromiseInterface
     *
     * @throws \Exception
     */
    protected function sendRequest(string $method, string $url, array $options = [])
    {}

    /**
     * Get the request data as an array so that we can attach it to the request for convenient assertions.
     *
     * @param  string  $method
     * @param  string  $url
     * @param  array  $options
     * @return array
     */
    protected function parseRequestData($method, $url, array $options)
    {}

    /**
     * Normalize the given request options.
     *
     * @param  array  $options
     * @return array
     */
    protected function normalizeRequestOptions(array $options)
    {}

    /**
     * Normalize the given header values.
     *
     * @param  array  $headers
     * @return array
     */
    protected function normalizeHeaderValues(array $headers): array
    {}

    /**
     * Normalize the given header value.
     *
     * @param  mixed  $value
     * @return string|array
     *
     * @throws \InvalidArgumentException
     */
    protected function normalizeHeaderValue($value): string|array
    {}

    /**
     * Normalize non-finite floats within a nested array.
     *
     * @param  array  $values
     * @return array
     */
    protected function normalizeNonFiniteFloatValues(array $values): array
    {}

    /**
     * Normalize the given multipart option.
     *
     * @param  array  $multipart
     * @return array
     */
    protected function normalizeMultipartOption(array $multipart): array
    {}

    /**
     * Normalize the given multipart headers.
     *
     * @param  array  $multipart
     * @return array
     *
     * @throws \InvalidArgumentException
     */
    protected function normalizeMultipartHeaders(array $multipart): array
    {}

    /**
     * Normalize the given request option value.
     *
     * @param  mixed  $value
     * @return mixed
     */
    protected function normalizeRequestOptionValue($value)
    {}

    /**
     * Normalize a scalar to a string without triggering PHP 8.5 non-finite float warnings.
     *
     * @param  scalar  $value
     * @return string
     */
    protected function normalizeScalarString($value): string
    {}

    /**
     * Ensure the given request body can be passed to Guzzle.
     *
     * @param  mixed  $body
     * @return void
     *
     * @throws \InvalidArgumentException
     */
    protected function ensureValidRequestBody($body): void
    {}

    /**
     * Populate the given response with additional data.
     *
     * @param  \Illuminate\Http\Client\Response  $response
     * @return \Illuminate\Http\Client\Response
     */
    protected function populateResponse(Response $response)
    {}

    /**
     * Build the Guzzle client.
     *
     * @return \GuzzleHttp\Client
     */
    public function buildClient()
    {}

    /**
     * Determine if a reusable client is required.
     *
     * @return bool
     */
    protected function requestsReusableClient()
    {}

    /**
     * Retrieve a reusable Guzzle client.
     *
     * @return \GuzzleHttp\Client
     */
    protected function getReusableClient()
    {}

    /**
     * Create new Guzzle client.
     *
     * @param  \GuzzleHttp\HandlerStack  $handlerStack
     * @return \GuzzleHttp\Client
     */
    public function createClient($handlerStack)
    {}

    /**
     * Build the Guzzle client handler stack.
     *
     * @return \GuzzleHttp\HandlerStack
     */
    public function buildHandlerStack()
    {}

    /**
     * Add the necessary handlers to the given handler stack.
     *
     * @param  \GuzzleHttp\HandlerStack  $handlerStack
     * @return \GuzzleHttp\HandlerStack
     */
    public function pushHandlers($handlerStack)
    {}

    /**
     * Build the before sending handler.
     *
     * @return \Closure
     */
    public function buildBeforeSendingHandler()
    {}

    /**
     * Build the recorder handler.
     *
     * @return \Closure
     */
    public function buildRecorderHandler()
    {}

    /**
     * Build the stub handler.
     *
     * @return \Closure
     *
     * @throws \Illuminate\Http\Client\StrayRequestException
     */
    public function buildStubHandler()
    {}

    /**
     * Get the sink stub handler callback.
     *
     * @param  string  $sink
     * @return \Closure
     */
    protected function sinkStubHandler($sink)
    {}

    /**
     * Execute the "before sending" callbacks.
     *
     * @param  \Psr\Http\Message\RequestInterface  $request
     * @param  array  $options
     * @return \Psr\Http\Message\RequestInterface
     */
    public function runBeforeSendingCallbacks($request, array $options)
    {}

    /**
     * Replace the given options with the current request options.
     *
     * @param  array  ...$options
     * @return array
     */
    public function mergeOptions(...$options)
    {}

    /**
     * Create a new response instance using the given PSR response.
     *
     * @param  \Psr\Http\Message\MessageInterface  $response
     * @return Response
     */
    protected function newResponse($response)
    {}

    /**
     * Execute the "after response" callbacks.
     *
     * @param  \Illuminate\Http\Client\Response  $response
     * @return \Illuminate\Http\Client\Response
     */
    protected function runAfterResponseCallbacks(Response $response)
    {}

    /**
     * Register a stub callable that will intercept requests and be able to return stub responses.
     *
     * @param  callable  $callback
     * @return $this
     */
    public function stub($callback)
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
     * Allow stray, unfaked requests entirely, or optionally allow only specific URLs.
     *
     * @param  array<int, string>  $only
     * @return $this
     */
    public function allowStrayRequests(array $only)
    {}

    /**
     * Determine if the given URL is allowed as a stray request.
     *
     * @param  string  $url
     * @return bool
     */
    public function isAllowedRequestUrl($url)
    {}

    /**
     * Toggle asynchronicity in requests.
     *
     * @template T of bool = true
     *
     * @param  T  $async
     * @return self<T>
     *
     * @phpstan-self-out self<T>
     */
    public function async(bool $async = true)
    {}

    /**
     * Retrieve the pending request promise.
     *
     * @return \GuzzleHttp\Promise\PromiseInterface|null
     */
    public function getPromise()
    {}

    /**
     * Dispatch the RequestSending event if a dispatcher is available.
     *
     * @return void
     */
    protected function dispatchRequestSendingEvent()
    {}

    /**
     * Dispatch the ResponseReceived event if a dispatcher is available.
     *
     * @param  \Illuminate\Http\Client\Response  $response
     * @return void
     */
    protected function dispatchResponseReceivedEvent(Response $response)
    {}

    /**
     * Dispatch the ConnectionFailed event if a dispatcher is available.
     *
     * @param  \Illuminate\Http\Client\Request  $request
     * @param  \Illuminate\Http\Client\ConnectionException  $exception
     * @return void
     */
    protected function dispatchConnectionFailedEvent(Request $request, ConnectionException $exception)
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
     * Get the PSR-7 response carried by the given exception, if any.
     *
     * @param  \Throwable  $e
     * @return \Psr\Http\Message\ResponseInterface|null
     */
    protected function responseFromException(Throwable $e)
    {}

    /**
     * Handle the given transport exception.
     *
     * @param  \GuzzleHttp\Exception\TransferException  $e
     * @return void
     *
     * @throws \Illuminate\Http\Client\ConnectionException
     */
    protected function marshalTransportException(TransferException $e)
    {}

    /**
     * Handle the given transport exception that carried a response.
     *
     * @param  \GuzzleHttp\Exception\TransferException  $e
     * @param  \Psr\Http\Message\ResponseInterface  $response
     * @return void
     *
     * @throws \Illuminate\Http\Client\RequestException
     * @throws \Illuminate\Http\Client\ConnectionException
     */
    protected function marshalTransportExceptionWithResponse(TransferException $e, ResponseInterface $response)
    {}

    /**
     * Set the client instance.
     *
     * @param  \GuzzleHttp\Client  $client
     * @return $this
     */
    public function setClient(Client $client)
    {}

    /**
     * Create a new client instance using the given handler.
     *
     * @param  callable  $handler
     * @return $this
     */
    public function setHandler($handler)
    {}

    /**
     * Get the pending request options.
     *
     * @return array
     */
    public function getOptions()
    {}
}
