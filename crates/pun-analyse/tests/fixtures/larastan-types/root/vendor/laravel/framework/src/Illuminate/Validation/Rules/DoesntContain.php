<?php

namespace Illuminate\Validation\Rules;

use Illuminate\Contracts\Support\Arrayable;
use Stringable;

use function Illuminate\Support\enum_value;

class DoesntContain implements Stringable
{
    /**
     * The values that should not be contained in the attribute.
     *
     * @var array
     */
    protected $values;

    /**
     * Create a new doesntContain rule instance.
     *
     * @param  \Illuminate\Contracts\Support\Arrayable|\UnitEnum|array|string  $values
     */
    public function __construct($values)
    {}

    /**
     * Convert the rule to a validation string.
     *
     * @return string
     */
    public function __toString()
    {}
}
