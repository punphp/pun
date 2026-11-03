<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Framework\Constraint;

use function array_key_exists;
use function array_values;
use function count;
use function is_array;
use PHPUnit\Framework\ExpectationFailedException;
use PHPUnit\Util\Exporter;
use SebastianBergmann\Comparator\ComparisonFailure;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 */
abstract class ArrayComparison extends Constraint
{
    /**
     * @var array<mixed>
     */
    protected readonly array $expected;
    protected readonly bool $keysMatter;
    protected readonly bool $orderMatters;

    /**
     * @param array<mixed> $expected
     */
    public function __construct(array $expected, bool $keysMatter, bool $orderMatters)
    {}

    /**
     * Evaluates the constraint for parameter $other.
     *
     * If $returnResult is set to false (the default), an exception is thrown
     * in case of a failure. null is returned otherwise.
     *
     * If $returnResult is true, the result of the evaluation is returned as
     * a boolean value instead: true in case of success, false in case of a
     * failure.
     *
     * @throws ExpectationFailedException
     */
    public function evaluate(mixed $other, string $description = '', bool $returnResult = false): ?bool
    {}

    /**
     * Returns a string representation of the constraint.
     */
    public function toString(): string
    {}

    /**
     * Returns the negated string representation of the constraint.
     */
    protected function negatedToString(): string
    {}

    /**
     * Returns the description of the failure.
     *
     * The beginning of failure messages is "Failed asserting that" in most
     * cases. This method should return the second part of that sentence.
     */
    protected function failureDescription(mixed $other): string
    {}

    protected function negatedFailureDescription(mixed $other): string
    {}

    /**
     * @return 'equal'|'identical'
     */
    abstract protected function comparisonType(): string;

    /**
     * Compares two non-array values (or two arrays in modes where order at
     * every level is significant) using the strictness defined by the
     * concrete subclass.
     */
    abstract protected function compareLeaf(mixed $expected, mixed $actual): bool;

    private function comparisonDescription(): string
    {}

    /**
     * Compares two values, recursing into arrays as needed.
     *
     * Whenever both $expected and $actual are arrays, the same comparison
     * mode (controlled by $keysMatter and $orderMatters) is applied at every
     * level of nesting. For all other value combinations, the leaf comparison
     * provided by the concrete subclass is used.
     */
    private function compare(mixed $expected, mixed $actual): bool
    {}

    /**
     * @param array<mixed> $expected
     * @param array<mixed> $actual
     */
    private function compareKeyedIgnoringOrder(array $expected, array $actual): bool
    {}

    /**
     * @param array<mixed> $expected
     * @param array<mixed> $actual
     */
    private function compareIgnoringKeysAndOrder(array $expected, array $actual): bool
    {}
}
