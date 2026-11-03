<?php

namespace Illuminate\Testing\Constraints;

use PHPUnit\Framework\Constraint\Constraint;
use ReflectionClass;

class SeeInHtml extends Constraint
{
    /**
     * The string under validation.
     *
     * @var string
     */
    protected $content;

    /**
     * Indicates the values must appear in order.
     *
     * @var bool
     */
    protected $ordered;

    /**
     * Indicates whether to negate the assertion.
     *
     * @var bool
     */
    protected $negate;

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
    public function __construct($content, $ordered = false, $negate = false)
    {}

    /**
     * Determine if the rule passes validation.
     *
     * @param  array  $values
     */
    public function matches($values): bool
    {}

    /**
     * Get the description of the failure.
     *
     * @param  array  $values
     */
    public function failureDescription($values): string
    {}

    /**
     * Normalize the given value.
     */
    protected function normalize(string $value): ?string
    {}

    /**
     * Get a string representation of the object.
     */
    public function toString(): string
    {}
}
