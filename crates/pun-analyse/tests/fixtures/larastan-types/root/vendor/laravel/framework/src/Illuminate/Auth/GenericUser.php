<?php

namespace Illuminate\Auth;

use Illuminate\Contracts\Auth\Authenticatable as UserContract;

class GenericUser implements UserContract
{
    /**
     * All of the user's attributes.
     *
     * @var array
     */
    protected $attributes;

    /**
     * Create a new generic User object.
     *
     * @param  array  $attributes
     */
    public function __construct(array $attributes)
    {}

    /**
     * Get the name of the unique identifier for the user.
     *
     * @return string
     */
    public function getAuthIdentifierName()
    {}

    /**
     * Get the unique identifier for the user.
     *
     * @return mixed
     */
    public function getAuthIdentifier()
    {}

    /**
     * Get the name of the password attribute for the user.
     *
     * @return string
     */
    public function getAuthPasswordName()
    {}

    /**
     * Get the password for the user.
     *
     * @return string|null
     */
    public function getAuthPassword()
    {}

    /**
     * Get the "remember me" token value.
     *
     * @return string
     */
    public function getRememberToken()
    {}

    /**
     * Set the "remember me" token value.
     *
     * @param  string|null  $value
     * @return void
     */
    public function setRememberToken($value)
    {}

    /**
     * Get the column name for the "remember me" token.
     *
     * @return string
     */
    public function getRememberTokenName()
    {}

    /**
     * Dynamically access the user's attributes.
     *
     * @param  string  $key
     * @return mixed
     */
    public function __get($key)
    {}

    /**
     * Dynamically set an attribute on the user.
     *
     * @param  string  $key
     * @param  mixed  $value
     * @return void
     */
    public function __set($key, $value)
    {}

    /**
     * Dynamically check if a value is set on the user.
     *
     * @param  string  $key
     * @return bool
     */
    public function __isset($key)
    {}

    /**
     * Dynamically unset a value on the user.
     *
     * @param  string  $key
     * @return void
     */
    public function __unset($key)
    {}
}
