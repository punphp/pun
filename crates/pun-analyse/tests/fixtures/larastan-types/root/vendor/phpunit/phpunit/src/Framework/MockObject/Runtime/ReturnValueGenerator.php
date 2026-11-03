<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Framework\MockObject;

use function array_all;
use function array_map;
use function explode;
use function in_array;
use function interface_exists;
use function sprintf;
use function str_contains;
use function str_ends_with;
use function str_starts_with;
use function substr;
use PHPUnit\Framework\MockObject\Generator\Generator;
use ReflectionClass;
use ReflectionObject;
use stdClass;
use Throwable;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class ReturnValueGenerator
{
    /**
     * @param class-string     $className
     * @param non-empty-string $methodName
     *
     * @throws Exception
     */
    public function generate(string $className, string $methodName, StubInternal $testStub, string $returnType): mixed
    {}

    /**
     * @param non-empty-list<string> $types
     */
    private function onlyInterfaces(array $types): bool
    {}

    /**
     * @param class-string     $className
     * @param non-empty-string $methodName
     *
     * @throws RuntimeException
     */
    private function newInstanceOf(StubInternal $testStub, string $className, string $methodName): Stub
    {}

    /**
     * @param class-string     $type
     * @param class-string     $className
     * @param non-empty-string $methodName
     *
     * @throws RuntimeException
     */
    private function testDoubleFor(string $type, string $className, string $methodName): Stub
    {}

    /**
     * @param non-empty-list<string> $types
     * @param class-string           $className
     * @param non-empty-string       $methodName
     *
     * @throws RuntimeException
     */
    private function testDoubleForIntersectionOfInterfaces(array $types, string $className, string $methodName): Stub
    {}
}
