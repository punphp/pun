<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Framework\MockObject\Rule;

use function array_diff_key;
use function array_is_list;
use function assert;
use function count;
use function is_int;
use function is_string;
use function method_exists;
use function sprintf;
use Exception;
use PHPUnit\Framework\Constraint\Callback;
use PHPUnit\Framework\Constraint\Constraint;
use PHPUnit\Framework\Constraint\IsAnything;
use PHPUnit\Framework\Constraint\IsEqual;
use PHPUnit\Framework\ExpectationFailedException;
use PHPUnit\Framework\MockObject\Invocation as BaseInvocation;
use PHPUnit\Util\Test;
use ReflectionException;
use ReflectionMethod;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class Parameters implements ParametersRule
{
    /**
     * @var array<int|string, Constraint>
     */
    private array $parameters           = [];
    private ?BaseInvocation $invocation = null;
    private null|bool|ExpectationFailedException $parameterVerificationResult;
    private bool $useAssertionCount = true;

    /**
     * @param array<int|string, mixed> $parameters
     *
     * @throws \PHPUnit\Framework\Exception
     */
    public function __construct(array $parameters)
    {}

    /**
     * @throws Exception
     */
    public function apply(BaseInvocation $invocation): void
    {}

    /**
     * Checks if the invocation $invocation matches the current rules. If it
     * does the rule will get the invoked() method called which should check
     * if an expectation is met.
     *
     * @throws ExpectationFailedException
     */
    public function verify(): void
    {}

    public function useAssertionCount(bool $useAssertionCount): void
    {}

    /**
     * @throws ExpectationFailedException
     */
    private function doVerify(): bool
    {}

    /**
     * Maps the constraints that were configured using named arguments to the
     * position of the parameter with that name. Constraints for named arguments
     * that are collected by a variadic parameter keep their name.
     *
     * @throws ExpectationFailedException
     *
     * @return array<int|string, Constraint>
     */
    private function resolveNamedParameters(BaseInvocation $invocation): array
    {}

    /**
     * @throws ExpectationFailedException
     */
    private function guardAgainstDuplicateEvaluationOfParameterConstraints(): bool
    {}

    private function incrementAssertionCount(): void
    {}

    /**
     * @return array<non-negative-int, non-empty-string>
     */
    private function parameters(BaseInvocation $invocation): array
    {}
}
