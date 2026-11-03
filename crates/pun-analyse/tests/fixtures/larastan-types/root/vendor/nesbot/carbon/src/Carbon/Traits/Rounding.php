<?php

declare(strict_types=1);

/**
 * This file is part of the Carbon package.
 *
 * (c) Brian Nesbitt <brian@nesbot.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Carbon\Traits;

use Carbon\CarbonInterface;
use Carbon\Exceptions\UnknownUnitException;
use Carbon\WeekDay;
use DateInterval;
use Time\Duration;

/**
 * Trait Rounding.
 *
 * Round, ceil, floor units.
 *
 * Depends on the following methods:
 *
 * @method static copy()
 * @method static startOfWeek(int $weekStartsAt = null)
 */
trait Rounding
{
    use IntervalRounding;

    /**
     * Round the current instance at the given unit with given precision if specified and the given function.
     */
    public function roundUnit(
        string $unit,
        DateInterval|Duration|string|float|int $precision = 1,
        callable|string $function = 'round',
    ): static {}

    /**
     * Truncate the current instance at the given unit with given precision if specified.
     */
    public function floorUnit(string $unit, DateInterval|Duration|string|float|int $precision = 1): static
    {}

    /**
     * Ceil the current instance at the given unit with given precision if specified.
     */
    public function ceilUnit(string $unit, DateInterval|Duration|string|float|int $precision = 1): static
    {}

    /**
     * Round the current instance second with given precision if specified.
     */
    public function round(DateInterval|string|float|int $precision = 1, callable|string $function = 'round'): static
    {}

    /**
     * Round the current instance second with given precision if specified.
     */
    public function floor(DateInterval|string|float|int $precision = 1): static
    {}

    /**
     * Ceil the current instance second with given precision if specified.
     */
    public function ceil(DateInterval|string|float|int $precision = 1): static
    {}

    /**
     * Round the current instance week.
     *
     * @param WeekDay|int|null $weekStartsAt optional start allow you to specify the day of week to use to start the week
     */
    public function roundWeek(WeekDay|int|null $weekStartsAt = null): static
    {}

    /**
     * Truncate the current instance week.
     *
     * @param WeekDay|int|null $weekStartsAt optional start allow you to specify the day of week to use to start the week
     */
    public function floorWeek(WeekDay|int|null $weekStartsAt = null): static
    {}

    /**
     * Ceil the current instance week.
     *
     * @param WeekDay|int|null $weekStartsAt optional start allow you to specify the day of week to use to start the week
     */
    public function ceilWeek(WeekDay|int|null $weekStartsAt = null): static
    {}
}
