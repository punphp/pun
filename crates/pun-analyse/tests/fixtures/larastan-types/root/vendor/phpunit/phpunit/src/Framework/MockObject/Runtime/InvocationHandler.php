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

use function array_any;
use function array_unique;
use function array_values;
use function in_array;
use function sprintf;
use function strtolower;
use Exception;
use PHPUnit\Framework\AssertionFailedError;
use PHPUnit\Framework\Constraint\Constraint;
use PHPUnit\Framework\InvalidArgumentException;
use PHPUnit\Framework\MockObject\Rule\AnyInvokedCount;
use PHPUnit\Framework\MockObject\Rule\InvocationOrder;
use PHPUnit\Framework\MockObject\Rule\InvokedCount;
use PHPUnit\Framework\MockObject\Rule\MethodName;
use PHPUnit\Framework\MockObject\Runtime\PropertyHook;
use Throwable;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class InvocationHandler
{
    /**
     * @var list<Matcher>
     */
    private array $matchers = [];

    /**
     * @var array<non-empty-string, Matcher>
     */
    private array $matcherMap = [];

    /**
     * @var list<ConfigurableMethod>
     */
    private readonly array $configurableMethods;

    /**
     * @var class-string
     */
    private readonly string $className;

    /**
     * @var non-empty-string
     */
    private readonly string $displayName;
    private readonly bool $returnValueGeneration;
    private readonly bool $isMockObject;
    private bool $sealed                            = false;
    private ?AssertionFailedError $assertionFailure = null;
    private ?InvocationJournalInternal $journal     = null;

    /**
     * @var ?non-empty-string
     */
    private ?string $journalLabel = null;

    /**
     * @var array<non-empty-string, non-empty-string>
     */
    private array $journalMethodLabels = [];

    /**
     * @param list<ConfigurableMethod> $configurableMethods
     * @param class-string             $className
     * @param ?non-empty-string        $displayName
     */
    public function __construct(array $configurableMethods, string $className, bool $returnValueGeneration, bool $isMockObject = false, ?string $displayName = null)
    {}

    public function isMockObject(): bool
    {}

    /**
     * @param ?non-empty-string                         $label
     * @param array<non-empty-string, non-empty-string> $methodLabels
     *
     * @throws EmptyInvocationJournalLabelException
     * @throws InvocationJournalAlreadyRegisteredException
     * @throws MethodCannotBeConfiguredException
     * @throws TestDoubleSealedException
     */
    public function recordInvocationsIn(InvocationJournalInternal $journal, ?string $label, array $methodLabels): void
    {}

    public function recordsInvocations(): bool
    {}

    public function hasInvocationCountRule(): bool
    {}

    public function hasParametersRule(): bool
    {}

    /**
     * Looks up the match builder with identification $id and returns it.
     *
     * @param non-empty-string $id
     */
    public function lookupMatcher(string $id): ?Matcher
    {}

    /**
     * Registers a matcher with the identification $id. The matcher can later be
     * looked up using lookupMatcher() to figure out if it has been invoked.
     *
     * @param non-empty-string $id
     *
     * @throws MatcherAlreadyRegisteredException
     */
    public function registerMatcher(string $id, Matcher $matcher): void
    {}

    /**
     * @throws TestDoubleSealedException
     */
    public function expects(InvocationOrder $rule): InvocationMocker|InvocationStubber
    {}

    /**
     * The matcher is only added once the method name has been configured
     * so that a method that cannot be configured does not leave behind a
     * matcher without a method name rule.
     *
     * @param Constraint|non-empty-string|PropertyHook $constraint
     *
     * @throws InvalidArgumentException
     * @throws MethodCannotBeConfiguredException
     * @throws TestDoubleSealedException
     */
    public function method(Constraint|PropertyHook|string $constraint): InvocationStubber
    {}

    /**
     * @throws \PHPUnit\Framework\MockObject\Exception
     * @throws Exception
     */
    public function invoke(Invocation $invocation): mixed
    {}

    /**
     * @throws Throwable
     */
    public function verify(): void
    {}

    public function seal(bool $isMockObject): void
    {}

    public function isSealed(): bool
    {}

    public function hasMatcherWithParametersRuleForMethodName(Matcher $excludeMatcher, string $methodName): bool
    {}

    private function addMatcher(Matcher $matcher): void
    {}

    /**
     * Returns the list of method names that have been configured with expectations.
     * Only considers exact string matches for method names.
     * Methods with any() expectation are considered configured.
     *
     * @return list<non-empty-string>
     */
    private function configuredMethodNames(): array
    {}

    /**
     * @param non-empty-string $methodName
     *
     * @return non-empty-string
     */
    private function journalLabelFor(string $methodName): string
    {}

    /**
     * @param non-empty-string $methodName
     */
    private function hasConfigurableMethod(string $methodName): bool
    {}
}
