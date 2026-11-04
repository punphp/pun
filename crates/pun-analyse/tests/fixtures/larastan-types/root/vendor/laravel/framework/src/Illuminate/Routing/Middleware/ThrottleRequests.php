<?php

namespace Illuminate\Routing\Middleware;

use Closure;
use Illuminate\Cache\RateLimiter;
use Illuminate\Cache\RateLimiting\Unlimited;
use Illuminate\Http\Exceptions\HttpResponseException;
use Illuminate\Http\Exceptions\ThrottleRequestsException;
use Illuminate\Routing\Exceptions\MissingRateLimiterException;
use Illuminate\Support\Collection;
use Illuminate\Support\InteractsWithTime;
use RuntimeException;
use Symfony\Component\HttpFoundation\Response;

use function Illuminate\Support\enum_value;

class ThrottleRequests
{
    use InteractsWithTime;

    /**
     * The rate limiter instance.
     *
     * @var \Illuminate\Cache\RateLimiter
     */
    protected $limiter;

    /**
     * Indicates if the rate limiter keys should be hashed.
     *
     * @var bool
     */
    protected static $shouldHashKeys = true;

    /**
     * Create a new request throttler.
     *
     * @param  \Illuminate\Cache\RateLimiter  $limiter
     */
    public function __construct(RateLimiter $limiter)
    {}

    /**
     * Specify the named rate limiter to use for the middleware.
     *
     * @param  \UnitEnum|string  $name
     * @return string
     */
    public static function using($name)
    {}

    /**
     * Specify the rate limiter configuration for the middleware.
     *
     * @param  int  $maxAttempts
     * @param  int  $decayMinutes
     * @param  string  $prefix
     * @return string
     *
     * @named-arguments-supported
     */
    public static function with($maxAttempts = 60, $decayMinutes = 1, $prefix = '')
    { func_get_args(); }

    /**
     * Handle an incoming request.
     *
     * @param  \Illuminate\Http\Request  $request
     * @param  \Closure  $next
     * @param  int|string  $maxAttempts
     * @param  float|int  $decayMinutes
     * @param  string  $prefix
     * @return \Symfony\Component\HttpFoundation\Response
     *
     * @throws \Illuminate\Http\Exceptions\ThrottleRequestsException
     * @throws \Illuminate\Routing\Exceptions\MissingRateLimiterException
     */
    public function handle($request, Closure $next, $maxAttempts = 60, $decayMinutes = 1, $prefix = '')
    { func_get_args(); }

    /**
     * Handle an incoming request.
     *
     * @param  \Illuminate\Http\Request  $request
     * @param  \Closure  $next
     * @param  string  $limiterName
     * @param  \Closure  $limiter
     * @return \Symfony\Component\HttpFoundation\Response
     *
     * @throws \Illuminate\Http\Exceptions\ThrottleRequestsException
     */
    protected function handleRequestUsingNamedLimiter($request, Closure $next, $limiterName, Closure $limiter)
    {}

    /**
     * Handle an incoming request.
     *
     * @param  \Illuminate\Http\Request  $request
     * @param  \Closure  $next
     * @param  array  $limits
     * @return \Symfony\Component\HttpFoundation\Response
     *
     * @throws \Illuminate\Http\Exceptions\ThrottleRequestsException
     */
    protected function handleRequest($request, Closure $next, array $limits)
    {}

    /**
     * Resolve the number of attempts if the user is authenticated or not.
     *
     * @param  \Illuminate\Http\Request  $request
     * @param  int|string  $maxAttempts
     * @return int
     *
     * @throws \Illuminate\Routing\Exceptions\MissingRateLimiterException
     */
    protected function resolveMaxAttempts($request, $maxAttempts)
    {}

    /**
     * Resolve request signature.
     *
     * @param  \Illuminate\Http\Request  $request
     * @return string
     *
     * @throws \RuntimeException
     */
    protected function resolveRequestSignature($request)
    {}

    /**
     * Create a 'too many attempts' exception.
     *
     * @param  \Illuminate\Http\Request  $request
     * @param  string  $key
     * @param  int  $maxAttempts
     * @param  callable|null  $responseCallback
     * @return \Illuminate\Http\Exceptions\ThrottleRequestsException|\Illuminate\Http\Exceptions\HttpResponseException
     */
    protected function buildException($request, $key, $maxAttempts, $responseCallback = null)
    {}

    /**
     * Get the number of seconds until the next retry.
     *
     * @param  string  $key
     * @return int
     */
    protected function getTimeUntilNextRetry($key)
    {}

    /**
     * Add the limit header information to the given response.
     *
     * @param  \Symfony\Component\HttpFoundation\Response  $response
     * @param  int  $maxAttempts
     * @param  int  $remainingAttempts
     * @param  int|null  $retryAfter
     * @return \Symfony\Component\HttpFoundation\Response
     */
    protected function addHeaders(Response $response, $maxAttempts, $remainingAttempts, $retryAfter = null)
    {}

    /**
     * Get the limit headers information.
     *
     * @param  int  $maxAttempts
     * @param  int  $remainingAttempts
     * @param  int|null  $retryAfter
     * @param  \Symfony\Component\HttpFoundation\Response|null  $response
     * @return array
     */
    protected function getHeaders($maxAttempts,
        $remainingAttempts,
        $retryAfter = null,
        ?Response $response = null)
    {}

    /**
     * Calculate the number of remaining attempts.
     *
     * @param  string  $key
     * @param  int  $maxAttempts
     * @param  int|null  $retryAfter
     * @return int
     */
    protected function calculateRemainingAttempts($key, $maxAttempts, $retryAfter = null)
    {}

    /**
     * Format the given identifier based on the configured hashing settings.
     *
     * @param  string  $value
     * @return string
     */
    private function formatIdentifier($value)
    {}

    /**
     * Specify whether rate limiter keys should be hashed.
     *
     * @param  bool  $shouldHashKeys
     * @return void
     */
    public static function shouldHashKeys(bool $shouldHashKeys = true)
    {}
}
