<?php

namespace Illuminate\Container;

use Closure;
use Illuminate\Contracts\Container\BindingResolutionException;
use InvalidArgumentException;
use ReflectionFunction;
use ReflectionMethod;

class BoundMethod
{
    /**
     * Call the given Closure / class@method and inject its dependencies.
     *
     * @param  \Illuminate\Container\Container  $container
     * @param  callable|string  $callback
     * @param  array  $parameters
     * @param  string|null  $defaultMethod
     * @return mixed
     *
     * @throws \ReflectionException
     * @throws \InvalidArgumentException
     */
    public static function call($container, $callback, array $parameters = [], $defaultMethod = null)
    {}

    /**
     * Call a string reference to a class using Class@method syntax.
     *
     * @param  \Illuminate\Container\Container  $container
     * @param  string  $target
     * @param  array  $parameters
     * @param  string|null  $defaultMethod
     * @return mixed
     *
     * @throws \InvalidArgumentException
     */
    protected static function callClass($container, $target, array $parameters = [], $defaultMethod = null)
    {}

    /**
     * Call a method that has been bound to the container.
     *
     * @param  \Illuminate\Container\Container  $container
     * @param  callable  $callback
     * @param  mixed  $default
     * @return mixed
     */
    protected static function callBoundMethod($container, $callback, $default)
    {}

    /**
     * Normalize the given callback into a Class@method string.
     *
     * @param  callable  $callback
     * @return string
     */
    protected static function normalizeMethod($callback)
    {}

    /**
     * Get all dependencies for a given method.
     *
     * @param  \Illuminate\Container\Container  $container
     * @param  callable|string  $callback
     * @param  array  $parameters
     * @return array
     *
     * @throws \ReflectionException
     */
    protected static function getMethodDependencies($container, $callback, array $parameters = [])
    {}

    /**
     * Get the proper reflection instance for the given callback.
     *
     * @param  callable|string  $callback
     * @return \ReflectionFunctionAbstract
     *
     * @throws \ReflectionException
     */
    protected static function getCallReflector($callback)
    {}

    /**
     * Get the dependency for the given call parameter.
     *
     * @param  \Illuminate\Container\Container  $container
     * @param  \ReflectionParameter  $parameter
     * @param  array  $parameters
     * @param  array  $dependencies
     * @return void
     *
     * @throws \Illuminate\Contracts\Container\BindingResolutionException
     */
    protected static function addDependencyForCallParameter(
        $container,
        $parameter,
        array &$parameters,
        &$dependencies,
    ) {}

    /**
     * Determine if the given string is in Class@method syntax.
     *
     * @param  mixed  $callback
     * @return bool
     */
    protected static function isCallableWithAtSign($callback)
    {}
}
