<?php

namespace Illuminate\Auth\Passwords;

use Closure;
use Illuminate\Auth\Events\PasswordResetLinkSent;
use Illuminate\Contracts\Auth\CanResetPassword as CanResetPasswordContract;
use Illuminate\Contracts\Auth\PasswordBroker as PasswordBrokerContract;
use Illuminate\Contracts\Auth\UserProvider;
use Illuminate\Contracts\Events\Dispatcher;
use Illuminate\Support\Arr;
use Illuminate\Support\Timebox;
use UnexpectedValueException;

class PasswordBroker implements PasswordBrokerContract
{
    /**
     * The password token repository.
     *
     * @var \Illuminate\Auth\Passwords\TokenRepositoryInterface
     */
    protected $tokens;

    /**
     * The user provider implementation.
     *
     * @var \Illuminate\Contracts\Auth\UserProvider
     */
    protected $users;

    /**
     * The event dispatcher instance.
     *
     * @var \Illuminate\Contracts\Events\Dispatcher|null
     */
    protected $events;

    /**
     * The timebox instance.
     *
     * @var \Illuminate\Support\Timebox
     */
    protected $timebox;

    /**
     * The number of microseconds that the timebox should wait for.
     *
     * @var int
     */
    protected $timeboxDuration;

    /**
     * Create a new password broker instance.
     *
     * @param  \Illuminate\Auth\Passwords\TokenRepositoryInterface  $tokens
     * @param  \Illuminate\Contracts\Auth\UserProvider  $users
     * @param  \Illuminate\Contracts\Events\Dispatcher|null  $dispatcher
     * @param  \Illuminate\Support\Timebox|null  $timebox
     * @param  int  $timeboxDuration
     */
    public function __construct(
        #[\SensitiveParameter] TokenRepositoryInterface $tokens,
        UserProvider $users,
        ?Dispatcher $dispatcher = null,
        ?Timebox $timebox = null,
        int $timeboxDuration = 200000,
    ) {}

    /**
     * Send a password reset link to a user.
     *
     * @param  array  $credentials
     * @param  \Closure|null  $callback
     * @return string
     */
    public function sendResetLink(#[\SensitiveParameter] array $credentials, ?Closure $callback = null)
    {}

    /**
     * Reset the password for the given token.
     *
     * @param  array  $credentials
     * @param  \Closure  $callback
     * @return string
     */
    public function reset(#[\SensitiveParameter] array $credentials, Closure $callback)
    {}

    /**
     * Validate a password reset for the given credentials.
     *
     * @param  array  $credentials
     * @return \Illuminate\Contracts\Auth\CanResetPassword|string
     */
    protected function validateReset(#[\SensitiveParameter] array $credentials)
    {}

    /**
     * Get the user for the given credentials.
     *
     * @param  array  $credentials
     * @return \Illuminate\Contracts\Auth\CanResetPassword|null
     *
     * @throws \UnexpectedValueException
     */
    public function getUser(#[\SensitiveParameter] array $credentials)
    {}

    /**
     * Create a new password reset token for the given user.
     *
     * @param  \Illuminate\Contracts\Auth\CanResetPassword  $user
     * @return string
     */
    public function createToken(CanResetPasswordContract $user)
    {}

    /**
     * Delete password reset tokens of the given user.
     *
     * @param  \Illuminate\Contracts\Auth\CanResetPassword  $user
     * @return void
     */
    public function deleteToken(CanResetPasswordContract $user)
    {}

    /**
     * Validate the given password reset token.
     *
     * @param  \Illuminate\Contracts\Auth\CanResetPassword  $user
     * @param  string  $token
     * @return bool
     */
    public function tokenExists(CanResetPasswordContract $user, #[\SensitiveParameter] $token)
    {}

    /**
     * Get the password reset token repository implementation.
     *
     * @return \Illuminate\Auth\Passwords\TokenRepositoryInterface
     */
    public function getRepository()
    {}

    /**
     * Get the timebox instance used by the guard.
     *
     * @return \Illuminate\Support\Timebox
     */
    public function getTimebox()
    {}
}
