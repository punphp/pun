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

use function sprintf;
use PHPUnit\Framework\ExpectationFailedException;
use PHPUnit\Framework\MockObject\Rule\AnyInvokedCount;
use PHPUnit\Framework\MockObject\Rule\AnyParameters;
use PHPUnit\Framework\MockObject\Rule\InvocationOrder;
use PHPUnit\Framework\MockObject\Rule\InvokedAtMostCount;
use PHPUnit\Framework\MockObject\Rule\InvokedCount;
use PHPUnit\Framework\MockObject\Rule\MethodName;
use PHPUnit\Framework\MockObject\Rule\ParametersRule;
use PHPUnit\Framework\MockObject\Stub\Stub;
use PHPUnit\Util\ThrowableToStringMapper;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class Matcher
{
    private readonly InvocationOrder $invocationRule;

    /**
     * @var class-string
     */
    private readonly string $className;

    /**
     * @var ?non-empty-string
     */
    private ?string $afterMatchBuilderId    = null;
    private ?MethodName $methodNameRule     = null;
    private ?ParametersRule $parametersRule = null;
    private ?Stub $stub                     = null;

    /**
     * @param class-string $className
     */
    public function __construct(InvocationOrder $rule, string $className)
    {}

    public function hasInvocationCountRule(): bool
    {}

    /**
     * @phpstan-assert-if-true !null $this->methodNameRule
     */
    public function hasMethodNameRule(): bool
    {}

    /**
     * @throws MethodNameNotConfiguredException
     */
    public function methodNameRule(): MethodName
    {}

    public function setMethodNameRule(MethodName $rule): void
    {}

    /**
     * @phpstan-assert-if-true !null $this->parametersRule
     */
    public function hasParametersRule(): bool
    {}

    public function setParametersRule(ParametersRule $rule): void
    {}

    public function setStub(Stub $stub): void
    {}

    /**
     * @param non-empty-string $id
     */
    public function setAfterMatchBuilderId(string $id): void
    {}

    /**
     * @throws Exception
     * @throws ExpectationFailedException
     * @throws MatchBuilderNotFoundException
     * @throws MethodNameNotConfiguredException
     * @throws RuntimeException
     */
    public function invoked(Invocation $invocation): mixed
    {}

    /**
     * @throws ExpectationFailedException
     * @throws MatchBuilderNotFoundException
     * @throws MethodNameNotConfiguredException
     * @throws RuntimeException
     */
    public function matches(Invocation $invocation): bool
    {}

    /**
     * @throws ExpectationFailedException
     * @throws MethodNameNotConfiguredException
     */
    public function verify(): void
    {}
}
