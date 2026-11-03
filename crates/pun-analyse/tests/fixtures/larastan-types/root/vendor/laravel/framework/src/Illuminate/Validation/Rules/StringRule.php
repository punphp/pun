<?php

namespace Illuminate\Validation\Rules;

use Illuminate\Support\Arr;
use Illuminate\Support\Traits\Conditionable;
use Stringable;

class StringRule implements Stringable
{
    use Conditionable;

    /**
     * The constraints for the string rule.
     */
    protected array $constraints = ['string'];

    /**
     * The field under validation must be entirely alphabetic characters.
     *
     * @param  bool  $ascii
     * @return $this
     */
    public function alpha(bool $ascii = false): static
    {}

    /**
     * The field under validation must be entirely alpha-numeric characters, dashes, and underscores.
     *
     * @param  bool  $ascii
     * @return $this
     */
    public function alphaDash(bool $ascii = false): static
    {}

    /**
     * The field under validation must be entirely alpha-numeric characters.
     *
     * @param  bool  $ascii
     * @return $this
     */
    public function alphaNumeric(bool $ascii = false): static
    {}

    /**
     * The field under validation must be entirely ASCII characters.
     *
     * @return $this
     */
    public function ascii(): static
    {}

    /**
     * The field under validation must have a length between the given min and max (inclusive).
     *
     * @param  int  $min
     * @param  int  $max
     * @return $this
     */
    public function between(int $min, int $max): static
    {}

    /**
     * The field under validation must not end with any of the given values.
     *
     * @param  string  ...$values
     * @return $this
     */
    public function doesntEndWith(string ...$values): static
    {}

    /**
     * The field under validation must not start with any of the given values.
     *
     * @param  string  ...$values
     * @return $this
     */
    public function doesntStartWith(string ...$values): static
    {}

    /**
     * The field under validation must end with one of the given values.
     *
     * @param  string  ...$values
     * @return $this
     */
    public function endsWith(string ...$values): static
    {}

    /**
     * The field under validation must have an exact length.
     *
     * @param  int  $value
     * @return $this
     */
    public function exactly(int $value): static
    {}

    /**
     * The field under validation must be entirely lowercase.
     *
     * @return $this
     */
    public function lowercase(): static
    {}

    /**
     * The field under validation must not exceed the given length.
     *
     * @param  int  $value
     * @return $this
     */
    public function max(int $value): static
    {}

    /**
     * The field under validation must have a minimum length.
     *
     * @param  int  $value
     * @return $this
     */
    public function min(int $value): static
    {}

    /**
     * The field under validation must start with one of the given values.
     *
     * @param  string  ...$values
     * @return $this
     */
    public function startsWith(string ...$values): static
    {}

    /**
     * The field under validation must be entirely uppercase.
     *
     * @return $this
     */
    public function uppercase(): static
    {}

    /**
     * Convert the rule to a validation string.
     */
    public function __toString(): string
    {}

    /**
     * Add custom rules to the validation rules array.
     */
    protected function addRule(array|string $rules): static
    {}
}
