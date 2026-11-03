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

use function is_string;
use function json_decode;
use function sprintf;
use PHPUnit\Framework\ExpectationFailedException;
use PHPUnit\Util\InvalidJsonException;
use PHPUnit\Util\Json;
use SebastianBergmann\Comparator\ComparisonFailure;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 */
final class JsonMatches extends Constraint
{
    private readonly string $value;

    public function __construct(string $value)
    {}

    /**
     * Returns a string representation of the object.
     */
    public function toString(): string
    {}

    /**
     * Returns the negated string representation of the constraint.
     *
     * Authoring the negation here keeps the expected JSON out of the negation
     * entirely.
     */
    protected function negatedToString(): string
    {}

    /**
     * Evaluates the constraint for parameter $other. Returns true if the
     * constraint is met, false otherwise.
     *
     * This method can be overridden to implement the evaluation algorithm.
     */
    protected function matches(mixed $other): bool
    {}

    /**
     * Throws an exception for the given compared value and test description.
     *
     * @throws ExpectationFailedException
     * @throws InvalidJsonException
     */
    protected function fail(mixed $other, string $description, ?ComparisonFailure $comparisonFailure = null): never
    {}
}
