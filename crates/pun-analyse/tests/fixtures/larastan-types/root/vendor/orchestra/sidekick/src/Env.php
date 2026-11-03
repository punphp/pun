<?php

namespace Orchestra\Sidekick;

/**
 * @api
 */
class Env extends \Illuminate\Support\Env
{
    /**
     * Determine if environmemt variable is available.
     */
    public static function has(string $key): bool
    {}

    /**
     * Set an environment value.
     */
    public static function set(string $key, string $value): void
    {}

    /**
     * Forget an environment variable.
     *
     *
     * @throws \InvalidArgumentException
     */
    public static function forget(string $key): bool
    {}

    /**
     * Forward environment value.
     *
     * @param  \Orchestra\Sidekick\UndefinedValue|mixed|null  $default
     * @return mixed
     */
    public static function forward(string $key, $default = new UndefinedValue)
    {}

    /**
     * Encode environment variable value.
     *
     * @param  mixed  $value
     * @return mixed
     */
    public static function encode($value)
    {}
}
