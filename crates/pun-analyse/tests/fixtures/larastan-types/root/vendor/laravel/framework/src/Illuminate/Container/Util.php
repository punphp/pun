<?php

namespace Illuminate\Container;

use Closure;
use Illuminate\Contracts\Container\ContextualAttribute;
use ReflectionAttribute;
use ReflectionNamedType;

/**
 * @internal
 */
class Util
{
    /**
     * If the given value is not an array and not null, wrap it in one.
     *
     * From Arr::wrap() in Illuminate\Support.
     *
     * @param  mixed  $value
     * @return array
     */
    public static function arrayWrap($value)
    {}

    /**
     * Return the default value of the given value.
     *
     * From global value() helper in Illuminate\Support.
     *
     * @param  mixed  $value
     * @param  mixed  ...$args
     * @return mixed
     */
    public static function unwrapIfClosure($value, ...$args)
    {}

    /**
     * Get the class name of the given parameter's type, if possible.
     *
     * From Reflector::getParameterClassName() in Illuminate\Support.
     *
     * @param  \ReflectionParameter  $parameter
     * @return string|null
     */
    public static function getParameterClassName($parameter)
    {}

    /**
     * Get a contextual attribute from a dependency.
     *
     * @param  \ReflectionParameter  $dependency
     * @return \ReflectionAttribute|null
     */
    public static function getContextualAttributeFromDependency($dependency)
    {}
}
