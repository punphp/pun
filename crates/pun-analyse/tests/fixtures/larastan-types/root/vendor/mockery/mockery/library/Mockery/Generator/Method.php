<?php

/**
 * Mockery (https://docs.mockery.io/en/stable/)
 *
 * @copyright https://github.com/mockery/mockery/blob/HEAD/COPYRIGHT.md
 * @license   https://github.com/mockery/mockery/blob/HEAD/LICENSE BSD 3-Clause License
 * @see       https://github.com/mockery/mockery for the canonical source repository
 */

namespace Mockery\Generator;

use Mockery\Reflector;
use ReflectionMethod;
use ReflectionParameter;

use function array_map;

/**
 * @mixin ReflectionMethod
 */
class Method
{
    /**
     * @var ReflectionMethod
     */
    private $reflectionMethod;

    public function __construct(ReflectionMethod $method)
    {}

    /**
     * @param  string       $method
     * @param  array<mixed> $args
     * @return mixed
     */
    public function __call($method, $args)
    {}

    /**
     * @return list<Parameter>
     */
    public function getParameters()
    {}

    /**
     * @return null|string
     */
    public function getReturnType()
    {}
}
