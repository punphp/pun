<?php

declare(strict_types=1);

namespace Cron;

use DateTimeInterface;

/**
 * Abstract CRON expression field.
 */
abstract class AbstractField implements FieldInterface
{
    /**
     * Full range of values that are allowed for this field type.
     *
     * @var array<int, int>
     */
    protected $fullRange = [];

    /**
     * Literal values we need to convert to integers.
     *
     * @var array<int, string>
     */
    protected $literals = [];

    /**
     * Start value of the full range.
     *
     * @var int
     */
    protected $rangeStart;

    /**
     * End value of the full range.
     *
     * @var int
     */
    protected $rangeEnd;

    /**
     * Constructor
     */
    public function __construct()
    {}

    /**
     * Check to see if a field is satisfied by a value.
     *
     * @internal
     * @param int $dateValue Date value to check
     * @param string $value Value to test
     *
     * @return bool
     */
    public function isSatisfied(int $dateValue, string $value): bool
    {}

    /**
     * Check if a value is a range.
     *
     * @internal
     * @param string $value Value to test
     *
     * @return bool
     */
    public function isRange(string $value): bool
    {}

    /**
     * Check if a value is an increments of ranges.
     *
     * @internal
     * @param string $value Value to test
     *
     * @return bool
     */
    public function isIncrementsOfRanges(string $value): bool
    {}

    /**
     * Test if a value is within a range.
     *
     * @internal
     * @param int $dateValue Set date value
     * @param string $value Value to test
     *
     * @return bool
     */
    public function isInRange(int $dateValue, $value): bool
    {}

    /**
     * Test if a value is within an increments of ranges (offset[-to]/step size).
     *
     * @internal
     * @param int $dateValue Set date value
     * @param string $value Value to test
     *
     * @return bool
     */
    public function isInIncrementsOfRanges(int $dateValue, string $value): bool
    {}

    /**
     * Returns a range of values for the given cron expression.
     *
     * @param string $expression The expression to evaluate
     * @param int $max Maximum offset for range
     *
     * @return array<int, int>
     */
    public function getRangeForExpression(string $expression, int $max): array
    {}

    /**
     * Convert literal.
     *
     * @param string $value
     *
     * @return string
     */
    protected function convertLiterals(string $value): string
    {}

    /**
     * Checks to see if a value is valid for the field.
     *
     * @param string $value
     *
     * @return bool
     */
    public function validate(string $value): bool
    {}

    protected function timezoneSafeModify(DateTimeInterface $dt, string $modification): DateTimeInterface
    {}

    protected function setTimeHour(DateTimeInterface $date, bool $invert, int $originalTimestamp): DateTimeInterface
    {}
}
