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

use function array_flip;
use function array_key_exists;
use function array_map;
use function array_merge;
use function array_pop;
use function assert;
use function count;
use function is_string;
use function range;
use function strtolower;
use PHPUnit\Framework\Constraint\Constraint;
use PHPUnit\Framework\InvalidArgumentException;
use PHPUnit\Framework\MockObject\Runtime\PropertyHook;
use PHPUnit\Framework\MockObject\Stub\ConsecutiveCalls;
use PHPUnit\Framework\MockObject\Stub\Exception;
use PHPUnit\Framework\MockObject\Stub\ReturnArgument;
use PHPUnit\Framework\MockObject\Stub\ReturnCallback;
use PHPUnit\Framework\MockObject\Stub\ReturnReference;
use PHPUnit\Framework\MockObject\Stub\ReturnSelf;
use PHPUnit\Framework\MockObject\Stub\ReturnStub;
use PHPUnit\Framework\MockObject\Stub\ReturnValueMap;
use PHPUnit\Framework\MockObject\Stub\Stub;
use Throwable;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
abstract class AbstractInvocationImplementation implements InvocationStubber
{
    protected readonly InvocationHandler $invocationHandler;
    protected readonly Matcher $matcher;

    /**
     * @var list<ConfigurableMethod>
     */
    protected readonly array $configurableMethods;

    /**
     * @var ?array<string, int>
     */
    protected ?array $configurableMethodNames     = null;
    protected bool $createdWithoutExplicitExpects = false;

    final public function __construct(InvocationHandler $handler, Matcher $matcher, ConfigurableMethod ...$configurableMethods)
    {}

    /**
     * @param Constraint|non-empty-string|PropertyHook $constraint
     *
     * @throws InvalidArgumentException
     * @throws MethodCannotBeConfiguredException
     * @throws MethodNameAlreadyConfiguredException
     *
     * @return $this
     */
    final public function method(Constraint|PropertyHook|string $constraint): InvocationStubber
    {}

    /**
     * @return $this
     */
    final public function will(Stub $stub): InvocationStubber
    {}

    /**
     * @throws IncompatibleReturnValueException
     */
    final public function willReturn(mixed $value, mixed ...$nextValues): InvocationStubber
    {}

    final public function willReturnReference(mixed &$reference): InvocationStubber
    {}

    final public function willReturnMap(array $valueMap): InvocationStubber
    {}

    final public function willReturnStrictMap(array $valueMap): InvocationStubber
    {}

    final public function willReturnArgument(int $argumentIndex): InvocationStubber
    {}

    final public function willReturnCallback(callable $callback): InvocationStubber
    {}

    final public function willReturnSelf(): InvocationStubber
    {}

    final public function willReturnOnConsecutiveCalls(mixed ...$values): InvocationStubber
    {}

    final public function willThrowException(Throwable $exception): InvocationStubber
    {}

    /**
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    final public function markAsCreatedWithoutExplicitExpects(): static
    {}

    final public function seal(): void
    {}

    /**
     * @throws MethodNameNotConfiguredException
     * @throws MethodParametersAlreadyConfiguredException
     */
    final protected function ensureParametersCanBeConfigured(): void
    {}

    private function configuredMethod(): ?ConfigurableMethod
    {}

    /**
     * @param array<int, array<int, mixed>> $valueMap
     *
     * @return array<int, array<int, mixed>>
     */
    private function normalizeValueMap(array $valueMap): array
    {}

    /**
     * @param array<mixed> $values
     *
     * @throws IncompatibleReturnValueException
     */
    private function ensureTypeOfReturnValues(array $values): void
    {}
}
