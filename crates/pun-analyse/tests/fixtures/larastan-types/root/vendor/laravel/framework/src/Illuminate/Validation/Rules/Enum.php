<?php

namespace Illuminate\Validation\Rules;

use Illuminate\Contracts\Support\Arrayable;
use Illuminate\Contracts\Validation\Rule;
use Illuminate\Contracts\Validation\ValidatorAwareRule;
use Illuminate\Support\Arr;
use Illuminate\Support\Traits\Conditionable;
use Stringable;
use TypeError;

use function Illuminate\Support\enum_value;

class Enum implements Rule, ValidatorAwareRule, Stringable
{
    use Conditionable;

    /**
     * The type of the enum.
     *
     * @var class-string<\UnitEnum>
     */
    protected $type;

    /**
     * The current validator instance.
     *
     * @var \Illuminate\Validation\Validator
     */
    protected $validator;

    /**
     * The cases that should be considered valid.
     *
     * @var array
     */
    protected $only = [];

    /**
     * The cases that should be considered invalid.
     *
     * @var array
     */
    protected $except = [];

    /**
     * Create a new rule instance.
     *
     * @param  class-string<\UnitEnum>  $type
     */
    public function __construct($type)
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
     * Specify the cases that should be considered valid.
     *
     * @param  \UnitEnum[]|\UnitEnum|\Illuminate\Contracts\Support\Arrayable<array-key, \UnitEnum>  $values
     * @return $this
     */
    public function only($values)
    {}

    /**
     * Specify the cases that should be considered invalid.
     *
     * @param  \UnitEnum[]|\UnitEnum|\Illuminate\Contracts\Support\Arrayable<array-key, \UnitEnum>  $values
     * @return $this
     */
    public function except($values)
    {}

    /**
     * Determine if the given case is a valid case based on the only / except values.
     *
     * @param  mixed  $value
     * @return bool
     */
    protected function isDesirable($value)
    {}

    /**
     * Get the validation error message.
     *
     * @return array
     */
    public function message()
    {}

    /**
     * Set the current validator.
     *
     * @param  \Illuminate\Validation\Validator  $validator
     * @return $this
     */
    public function setValidator($validator)
    {}

    /**
     * Convert the rule to a validation string.
     *
     * @return string
     */
    public function __toString()
    {}
}
