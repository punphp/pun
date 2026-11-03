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

use function array_merge;
use PHPUnit\Framework\MockObject\Generator\Generator;
use PHPUnit\Framework\MockObject\Generator\ReflectionException;
use ReflectionClass;

/**
 * @template T of object
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 */
abstract class TestDoubleBuilder
{
    /**
     * @var class-string<T>
     */
    protected readonly string $type;

    /**
     * @var list<non-empty-string>
     */
    protected array $methods          = [];
    protected bool $emptyMethodsArray = false;

    /**
     * @var array<mixed>
     */
    protected array $constructorArgs      = [];
    protected bool $originalConstructor   = true;
    protected bool $originalClone         = true;
    protected bool $returnValueGeneration = true;

    /**
     * @var list<non-empty-string>
     */
    protected array $doubledProperties = [];

    /**
     * @param class-string<T> $type
     */
    public function __construct(string $type)
    {}

    /**
     * Specifies the subset of methods to mock, requiring each to exist in the class.
     *
     * @param list<non-empty-string> $methods
     *
     * @throws CannotUseOnlyMethodsException
     * @throws ReflectionException
     *
     * @return $this
     */
    public function onlyMethods(array $methods): static
    {}

    /**
     * Specifies properties that do not declare property hooks for which property hooks should be doubled.
     *
     * @param list<non-empty-string> $properties
     *
     * @throws PropertyCannotBeDoubledException
     * @throws ReflectionException
     *
     * @return $this
     */
    public function doubleProperties(array $properties): static
    {}

    /**
     * Specifies the arguments for the constructor.
     *
     * @param array<mixed> $arguments
     *
     * @return $this
     */
    public function setConstructorArgs(array $arguments): static
    {}

    /**
     * Disables the invocation of the original constructor.
     *
     * @return $this
     */
    public function disableOriginalConstructor(): static
    {}

    /**
     * Enables the invocation of the original constructor.
     *
     * @return $this
     */
    public function enableOriginalConstructor(): static
    {}

    /**
     * Disables the invocation of the original clone constructor.
     *
     * @return $this
     */
    public function disableOriginalClone(): static
    {}

    /**
     * Enables the invocation of the original clone constructor.
     *
     * @return $this
     */
    public function enableOriginalClone(): static
    {}

    /**
     * @return $this
     */
    public function enableAutoReturnValueGeneration(): static
    {}

    /**
     * @return $this
     */
    public function disableAutoReturnValueGeneration(): static
    {}

    protected function getTestDouble(?string $testDoubleClassName, bool $mockObject): MockObject|Stub
    {}
}
