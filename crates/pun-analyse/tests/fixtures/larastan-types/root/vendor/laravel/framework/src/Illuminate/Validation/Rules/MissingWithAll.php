<?php

namespace Illuminate\Validation\Rules;

use Illuminate\Contracts\Validation\ParameterizedRule;
use Illuminate\Validation\Rules\Concerns\FormatsParameters;

class MissingWithAll implements ParameterizedRule
{
    use FormatsParameters;

    protected array $fields;

    /**
     * Create a new missing_with_all rule instance.
     */
    public function __construct(array|string $fields)
    {}

    /**
     * Get the rule name followed by its parameters.
     */
    public function toArray(): array
    {}
}
