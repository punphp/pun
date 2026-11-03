<?php

namespace Illuminate\Support;

use ReflectionAttribute;
use ReflectionClass;
use ReflectionEnum;
use ReflectionMethod;
use ReflectionNamedType;
use ReflectionUnionType;

class Reflector
{
    /**
     * This is a PHP 7.4 compatible implementation of is_callable.
     *
     * @param  mixed  $var
     * @param  bool  $syntaxOnly
     * @return bool
     */
    public static function isCallable($var, $syntaxOnly = false)
    {}

    /**
     * Get the specified class attribute, optionally following an inheritance chain.
     *
     * @template TAttribute of object
     *
     * @param  object|class-string  $objectOrClass
     * @param  class-string<TAttribute>  $attribute
     * @param  bool  $ascend
     * @return TAttribute|null
     */
    public static function getClassAttribute($objectOrClass, $attribute, $ascend = false)
    {}

    /**
     * Get the specified class attribute(s), optionally following an inheritance chain.
     *
     * @template TTarget of object
     * @template TAttribute of object
     *
     * @param  TTarget|class-string<TTarget>  $objectOrClass
     * @param  class-string<TAttribute>  $attribute
     * @param  bool  $includeParents
     * @return ($includeParents is true ? Collection<class-string<contravariant TTarget>, Collection<int, TAttribute>> : Collection<int, TAttribute>)
     *
     * @throws \ReflectionException
     */
    public static function getClassAttributes($objectOrClass, $attribute, $includeParents = false)
    {}

    /**
     * Get the class name of the given parameter's type, if possible.
     *
     * @param  \ReflectionParameter  $parameter
     * @return string|null
     */
    public static function getParameterClassName($parameter)
    {}

    /**
     * Get the class names of the given parameter's type, including union types.
     *
     * @param  \ReflectionParameter  $parameter
     * @return array
     */
    public static function getParameterClassNames($parameter)
    {}

    /**
     * Get the given type's class name.
     *
     * @param  \ReflectionParameter  $parameter
     * @param  \ReflectionNamedType  $type
     * @return string
     */
    protected static function getTypeName($parameter, $type)
    {}

    /**
     * Determine if the parameter's type is a subclass of the given type.
     *
     * @param  \ReflectionParameter  $parameter
     * @param  string  $className
     * @return bool
     */
    public static function isParameterSubclassOf($parameter, $className)
    {}

    /**
     * Determine if the parameter's type is a Backed Enum with a string backing type.
     *
     * @param  \ReflectionParameter  $parameter
     * @return bool
     */
    public static function isParameterBackedEnumWithStringBackingType($parameter)
    {}
}
