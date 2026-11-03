<?php

namespace Illuminate\Queue;

use Aws\Credentials\CredentialsInterface;
use Closure;
use GuzzleHttp\Promise\Create;
use GuzzleHttp\Promise\RejectedPromise;
use Illuminate\Contracts\Cache\Lock;
use Illuminate\Contracts\Cache\LockProvider;
use Throwable;

class AwsCredentialCache
{
    /**
     * The number of seconds before expiration that credentials should be refreshed.
     *
     * @var int
     */
    protected const REFRESH_WINDOW = 60;

    /**
     * The number of seconds the credential refresh lock should be maintained.
     *
     * @var int
     */
    protected const LOCK_SECONDS = 15;

    /**
     * The number of seconds to wait for another process to refresh credentials.
     *
     * @var int
     */
    protected const LOCK_WAIT_SECONDS = 5;

    /**
     * The resolver for the backing cache repository.
     *
     * @var \Closure(): \Illuminate\Contracts\Cache\Repository
     */
    protected $repository;

    /**
     * The resolver for the fallback cache repository, if any.
     *
     * @var \Closure(): \Illuminate\Contracts\Cache\Repository|null
     */
    protected $fallback;

    /**
     * Create a new AWS credential cache.
     *
     * @param  \Closure(): \Illuminate\Contracts\Cache\Repository  $repository
     * @param  \Closure(): \Illuminate\Contracts\Cache\Repository|null  $fallback
     */
    public function __construct(Closure $repository, ?Closure $fallback = null)
    {}

    /**
     * Resolve credentials while sharing a single refresh across processes.
     *
     * @param  string  $key
     * @param  callable  $provider
     * @return \GuzzleHttp\Promise\PromiseInterface
     */
    public function resolve($key, callable $provider)
    {}

    /**
     * Get fresh credentials from the available cache repositories.
     *
     * @param  string  $key
     * @return \Aws\Credentials\CredentialsInterface|null
     */
    protected function freshCredentials($key)
    {}

    /**
     * Resolve and cache credentials before releasing the refresh lock.
     *
     * @param  string  $key
     * @param  callable  $provider
     * @param  \Illuminate\Contracts\Cache\Lock|null  $lock
     * @return \GuzzleHttp\Promise\PromiseInterface
     */
    protected function resolveAndCache($key, callable $provider, ?Lock $lock = null)
    {}

    /**
     * Store the credentials, silently ignoring unavailable cache stores.
     *
     * @param  string  $key
     * @param  \Aws\Credentials\CredentialsInterface  $credentials
     * @param  int  $ttl
     * @return void
     */
    protected function put($key, CredentialsInterface $credentials, $ttl = 0)
    {}

    /**
     * Remove the cached credentials, silently ignoring unavailable cache stores.
     *
     * @param  string  $key
     * @return void
     */
    protected function forget($key)
    {}

    /**
     * Get the cache repository resolvers in order of preference.
     *
     * @return array<int, \Closure(): \Illuminate\Contracts\Cache\Repository>
     */
    protected function repositories()
    {}

    /**
     * Release the credential refresh lock without affecting credential resolution.
     *
     * @param  \Illuminate\Contracts\Cache\Lock|null  $lock
     * @return void
     */
    protected function release(?Lock $lock)
    {}
}
