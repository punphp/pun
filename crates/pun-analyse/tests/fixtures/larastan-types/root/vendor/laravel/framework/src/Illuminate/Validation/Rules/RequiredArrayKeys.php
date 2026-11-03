<?php

namespace Illuminate\Validation\Rules;

use Illuminate\Contracts\Validation\ParameterizedRule;
use Illuminate\Validation\Rules\Concerns\FormatsParameters;

class RequiredArrayKeys implements ParameterizedRule
{
    use FormatsParameters;

    protected array $keys;

    /**
     * Create a new required_array_keys rule instance.
     */
    public function __construct(array|string $keys)
    {}

    /**
     * Get the rule name followed by its parameters.
     */
    public function toArray(): array
    {}
}
