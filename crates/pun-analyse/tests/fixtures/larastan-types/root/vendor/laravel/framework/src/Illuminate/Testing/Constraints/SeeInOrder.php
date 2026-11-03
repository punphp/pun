<?php

namespace Illuminate\Testing\Constraints;

use PHPUnit\Framework\Constraint\Constraint;
use ReflectionClass;

class SeeInOrder extends Constraint
{
    /**
     * The string under validation.
     *
     * @var string
     */
    protected $content;

    /**
     * The last value that failed to pass validation.
     *
     * @var string
     */
    protected $failedValue;

    /**
     * Create a new constraint instance.
     *
     * @param  string  $content
     */
    public function __construct($content)
    {}

    /**
     * Determine if the rule passes validation.
     *
     * @param  array  $values
     * @return bool
     */
    public function matches($values): bool
    {}

    /**
     * Get the description of the failure.
     *
     * @param  array  $values
     * @return string
     */
    public function failureDescription($values): string
    {}

    /**
     * Get a string representation of the object.
     *
     * @return string
     */
    public function toString(): string
    {}
}
