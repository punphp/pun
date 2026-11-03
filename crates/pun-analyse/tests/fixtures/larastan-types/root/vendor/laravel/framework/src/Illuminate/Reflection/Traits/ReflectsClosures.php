<?php

namespace Illuminate\Support\Traits;

use Closure;
use Illuminate\Support\Collection;
use Illuminate\Support\Reflector;
use ReflectionFunction;
use ReflectionIntersectionType;
use ReflectionUnionType;
use RuntimeException;

trait ReflectsClosures
{
    /**
     * Get the class name of the first parameter of the given Closure.
     *
     * @param  \Closure  $closure
     * @return string
     *
     * @throws \ReflectionException
     * @throws \RuntimeException
     */
    protected function firstClosureParameterType(Closure $closure)
    {}

    /**
     * Get the class names of the first parameter of the given Closure, including union types.
     *
     * @param  \Closure  $closure
     * @return array
     *
     * @throws \ReflectionException
     * @throws \RuntimeException
     */
    protected function firstClosureParameterTypes(Closure $closure)
    {}

    /**
     * Get the class names / types of the parameters of the given Closure.
     *
     * @param  \Closure  $closure
     * @return array
     *
     * @throws \ReflectionException
     */
    protected function closureParameterTypes(Closure $closure)
    {}

    /**
     * Get the class names / types of the return type of the given Closure.
     *
     * @param  \Closure  $closure
     * @return list<class-string>
     *
     * @throws \ReflectionException
     */
    protected function closureReturnTypes(Closure $closure)
    {}
}
