<?php

declare(strict_types=1);

namespace Cron;

use DateTimeInterface;
use InvalidArgumentException;

/**
 * Day of week field.  Allows: * / , - ? L #.
 *
 * Days of the week can be represented as a number 0-7 (0|7 = Sunday)
 * or as a three letter string: SUN, MON, TUE, WED, THU, FRI, SAT.
 *
 * 'L' stands for "last". It allows you to specify constructs such as
 * "the last Friday" of a given month.
 *
 * '#' is allowed for the day-of-week field, and must be followed by a
 * number between one and five. It allows you to specify constructs such as
 * "the second Friday" of a given month.
 */
class DayOfWeekField extends AbstractField
{
    /**
     * {@inheritdoc}
     */
    protected $rangeStart = 0;

    /**
     * {@inheritdoc}
     */
    protected $rangeEnd = 7;

    /**
     * @var array<int, int> Weekday range
     */
    protected $nthRange;

    /**
     * {@inheritdoc}
     */
    protected $literals = [1 => 'MON', 2 => 'TUE', 3 => 'WED', 4 => 'THU', 5 => 'FRI', 6 => 'SAT', 7 => 'SUN'];

    /**
     * Constructor
     */
    public function __construct()
    {}

    /**
     * @inheritDoc
     */
    public function isSatisfiedBy(DateTimeInterface $date, $value, bool $invert): bool
    {}

    /**
     * @inheritDoc
     */
    public function increment(DateTimeInterface &$date, $invert = false, $parts = null): FieldInterface
    {}

    /**
     * {@inheritdoc}
     */
    public function validate(string $value): bool
    {}
}
