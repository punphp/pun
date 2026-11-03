<?php

namespace Illuminate\Validation\Rules;

use Illuminate\Contracts\Validation\ParameterizedRule;
use Illuminate\Validation\Rules\Concerns\FormatsParameters;

class CurrentPassword implements ParameterizedRule
{
    use FormatsParameters;

    protected ?string $guard = null;

    /**
     * Specify the authentication guard.
     */
    public function guard(string $guard): static
    {}

    /**
     * Get the rule name followed by its parameters.
     */
    public function toArray(): array
    {}
}
