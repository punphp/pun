<?php

namespace Illuminate\Cache\RateLimiting;

class Limit
{
    /**
     * The rate limit signature key.
     *
     * @var mixed
     */
    public $key;

    /**
     * The maximum number of attempts allowed within the given number of seconds.
     *
     * @var int
     */
    public $maxAttempts;

    /**
     * The number of seconds until the rate limit is reset.
     *
     * @var int
     */
    public $decaySeconds;

    /**
     * The after callback used to determine if the limiter should be hit.
     *
     * @var ?callable
     */
    public $afterCallback = null;

    /**
     * The response generator callback.
     *
     * @var callable
     */
    public $responseCallback;

    /**
     * Create a new limit instance.
     *
     * @param  mixed  $key
     * @param  int  $maxAttempts
     * @param  int  $decaySeconds
     */
    public function __construct($key = '', int $maxAttempts = 60, int $decaySeconds = 60)
    {}

    /**
     * Create a new rate limit.
     *
     * @param  int  $maxAttempts
     * @param  int  $decaySeconds
     * @return static
     */
    public static function perSecond($maxAttempts, $decaySeconds = 1)
    {}

    /**
     * Create a new rate limit.
     *
     * @param  int  $maxAttempts
     * @param  int  $decayMinutes
     * @return static
     */
    public static function perMinute($maxAttempts, $decayMinutes = 1)
    {}

    /**
     * Create a new rate limit using minutes as decay time.
     *
     * @param  int  $decayMinutes
     * @param  int  $maxAttempts
     * @return static
     */
    public static function perMinutes($decayMinutes, $maxAttempts)
    {}

    /**
     * Create a new rate limit using hours as decay time.
     *
     * @param  int  $maxAttempts
     * @param  int  $decayHours
     * @return static
     */
    public static function perHour($maxAttempts, $decayHours = 1)
    {}

    /**
     * Create a new rate limit using days as decay time.
     *
     * @param  int  $maxAttempts
     * @param  int  $decayDays
     * @return static
     */
    public static function perDay($maxAttempts, $decayDays = 1)
    {}

    /**
     * Create a new unlimited rate limit.
     *
     * @return \Illuminate\Cache\RateLimiting\Unlimited
     */
    public static function none()
    {}

    /**
     * Set the key of the rate limit.
     *
     * @param  mixed  $key
     * @return $this
     */
    public function by($key)
    {}

    /**
     * Set the callback to determine if the limiter should be hit.
     *
     * @param  callable  $callback
     * @return $this
     */
    public function after($callback)
    {}

    /**
     * Set the callback that should generate the response when the limit is exceeded.
     *
     * @param  callable  $callback
     * @return $this
     */
    public function response(callable $callback)
    {}

    /**
     * Get a potential fallback key for the limit.
     *
     * @return string
     */
    public function fallbackKey()
    {}
}
