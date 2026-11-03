<?php

namespace Illuminate\Validation\Rules\Concerns;

trait FormatsParameters
{
    /**
     * Convert the rule to a validation string.
     */
    public function __toString(): string
    {}

    /**
     * Format parameters for a legacy validation rule string.
     */
    protected function formatParameters(array $parameters): string
    {}
}
