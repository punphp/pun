<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Util;

use function array_keys;
use function array_merge;
use function array_reverse;
use function assert;
use PHPUnit\Framework\Assert;
use PHPUnit\Framework\TestCase;
use ReflectionClass;
use ReflectionException;
use ReflectionMethod;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class Reflection
{
    /**
     * @param class-string     $className
     * @param non-empty-string $methodName
     *
     * @return array{file: non-empty-string, line: non-negative-int}
     */
    public static function sourceLocationFor(string $className, string $methodName): array
    {}

    /**
     * A hook method that is not declared by the test class, or that is only
     * declared by TestCase itself, must not be invoked as a hook method.
     *
     * @param ReflectionClass<TestCase> $class
     * @param non-empty-string          $methodName
     */
    public static function methodDoesNotExistOrIsDeclaredInTestCase(ReflectionClass $class, string $methodName): bool
    {}

    /**
     * @param ReflectionClass<TestCase> $class
     *
     * @return list<ReflectionMethod>
     */
    public static function publicMethodsDeclaredDirectlyInTestClass(ReflectionClass $class): array
    {}

    /**
     * @param ReflectionClass<TestCase> $class
     *
     * @return list<ReflectionMethod>
     */
    public static function methodsDeclaredDirectlyInTestClass(ReflectionClass $class): array
    {}

    /**
     * The files that declare what a test class is made of: the file that
     * declares the class itself, the files that declare the classes it
     * extends, and the files that declare the traits any of them use.
     *
     * Test methods, and the metadata they carry, are not contributed by the
     * file that declares the class alone: they are collected from parent
     * classes as well, see filterAndSortMethods(), and both classes and traits
     * may contribute them.
     *
     * The walk stops at TestCase, which PHPUnit declares itself: a method
     * declared by TestCase, or by Assert, the class it extends, is never
     * treated as a test method, so neither of them can contribute anything.
     *
     * Interfaces are not considered: they cannot contribute a test method to a
     * concrete test class.
     *
     * @param ReflectionClass<TestCase> $class
     *
     * @return list<non-empty-string>
     */
    public static function sourceFilesOf(ReflectionClass $class): array
    {}

    /**
     * @param ReflectionClass<object>       $class
     * @param array<non-empty-string, true> $files
     */
    private static function collectSourceFilesOf(ReflectionClass $class, array &$files): void
    {}

    /**
     * @param ReflectionClass<TestCase> $class
     *
     * @return list<ReflectionMethod>
     */
    private static function filterAndSortMethods(ReflectionClass $class, ?int $filter, bool $sortHighestToLowest): array
    {}
}
