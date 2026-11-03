<?php

declare(strict_types=1);

namespace GuzzleHttp;

use GuzzleHttp\Cookie\CookieJarInterface;
use GuzzleHttp\Exception\RequestException;
use GuzzleHttp\Exception\ResponseException;
use GuzzleHttp\Promise as P;
use GuzzleHttp\Promise\PromiseInterface;
use Psr\Http\Message\RequestInterface;
use Psr\Http\Message\ResponseInterface;
use Psr\Log\LoggerInterface;

/**
 * Functions used to create and wrap handlers with handler middleware.
 */
final class Middleware
{
    private function __construct()
    {}

    /**
     * Middleware that applies built-in Basic authentication and handles Digest
     * authentication challenges when the "auth" request option is set.
     *
     * @param bool $reuseChallenges Whether Digest challenges may be reused preemptively.
     *
     * @return callable((callable(RequestInterface, array<array-key, mixed>): PromiseInterface<ResponseInterface, mixed>)): (callable(RequestInterface, array<array-key, mixed>): PromiseInterface<ResponseInterface, mixed>)
     */
    public static function auth(bool $reuseChallenges = true): callable
    {}

    /**
     * Middleware that adds cookies to requests.
     *
     * The options array must be set to a CookieJarInterface in order to use
     * cookies. This is typically handled for you by a client.
     *
     * @return callable((callable(RequestInterface, array<array-key, mixed>): PromiseInterface<ResponseInterface, mixed>)): (callable(RequestInterface, array<array-key, mixed>): PromiseInterface<ResponseInterface, mixed>)
     */
    public static function cookies(): callable
    {}

    /**
     * Middleware that throws exceptions for 4xx or 5xx responses when the
     * "http_errors" request option is set to true.
     *
     * @param BodySummarizerInterface|null $bodySummarizer The body summarizer to use in exception messages.
     *
     * @return callable((callable(RequestInterface, array<array-key, mixed>): PromiseInterface<ResponseInterface, mixed>)): (callable(RequestInterface, array<array-key, mixed>): PromiseInterface<ResponseInterface, mixed>)
     */
    public static function httpErrors(?BodySummarizerInterface $bodySummarizer = null): callable
    {}

    /**
     * Middleware that pushes history data to an ArrayAccess container.
     *
     * @param array<array-key, array{request: RequestInterface, response: ResponseInterface|null, error: mixed, options: array<array-key, mixed>}>|\ArrayAccess<int, array{request: RequestInterface, response: ResponseInterface|null, error: mixed, options: array<array-key, mixed>}> $container Container to hold the history (by reference).
     *
     * @return callable((callable(RequestInterface, array<array-key, mixed>): PromiseInterface<ResponseInterface, mixed>)): (callable(RequestInterface, array<array-key, mixed>): PromiseInterface<ResponseInterface, mixed>)
     *
     * @throws \InvalidArgumentException if container is not an array or ArrayAccess.
     */
    public static function history(&$container): callable
    {}

    /**
     * Middleware that observes requests and responses as they flow through the
     * stack without modifying them. This is useful for metrics, tracing, and
     * debugging.
     *
     * The provided listener cannot modify or alter the response. It simply
     * "taps" into the chain to be notified before returning the promise. The
     * before listener accepts a request and options array, and the after
     * listener accepts a request, options array, and response promise.
     *
     * @param (callable(RequestInterface, array<array-key, mixed>): mixed)|null                                             $before Function to invoke before forwarding the request.
     * @param (callable(RequestInterface, array<array-key, mixed>, PromiseInterface<ResponseInterface, mixed>): mixed)|null $after  Function invoked after forwarding.
     *
     * @return callable((callable(RequestInterface, array<array-key, mixed>): PromiseInterface<ResponseInterface, mixed>)): (callable(RequestInterface, array<array-key, mixed>): PromiseInterface<ResponseInterface, mixed>)
     */
    public static function tap(?callable $before = null, ?callable $after = null): callable
    {}

    /**
     * Middleware that handles request redirects.
     *
     * @return callable((callable(RequestInterface, array<array-key, mixed>): PromiseInterface<ResponseInterface, mixed>)): (callable(RequestInterface, array<array-key, mixed>): PromiseInterface<ResponseInterface, mixed>)
     */
    public static function redirect(): callable
    {}

    /**
     * Middleware that retries requests based on the boolean result of
     * invoking the provided "decider" function.
     *
     * If no delay function is provided, a simple implementation of exponential
     * backoff will be utilized.
     *
     * @param callable(int, RequestInterface, ResponseInterface|null, mixed): bool $decider Function that accepts the number of retries,
     *                                                                                      a request, [response], and [rejection reason]
     *                                                                                      and returns true if the request is to be retried.
     * @param (callable(int, ResponseInterface|null, RequestInterface): int)|null  $delay   Function that accepts the number of retries,
     *                                                                                      [response], and request, and returns the
     *                                                                                      number of milliseconds to delay.
     *
     * @return callable((callable(RequestInterface, array<array-key, mixed>): PromiseInterface<ResponseInterface, mixed>)): (callable(RequestInterface, array<array-key, mixed>): PromiseInterface<ResponseInterface, mixed>)
     */
    public static function retry(callable $decider, ?callable $delay = null): callable
    {}

    /**
     * Middleware that logs requests, responses, and errors using a message
     * formatter.
     *
     * @param LoggerInterface           $logger    Logs messages.
     * @param MessageFormatterInterface $formatter Formatter used to create message strings.
     * @param string                    $logLevel  Level at which to log requests.
     *
     * @return callable((callable(RequestInterface, array<array-key, mixed>): PromiseInterface<ResponseInterface, mixed>)): (callable(RequestInterface, array<array-key, mixed>): PromiseInterface<ResponseInterface, mixed>)
     */
    public static function log(LoggerInterface $logger, MessageFormatterInterface $formatter, string $logLevel = 'info'): callable
    {}

    /**
     * This middleware adds a default content-type if possible, a default
     * content-length or transfer-encoding header, and the expect header.
     *
     * @return callable((callable(RequestInterface, array<array-key, mixed>): PromiseInterface<ResponseInterface, mixed>)): (callable(RequestInterface, array<array-key, mixed>): PromiseInterface<ResponseInterface, mixed>)
     */
    public static function prepareBody(): callable
    {}

    /**
     * Middleware that applies a map function to the request before passing to
     * the next handler.
     *
     * @param callable(RequestInterface): RequestInterface $fn Function that accepts a RequestInterface and returns
     *                                                         a RequestInterface.
     *
     * @return callable((callable(RequestInterface, array<array-key, mixed>): PromiseInterface<ResponseInterface, mixed>)): (callable(RequestInterface, array<array-key, mixed>): PromiseInterface<ResponseInterface, mixed>)
     */
    public static function mapRequest(callable $fn): callable
    {}

    /**
     * Middleware that applies a map function to the resolved promise's
     * response.
     *
     * @param callable(ResponseInterface): ResponseInterface $fn Function that accepts a ResponseInterface and
     *                                                           returns a ResponseInterface.
     *
     * @return callable((callable(RequestInterface, array<array-key, mixed>): PromiseInterface<ResponseInterface, mixed>)): (callable(RequestInterface, array<array-key, mixed>): PromiseInterface<ResponseInterface, mixed>)
     */
    public static function mapResponse(callable $fn): callable
    {}
}
