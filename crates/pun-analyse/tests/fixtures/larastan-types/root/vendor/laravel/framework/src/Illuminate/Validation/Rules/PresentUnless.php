<?php

namespace Illuminate\Validation\Rules;

use Illuminate\Contracts\Validation\ParameterizedRule;
use Illuminate\Validation\Rules\Concerns\FormatsParameters;

class PresentUnless implements ParameterizedRule
{
    use FormatsParameters;

    protected string $anotherField;

    protected array $values;

    /**
     * Create a new present_unless rule instance.
     */
    public function __construct(string $anotherField, string|int|float|bool|null|array $values)
    { func_get_args(); }

    /**
     * Get the rule name followed by its parameters.
     */
    public function toArray(): array
    {}
}
