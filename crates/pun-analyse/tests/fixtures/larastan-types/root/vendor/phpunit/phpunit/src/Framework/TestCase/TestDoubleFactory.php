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
use function implode;
use PHPUnit\Event;
use PHPUnit\Event\NoPreviousThrowableException;
use PHPUnit\Framework\InvalidArgumentException;
use PHPUnit\Framework\MockObject\Exception as MockObjectException;
use PHPUnit\Framework\MockObject\Generator\Generator as MockGenerator;
use PHPUnit\Framework\MockObject\MockBuilder;
use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\MockObject\Stub;
use PHPUnit\Framework\TestCase;
use PHPUnit\Metadata\Parser\Registry as MetadataRegistry;

/**
 * Creates the test doubles that a test case class requests, registers mock
 * objects with the test that requested them, and emits the corresponding
 * events.
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class TestDoubleFactory
{
    /**
     * @var class-string
     */
    private string $testClassName;
    private Event\Emitter $emitter;

    /**
     * @param class-string $testClassName
     */
    public function __construct(string $testClassName, Event\Emitter $emitter)
    {}

    /**
     * @template RealInstanceType of object
     *
     * @param class-string<RealInstanceType> $type
     *
     * @throws InvalidArgumentException
     * @throws MockObjectException
     * @throws NoPreviousThrowableException
     *
     * @return MockObject&RealInstanceType
     */
    public function createMock(TestCase $test, string $type): MockObject
    {}

    /**
     * @param list<class-string> $interfaces
     *
     * @throws MockObjectException
     */
    public function createMockForIntersectionOfInterfaces(TestCase $test, array $interfaces): MockObject
    {}

    /**
     * @template RealInstanceType of object
     *
     * @param class-string<RealInstanceType> $type
     * @param array<non-empty-string, mixed> $configuration
     *
     * @throws InvalidArgumentException
     * @throws MockObjectException
     * @throws NoPreviousThrowableException
     *
     * @return MockObject&RealInstanceType
     */
    public function createConfiguredMock(TestCase $test, string $type, array $configuration): MockObject
    {}

    /**
     * @template RealInstanceType of object
     *
     * @param class-string<RealInstanceType> $type
     * @param list<non-empty-string>         $methods
     *
     * @throws InvalidArgumentException
     * @throws MockObjectException
     *
     * @return MockObject&RealInstanceType
     */
    public function createPartialMock(TestCase $test, string $type, array $methods): MockObject
    {}

    /**
     * @template RealInstanceType of object
     *
     * @param class-string<RealInstanceType> $type
     *
     * @throws InvalidArgumentException
     * @throws MockObjectException
     * @throws NoPreviousThrowableException
     *
     * @return RealInstanceType&Stub
     */
    public function createStub(string $type): Stub
    {}

    /**
     * @param list<class-string> $interfaces
     *
     * @throws MockObjectException
     */
    public function createStubForIntersectionOfInterfaces(array $interfaces): Stub
    {}

    /**
     * @template RealInstanceType of object
     *
     * @param class-string<RealInstanceType> $type
     * @param array<non-empty-string, mixed> $configuration
     *
     * @throws InvalidArgumentException
     * @throws MockObjectException
     * @throws NoPreviousThrowableException
     *
     * @return RealInstanceType&Stub
     */
    public function createConfiguredStub(string $type, array $configuration): Stub
    {}

    private function generateReturnValues(): bool
    {}
}
