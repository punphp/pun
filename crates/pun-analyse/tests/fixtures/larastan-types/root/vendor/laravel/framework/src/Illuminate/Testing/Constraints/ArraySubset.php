<?php

namespace Illuminate\Testing\Constraints;

use ArrayObject;
use PHPUnit\Framework\Constraint\Constraint;
use SebastianBergmann\Comparator\ComparisonFailure;
use SebastianBergmann\Exporter\Exporter;
use Traversable;

class ArraySubset extends Constraint
{
    /**
     * @var iterable
     */
    protected $subset;

    /**
     * @var bool
     */
    protected $strict;

    /**
     * Create a new array subset constraint instance.
     *
     * @param  iterable  $subset
     * @param  bool  $strict
     */
    public function __construct(iterable $subset, bool $strict = false)
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
     * @param  mixed  $other
     * @param  string  $description
     * @param  bool  $returnResult
     * @return bool|null
     *
     * @throws \PHPUnit\Framework\ExpectationFailedException
     * @throws \SebastianBergmann\RecursionContext\InvalidArgumentException
     */
    public function evaluate($other, string $description = '', bool $returnResult = false): ?bool
    {}

    /**
     * Returns a string representation of the constraint.
     *
     * @return string
     *
     * @throws \SebastianBergmann\RecursionContext\InvalidArgumentException
     */
    public function toString(): string
    {}

    /**
     * Returns the description of the failure.
     *
     * The beginning of failure messages is "Failed asserting that" in most
     * cases. This method should return the second part of that sentence.
     *
     * @param  mixed  $other
     * @return string
     *
     * @throws \SebastianBergmann\RecursionContext\InvalidArgumentException
     */
    protected function failureDescription($other): string
    {}

    /**
     * Returns the description of the failure.
     *
     * The beginning of failure messages is "Failed asserting that" in most
     * cases. This method should return the second part of that sentence.
     *
     * @param  iterable  $other
     * @return array
     */
    protected function toArray(iterable $other): array
    {}
}
