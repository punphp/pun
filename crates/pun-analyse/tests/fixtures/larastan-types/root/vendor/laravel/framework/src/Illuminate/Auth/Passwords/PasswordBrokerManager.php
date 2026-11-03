<?php

namespace Illuminate\Auth\Passwords;

use Illuminate\Contracts\Auth\PasswordBrokerFactory as FactoryContract;
use InvalidArgumentException;

use function Illuminate\Support\enum_value;

/**
 * @mixin \Illuminate\Contracts\Auth\PasswordBroker
 */
class PasswordBrokerManager implements FactoryContract
{
    /**
     * The application instance.
     *
     * @var \Illuminate\Contracts\Foundation\Application
     */
    protected $app;

    /**
     * The array of created "drivers".
     *
     * @var array
     */
    protected $brokers = [];

    /**
     * Create a new PasswordBroker manager instance.
     *
     * @param  \Illuminate\Contracts\Foundation\Application  $app
     */
    public function __construct($app)
    {}

    /**
     * Attempt to get the broker from the local cache.
     *
     * @param  \UnitEnum|string|null  $name
     * @return \Illuminate\Contracts\Auth\PasswordBroker
     */
    public function broker($name = null)
    {}

    /**
     * Resolve the given broker.
     *
     * @param  string  $name
     * @return \Illuminate\Contracts\Auth\PasswordBroker
     *
     * @throws \InvalidArgumentException
     */
    protected function resolve($name)
    {}

    /**
     * Create a token repository instance based on the given configuration.
     *
     * @param  array  $config
     * @return \Illuminate\Auth\Passwords\TokenRepositoryInterface
     */
    protected function createTokenRepository(array $config)
    {}

    /**
     * Get the password broker configuration.
     *
     * @param  string  $name
     * @return array|null
     */
    protected function getConfig($name)
    {}

    /**
     * Get the default password broker name.
     *
     * @return string
     */
    public function getDefaultDriver()
    {}

    /**
     * Set the default password broker name.
     *
     * @param  \UnitEnum|string  $name
     * @return void
     */
    public function setDefaultDriver($name)
    {}

    /**
     * Dynamically call the default driver instance.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return mixed
     */
    public function __call($method, $parameters)
    {}
}
