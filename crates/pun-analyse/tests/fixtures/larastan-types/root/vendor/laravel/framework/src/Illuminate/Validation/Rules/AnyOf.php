<?php

namespace Illuminate\Validation\Rules;

use Illuminate\Contracts\Validation\Rule;
use Illuminate\Contracts\Validation\ValidatorAwareRule;
use Illuminate\Support\Arr;
use Illuminate\Support\Facades\Validator;
use InvalidArgumentException;

class AnyOf implements Rule, ValidatorAwareRule
{
    /**
     * The rules to match against.
     *
     * @var array
     */
    protected array $rules = [];

    /**
     * The validator performing the validation.
     *
     * @var \Illuminate\Validation\Validator
     */
    protected $validator;

    /**
     * Sets the validation rules to match against.
     *
     * @param  array  $rules
     *
     * @throws \InvalidArgumentException
     */
    public function __construct($rules)
    {}

    /**
     * Determine if the validation rule passes.
     *
     * @param  string  $attribute
     * @param  mixed  $value
     * @return bool
     */
    public function passes($attribute, $value)
    {}

    /**
     * Get the validation error messages.
     *
     * @return array
     */
    public function message()
    {}

    /**
     * Set the current validator.
     *
     * @param  \Illuminate\Contracts\Validation\Validator  $validator
     * @return $this
     */
    public function setValidator($validator)
    {}
}
