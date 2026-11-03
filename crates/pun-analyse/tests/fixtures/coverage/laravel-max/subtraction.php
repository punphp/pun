<?php

namespace LaravelMax\Subtraction;

use ReflectionType;
use ReflectionUnionType;

abstract class Shape
{
}

class Circle extends Shape
{
}

class Square extends Shape
{
}

class Subtraction
{
    public function negatedSubclass(ReflectionType $type, Shape $shape): void
    {
        if ($type instanceof ReflectionUnionType) {
            return;
        }
        \PHPStan\dumpType($type);

        if ($shape instanceof Circle) {
            return;
        }
        \PHPStan\dumpType($shape);

        if (! $shape instanceof Square) {
            \PHPStan\dumpType($shape);
        }
    }
}
