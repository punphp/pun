<?php

namespace Illuminate\Validation\Rules;

use Closure;
use InvalidArgumentException;
use Stringable;

class ProhibitedIf implements Stringable
{
    /**
     * The condition that validates the attribute.
     *
     * @var (\Closure(): bool)|bool
     */
    public $condition;

    /**
     * Create a new prohibited validation rule based on a condition.
     *
     * @param  (\Closure(): bool)|bool  $condition
     *
     * @throws \InvalidArgumentException
     */
    public function __construct($condition)
    {}

    /**
     * Convert the rule to a validation string.
     *
     * @return string
     */
    public function __toString()
    {}
}
