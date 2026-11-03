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

use function array_key_exists;
use function array_values;
use function count;
use function is_array;
use PHPUnit\Framework\ExpectationFailedException;
use PHPUnit\Framework\MockObject\Invocation as BaseInvocation;
use PHPUnit\Framework\MockObject\NoMoreParameterSetsConfiguredException;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class PartiallyOrderedParameterSets implements ParametersRule
{
    /**
     * @var list<IndexedParameters>
     */
    private array $stack = [];

    /**
     * @var list<IndexedParameters>
     */
    private array $applied                     = [];
    private ?OrderedParameterSets $ordered     = null;
    private ?UnorderedParameterSets $unordered = null;
    private int $numberOfConfiguredParameterSets;
    private int $numberOfInvocations = 0;

    /**
     * @param list<mixed> $stack
     */
    public function __construct(array $stack)
    {}

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
}
