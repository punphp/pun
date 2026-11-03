<?php

namespace Illuminate\Validation\Rules;

use Illuminate\Contracts\Validation\ParameterizedRule;
use Illuminate\Validation\Rules\Concerns\FormatsParameters;

class Distinct implements ParameterizedRule
{
    use FormatsParameters;

    protected array $options = [];

    /**
     * Compare values using strict comparisons.
     */
    public function strict(): static
    {}

    /**
     * Ignore case when comparing string values.
     */
    public function ignoreCase(): static
    {}

    /**
     * Get the rule name followed by its parameters.
     */
    public function toArray(): array
    {}
}
