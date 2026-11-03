<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\TestRunner\TestResult;

use function assert;
use function explode;
use function in_array;
use PHPUnit\Event\Code\TestMethod;
use PHPUnit\Framework\TestSize\Known;
use PHPUnit\Framework\TestSize\TestSize;
use PHPUnit\Metadata\Api\Groups;
use ReflectionMethod;
use ReflectionNamedType;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class PassedTests
{
    private static ?self $instance = null;

    /**
     * @var list<class-string>
     */
    private array $passedTestClasses = [];

    /**
     * @var array<string,array{returnValue: mixed, size: TestSize}>
     */
    private array $passedTestMethods = [];

    public static function instance(): self
    {}

    /**
     * @param class-string $className
     */
    public function testClassPassed(string $className): void
    {}

    public function testMethodPassed(TestMethod $test, mixed $returnValue): void
    {}

    /**
     * The passes are added to the ones recorded so far rather than merged
     * with them into new arrays: merging would copy every pass recorded so
     * far on each import, and the main process imports the passes of every
     * test that runs in a separate process, which made importing them
     * quadratic.
     */
    public function import(self $other): void
    {}

    /**
     * @param class-string $className
     */
    public function hasTestClassPassed(string $className): bool
    {}

    public function hasTestMethodPassed(string $method): bool
    {}

    public function isGreaterThan(string $method, TestSize $other): bool
    {}

    public function hasReturnValue(string $method): bool
    {}

    public function returnValue(string $method): mixed
    {}
}
