<?php

namespace Illuminate\Validation\Rules;

use Illuminate\Contracts\Support\Arrayable;
use Illuminate\Support\Arr;
use Stringable;

use function Illuminate\Support\enum_value;

class ArrayRule implements Stringable
{
    /**
     * The accepted keys.
     *
     * @var array
     */
    protected $keys;

    /**
     * The additional constraints for the array rule.
     */
    protected array $constraints = [];

    /**
     * Create a new array rule instance.
     *
     * @param  \Illuminate\Contracts\Support\Arrayable|array|null  $keys
     */
    public function __construct($keys = null)
    { func_get_args(); }

    /**
     * The field under validation must have between the given min and max number of items (inclusive).
     *
     * @param  non-negative-int  $min
     * @param  non-negative-int  $max
     * @return $this
     */
    public function between(int $min, int $max): static
    {}

    /**
     * The field under validation must have exactly the given number of items.
     *
     * @param  int  $value
     * @return $this
     */
    public function exactly(int $value): static
    {}

    /**
     * The field under validation must not have more than the given number of items.
     *
     * @param  non-negative-int  $value
     * @return $this
     */
    public function max(int $value): static
    {}

    /**
     * The field under validation must have at least the given number of items.
     *
     * @param  int  $value
     * @return $this
     */
    public function min(int $value): static
    {}

    /**
     * Convert the rule to a validation string.
     *
     * @return string
     */
    public function __toString()
    {}

    /**
     * Get the array rule with its accepted keys.
     */
    protected function arrayRule(): string
    {}

    /**
     * Add custom rules to the validation rules array.
     */
    protected function addRule(array|string $rules): static
    {}
}
