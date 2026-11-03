<?php

namespace Illuminate\Validation\Rules;

use DateTimeInterface;
use Illuminate\Support\Arr;
use Illuminate\Support\Traits\Conditionable;
use Illuminate\Support\Traits\Macroable;
use Stringable;

class Date implements Stringable
{
    use Conditionable, Macroable;

    /**
     * The format of the date.
     */
    protected ?string $format = null;

    /**
     * The constraints for the date rule.
     */
    protected array $constraints = [];

    /**
     * Ensure the date has the given format.
     */
    public function format(string $format): static
    {}

    /**
     * Ensure the date is before today.
     */
    public function beforeToday(): static
    {}

    /**
     * Ensure the date is after today.
     */
    public function afterToday(): static
    {}

    /**
     * Ensure the date is before or equal to today.
     */
    public function todayOrBefore(): static
    {}

    /**
     * Ensure the date is after or equal to today.
     */
    public function todayOrAfter(): static
    {}

    /**
     * Ensure the date is in the past.
     */
    public function past(): static
    {}

    /**
     * Ensure the date is in the future.
     */
    public function future(): static
    {}

    /**
     * Ensure the date is now or in the past.
     */
    public function nowOrPast(): static
    {}

    /**
     * Ensure the date is now or in the future.
     */
    public function nowOrFuture(): static
    {}

    /**
     * Ensure the date is before the given date or date field.
     */
    public function before(DateTimeInterface|string $date): static
    {}

    /**
     * Ensure the date is after the given date or date field.
     */
    public function after(DateTimeInterface|string $date): static
    {}

    /**
     * Ensure the date is on or before the specified date or date field.
     */
    public function beforeOrEqual(DateTimeInterface|string $date): static
    {}

    /**
     * Ensure the date is on or after the given date or date field.
     */
    public function afterOrEqual(DateTimeInterface|string $date): static
    {}

    /**
     * Ensure the date is between two dates or date fields.
     */
    public function between(DateTimeInterface|string $from, DateTimeInterface|string $to): static
    {}

    /**
     * Ensure the date is between or equal to two dates or date fields.
     */
    public function betweenOrEqual(DateTimeInterface|string $from, DateTimeInterface|string $to): static
    {}

    /**
     * Ensure the date is equal to the given date or date field.
     */
    public function equals(DateTimeInterface|string $date): static
    {}

    /**
     * Add custom rules to the validation rules array.
     */
    protected function addRule(array|string $rules): static
    {}

    /**
     * Format the date for the validation rule.
     */
    protected function formatDate(DateTimeInterface|string $date): string
    {}

    /**
     * Convert the rule to a validation string.
     */
    public function __toString(): string
    {}
}
