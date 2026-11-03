<?php

namespace Illuminate\Cookie;

use Illuminate\Contracts\Cookie\QueueingFactory as JarContract;
use Illuminate\Support\Arr;
use Illuminate\Support\InteractsWithTime;
use Illuminate\Support\Traits\Macroable;
use Symfony\Component\HttpFoundation\Cookie;

class CookieJar implements JarContract
{
    use InteractsWithTime, Macroable;

    /**
     * The default path (if specified).
     *
     * @var string
     */
    protected $path = '/';

    /**
     * The default domain (if specified).
     *
     * @var string|null
     */
    protected $domain;

    /**
     * The default secure setting (defaults to null).
     *
     * @var bool|null
     */
    protected $secure;

    /**
     * The default SameSite option (defaults to lax).
     *
     * @var string|null
     */
    protected $sameSite = 'lax';

    /**
     * All of the cookies queued for sending, keyed by name and then by path.
     *
     * @var array<string, array<string, \Symfony\Component\HttpFoundation\Cookie>>
     */
    protected $queued = [];

    /**
     * Create a new cookie instance.
     *
     * @param  string  $name
     * @param  string  $value
     * @param  int  $minutes
     * @param  string|null  $path
     * @param  string|null  $domain
     * @param  bool|null  $secure
     * @param  bool  $httpOnly
     * @param  bool  $raw
     * @param  string|null  $sameSite
     * @return \Symfony\Component\HttpFoundation\Cookie
     */
    public function make($name, $value, $minutes = 0, $path = null, $domain = null, $secure = null, $httpOnly = true, $raw = false, $sameSite = null)
    {}

    /**
     * Create a cookie that lasts "forever" (400 days).
     *
     * @param  string  $name
     * @param  string  $value
     * @param  string|null  $path
     * @param  string|null  $domain
     * @param  bool|null  $secure
     * @param  bool  $httpOnly
     * @param  bool  $raw
     * @param  string|null  $sameSite
     * @return \Symfony\Component\HttpFoundation\Cookie
     */
    public function forever($name, $value, $path = null, $domain = null, $secure = null, $httpOnly = true, $raw = false, $sameSite = null)
    {}

    /**
     * Expire the given cookie.
     *
     * @param  string  $name
     * @param  string|null  $path
     * @param  string|null  $domain
     * @return \Symfony\Component\HttpFoundation\Cookie
     */
    public function forget($name, $path = null, $domain = null)
    {}

    /**
     * Determine if a cookie has been queued.
     *
     * @param  string  $key
     * @param  string|null  $path
     * @return bool
     */
    public function hasQueued($key, $path = null)
    {}

    /**
     * Get a queued cookie instance.
     *
     * @template TQueuedDefault
     *
     * @param  string  $key
     * @param  TQueuedDefault|(\Closure(): TQueuedDefault)  $default
     * @param  string|null  $path
     * @return \Symfony\Component\HttpFoundation\Cookie|TQueuedDefault
     */
    public function queued($key, $default = null, $path = null)
    {}

    /**
     * Queue a cookie to send with the next response.
     *
     * @param  mixed  ...$parameters
     * @return void
     */
    public function queue(...$parameters)
    {}

    /**
     * Queue a cookie to expire with the next response.
     *
     * @param  string  $name
     * @param  string|null  $path
     * @param  string|null  $domain
     * @return void
     */
    public function expire($name, $path = null, $domain = null)
    {}

    /**
     * Remove a cookie from the queue.
     *
     * @param  string  $name
     * @param  string|null  $path
     * @return void
     */
    public function unqueue($name, $path = null)
    {}

    /**
     * Get the path and domain, or the default values.
     *
     * @param  string  $path
     * @param  string|null  $domain
     * @param  bool|null  $secure
     * @param  string|null  $sameSite
     * @return array
     */
    protected function getPathAndDomain($path, $domain, $secure = null, $sameSite = null)
    {}

    /**
     * Set the default path and domain for the jar.
     *
     * @param  string  $path
     * @param  string|null  $domain
     * @param  bool|null  $secure
     * @param  string|null  $sameSite
     * @return $this
     */
    public function setDefaultPathAndDomain($path, $domain, $secure = false, $sameSite = null)
    {}

    /**
     * Get the cookies which have been queued for the next request.
     *
     * @return \Symfony\Component\HttpFoundation\Cookie[]
     */
    public function getQueuedCookies()
    {}

    /**
     * Flush the cookies which have been queued for the next request.
     *
     * @return $this
     */
    public function flushQueuedCookies()
    {}
}
