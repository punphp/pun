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

use function sprintf;
use function trim;
use PHPUnit\Framework\ExpectationFailedException;
use PHPUnit\Util\Exporter;
use SebastianBergmann\Comparator\ComparisonFailure;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 */
final class IsEqualWithDelta extends Constraint
{
    private readonly mixed $value;
    private readonly float $delta;

    public function __construct(mixed $value, float $delta)
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
    public function evaluate(mixed $other, string $description = '', bool $returnResult = false): bool
    {}

    /**
     * Returns a string representation of the constraint.
     */
    public function toString(): string
    {}

    /**
     * Returns the negated string representation of the constraint.
     *
     * Authoring the negation here keeps the exported value out of the
     * negation entirely.
     */
    protected function negatedToString(): string
    {}

    private function valueAsString(): string
    {}
}
