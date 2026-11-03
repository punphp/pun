<?php

namespace Illuminate\Auth\Passwords;

use Illuminate\Cache\Repository;
use Illuminate\Contracts\Auth\CanResetPassword as CanResetPasswordContract;
use Illuminate\Contracts\Hashing\Hasher as HasherContract;
use Illuminate\Support\Carbon;
use Illuminate\Support\Str;

class CacheTokenRepository implements TokenRepositoryInterface
{
    /**
     * The format of the stored Carbon object.
     */
    protected string $format = 'Y-m-d H:i:s';

    /**
     * Create a new token repository instance.
     */
    public function __construct(
        protected Repository $cache,
        protected HasherContract $hasher,
        #[\SensitiveParameter]
        protected string $hashKey,
        protected int $expires = 3600,
        protected int $throttle = 60,
    ) {}

    /**
     * Create a new token.
     *
     * @param  \Illuminate\Contracts\Auth\CanResetPassword  $user
     * @return string
     */
    public function create(CanResetPasswordContract $user)
    {}

    /**
     * Determine if a token record exists and is valid.
     *
     * @param  \Illuminate\Contracts\Auth\CanResetPassword  $user
     * @param  string  $token
     * @return bool
     */
    public function exists(CanResetPasswordContract $user, #[\SensitiveParameter] $token)
    {}

    /**
     * Determine if the token has expired.
     *
     * @param  string  $createdAt
     * @return bool
     */
    protected function tokenExpired($createdAt)
    {}

    /**
     * Determine if the given user recently created a password reset token.
     *
     * @param  \Illuminate\Contracts\Auth\CanResetPassword  $user
     * @return bool
     */
    public function recentlyCreatedToken(CanResetPasswordContract $user)
    {}

    /**
     * Determine if the token was recently created.
     *
     * @param  string  $createdAt
     * @return bool
     */
    protected function tokenRecentlyCreated($createdAt)
    {}

    /**
     * Delete a token record.
     *
     * @param  \Illuminate\Contracts\Auth\CanResetPassword  $user
     * @return void
     */
    public function delete(CanResetPasswordContract $user)
    {}

    /**
     * Delete expired tokens.
     *
     * @return void
     */
    public function deleteExpired()
    {}

    /**
     * Determine the cache key for the given user.
     *
     * @param  \Illuminate\Contracts\Auth\CanResetPassword  $user
     * @return string
     */
    public function cacheKey(CanResetPasswordContract $user): string
    {}
}
