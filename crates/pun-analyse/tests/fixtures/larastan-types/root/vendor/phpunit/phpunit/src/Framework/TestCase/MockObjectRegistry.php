<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Framework\TestCase;

use function assert;
use function in_array;
use function is_array;
use function is_object;
use function sprintf;
use function str_starts_with;
use PHPUnit\Event;
use PHPUnit\Framework\ExpectationFailedException;
use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\MockObject\MockObjectInternal;
use PHPUnit\Framework\MockObject\Rule\InvokedCount;
use PHPUnit\Framework\TestCase;
use PHPUnit\Metadata\Parser\Registry as MetadataRegistry;
use PHPUnit\TextUI\Configuration\Registry as ConfigurationRegistry;
use SebastianBergmann\ObjectEnumerator\Enumerator;
use Throwable;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class MockObjectRegistry
{
    /**
     * @var list<array{type: non-empty-string, mockObject: MockObjectInternal}>
     */
    private array $mockObjects = [];

    /**
     * @param non-empty-string $type
     */
    public function register(string $type, MockObject $mockObject): void
    {}

    public function clear(): void
    {}

    /**
     * @throws Throwable
     */
    public function verify(TestCase $test, Event\Emitter $emitter): void
    {}

    /**
     * An expectation failure raised by an invocation count rule while the
     * test method was running counts as an assertion.
     *
     * @see https://github.com/sebastianbergmann/phpunit/issues/6095
     */
    public function handleExceptionFromInvokedCountRule(TestCase $test, Throwable $t): void
    {}

    private function allowsMockObjectsWithoutExpectations(TestCase $test): bool
    {}

    private function shouldInvocationMockerBeReset(TestCase $test, MockObject $mock): bool
    {}
}
