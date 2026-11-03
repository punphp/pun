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

use function array_shift;
use function count;
use function implode;
use function is_array;
use function sprintf;
use PHPUnit\Framework\ExpectationFailedException;
use PHPUnit\Framework\MockObject\Invocation as BaseInvocation;
use PHPUnit\Framework\MockObject\NoMoreParameterSetsConfiguredException;

final class UnorderedParameterSets implements ParametersRule
{
    /**
     * @var list<IndexedParameters>
     */
    private array $stack = [];

    /**
     * @var list<IndexedParameters>
     */
    private array $unapplied = [];

    /**
     * @var list<IndexedParameters>
     */
    private array $applied = [];
    private int $numberOfConfiguredParameterSets;

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
