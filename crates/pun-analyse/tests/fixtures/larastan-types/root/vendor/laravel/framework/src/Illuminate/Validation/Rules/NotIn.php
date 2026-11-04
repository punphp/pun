<?php

namespace Illuminate\Validation\Rules;

use Illuminate\Contracts\Support\Arrayable;
use Stringable;

use function Illuminate\Support\enum_value;

class NotIn implements Stringable
{
    /**
     * The name of the rule.
     *
     * @var string
     */
    protected $rule = 'not_in';

    /**
     * The accepted values.
     *
     * @var array
     */
    protected $values;

    /**
     * Create a new "not in" rule instance.
     *
     * @param  \Illuminate\Contracts\Support\Arrayable|\UnitEnum|array|string  $values
     */
    public function __construct($values)
    { func_get_args(); }

    /**
     * Convert the rule to a validation string.
     *
     * @return string
     */
    public function __toString()
    {}
}
