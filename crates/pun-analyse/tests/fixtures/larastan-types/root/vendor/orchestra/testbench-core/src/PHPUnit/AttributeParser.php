<?php

namespace Orchestra\Testbench\PHPUnit;

use Orchestra\Testbench\Contracts\Attributes\Resolvable as ResolvableContract;
use Orchestra\Testbench\Contracts\Attributes\TestingFeature;
use ReflectionAttribute;
use ReflectionClass;
use ReflectionMethod;

/**
 * @internal
 *
 * @phpstan-type TTestingFeature \Orchestra\Testbench\Contracts\Attributes\TestingFeature
 * @phpstan-type TAttributes TTestingFeature|\Orchestra\Testbench\Contracts\Attributes\Resolvable
 */
class AttributeParser
{
    /**
     * Parse attribute for class.
     *
     * @param  class-string  $className
     * @return array<int, array{key: class-string, instance: object}>
     *
     * @phpstan-return array<int, array{key: class-string<TTestingFeature>, instance: TTestingFeature}>
     *
     * @codeCoverageIgnore
     */
    public static function forClass(string $className): array
    {}

    /**
     * Parse attribute for method.
     *
     * @param  class-string  $className
     * @param  string  $methodName
     * @return array<int, array{key: class-string, instance: object}>
     *
     * @phpstan-return array<int, array{key: class-string<TTestingFeature>, instance: TTestingFeature}>
     *
     * @codeCoverageIgnore
     */
    public static function forMethod(string $className, string $methodName): array
    {}

    /**
     * Validate given attribute.
     *
     * @param  class-string|object  $class
     * @return bool
     */
    public static function validAttribute($class): bool
    {}

    /**
     * Resolve given attribute.
     *
     * @param  \ReflectionAttribute  $attribute
     * @return array{0: class-string, 1: object|null}
     *
     * @phpstan-return array{0: class-string<TTestingFeature>|null, 1: TTestingFeature|null}
     */
    protected static function resolveAttribute(ReflectionAttribute $attribute): array
    {}
}
