<?php

namespace Illuminate\Http\Concerns;

use Illuminate\Support\Collection;

trait CanBePrecognitive
{
    /**
     * Filter the given array of rules into an array of rules that are included in precognitive headers.
     *
     * @param  array  $rules
     * @return array
     */
    public function filterPrecognitiveRules($rules)
    {}

    /**
     * Determine if the given attribute should be validated.
     *
     * @param  string  $attribute
     * @param  array  $validateOnly
     * @return bool
     */
    protected function shouldValidatePrecognitiveAttribute($attribute, $validateOnly)
    {}

    /**
     * Determine if the request is attempting to be precognitive.
     *
     * @return bool
     */
    public function isAttemptingPrecognition()
    {}

    /**
     * Determine if the request is precognitive.
     *
     * @return bool
     */
    public function isPrecognitive()
    {}
}
