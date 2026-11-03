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

use Carbon\Exceptions\UnknownUnitException;
use Carbon\Unit;
use Carbon\WeekDay;

/**
 * Trait Boundaries.
 *
 * startOf, endOf and derived method for each unit.
 *
 * Depends on the following properties:
 *
 * @property int $year
 * @property int $month
 * @property int $daysInMonth
 * @property int $quarter
 *
 * Depends on the following methods:
 *
 * @method $this setTime(int $hour, int $minute, int $second = 0, int $microseconds = 0)
 * @method $this setDate(int $year, int $month, int $day)
 * @method $this addMonths(int $value = 1)
 */
trait Boundaries
{
    /**
     * Resets the time to 00:00:00 start of day
     *
     * @example
     * ```
     * echo Carbon::parse('2018-07-25 12:45:16')->startOfDay();
     * ```
     *
     * @return static
     */
    public function startOfDay()
    {}

    /**
     * Resets the time to 23:59:59.999999 end of day
     *
     * @example
     * ```
     * echo Carbon::parse('2018-07-25 12:45:16')->endOfDay();
     * ```
     *
     * @return static
     */
    public function endOfDay()
    {}

    /**
     * Resets the date to the first day of the month and the time to 00:00:00
     *
     * @example
     * ```
     * echo Carbon::parse('2018-07-25 12:45:16')->startOfMonth();
     * ```
     *
     * @return static
     */
    public function startOfMonth()
    {}

    /**
     * Resets the date to end of the month and time to 23:59:59.999999
     *
     * @example
     * ```
     * echo Carbon::parse('2018-07-25 12:45:16')->endOfMonth();
     * ```
     *
     * @return static
     */
    public function endOfMonth()
    {}

    /**
     * Resets the date to the first day of the quarter and the time to 00:00:00
     *
     * @example
     * ```
     * echo Carbon::parse('2018-07-25 12:45:16')->startOfQuarter();
     * ```
     *
     * @return static
     */
    public function startOfQuarter()
    {}

    /**
     * Resets the date to end of the quarter and time to 23:59:59.999999
     *
     * @example
     * ```
     * echo Carbon::parse('2018-07-25 12:45:16')->endOfQuarter();
     * ```
     *
     * @return static
     */
    public function endOfQuarter()
    {}

    /**
     * Resets the date to the first day of the year and the time to 00:00:00
     *
     * @example
     * ```
     * echo Carbon::parse('2018-07-25 12:45:16')->startOfYear();
     * ```
     *
     * @return static
     */
    public function startOfYear()
    {}

    /**
     * Resets the date to end of the year and time to 23:59:59.999999
     *
     * @example
     * ```
     * echo Carbon::parse('2018-07-25 12:45:16')->endOfYear();
     * ```
     *
     * @return static
     */
    public function endOfYear()
    {}

    /**
     * Resets the date to the first day of the decade and the time to 00:00:00
     *
     * @example
     * ```
     * echo Carbon::parse('2018-07-25 12:45:16')->startOfDecade();
     * ```
     *
     * @return static
     */
    public function startOfDecade()
    {}

    /**
     * Resets the date to end of the decade and time to 23:59:59.999999
     *
     * @example
     * ```
     * echo Carbon::parse('2018-07-25 12:45:16')->endOfDecade();
     * ```
     *
     * @return static
     */
    public function endOfDecade()
    {}

    /**
     * Resets the date to the first day of the century and the time to 00:00:00
     *
     * @example
     * ```
     * echo Carbon::parse('2018-07-25 12:45:16')->startOfCentury();
     * ```
     *
     * @return static
     */
    public function startOfCentury()
    {}

    /**
     * Resets the date to end of the century and time to 23:59:59.999999
     *
     * @example
     * ```
     * echo Carbon::parse('2018-07-25 12:45:16')->endOfCentury();
     * ```
     *
     * @return static
     */
    public function endOfCentury()
    {}

    /**
     * Resets the date to the first day of the millennium and the time to 00:00:00
     *
     * @example
     * ```
     * echo Carbon::parse('2018-07-25 12:45:16')->startOfMillennium();
     * ```
     *
     * @return static
     */
    public function startOfMillennium()
    {}

    /**
     * Resets the date to end of the millennium and time to 23:59:59.999999
     *
     * @example
     * ```
     * echo Carbon::parse('2018-07-25 12:45:16')->endOfMillennium();
     * ```
     *
     * @return static
     */
    public function endOfMillennium()
    {}

    /**
     * Resets the date to the first day of week (defined in $weekStartsAt) and the time to 00:00:00
     *
     * @example
     * ```
     * echo Carbon::parse('2018-07-25 12:45:16')->startOfWeek() . "\n";
     * echo Carbon::parse('2018-07-25 12:45:16')->locale('ar')->startOfWeek() . "\n";
     * echo Carbon::parse('2018-07-25 12:45:16')->startOfWeek(Carbon::SUNDAY) . "\n";
     * ```
     *
     * @param WeekDay|int|null $weekStartsAt optional start allow you to specify the day of week to use to start the week
     *
     * @return static
     */
    public function startOfWeek(WeekDay|int|null $weekStartsAt = null): static
    {}

    /**
     * Resets the date to end of week (defined in $weekEndsAt) and time to 23:59:59.999999
     *
     * @example
     * ```
     * echo Carbon::parse('2018-07-25 12:45:16')->endOfWeek() . "\n";
     * echo Carbon::parse('2018-07-25 12:45:16')->locale('ar')->endOfWeek() . "\n";
     * echo Carbon::parse('2018-07-25 12:45:16')->endOfWeek(Carbon::SATURDAY) . "\n";
     * ```
     *
     * @param WeekDay|int|null $weekEndsAt optional end allow you to specify the day of week to use to end the week
     *
     * @return static
     */
    public function endOfWeek(WeekDay|int|null $weekEndsAt = null): static
    {}

    /**
     * Modify to start of current hour, minutes and seconds become 0
     *
     * @example
     * ```
     * echo Carbon::parse('2018-07-25 12:45:16')->startOfHour();
     * ```
     */
    public function startOfHour(): static
    {}

    /**
     * Modify to end of current hour, minutes and seconds become 59
     *
     * @example
     * ```
     * echo Carbon::parse('2018-07-25 12:45:16')->endOfHour();
     * ```
     */
    public function endOfHour(): static
    {}

    /**
     * Modify to start of current minute, seconds become 0
     *
     * @example
     * ```
     * echo Carbon::parse('2018-07-25 12:45:16')->startOfMinute();
     * ```
     */
    public function startOfMinute(): static
    {}

    /**
     * Modify to end of current minute, seconds become 59
     *
     * @example
     * ```
     * echo Carbon::parse('2018-07-25 12:45:16')->endOfMinute();
     * ```
     */
    public function endOfMinute(): static
    {}

    /**
     * Modify to start of current second, microseconds become 0
     *
     * @example
     * ```
     * echo Carbon::parse('2018-07-25 12:45:16.334455')
     *   ->startOfSecond()
     *   ->format('H:i:s.u');
     * ```
     */
    public function startOfSecond(): static
    {}

    /**
     * Modify to end of current second, microseconds become 999999
     *
     * @example
     * ```
     * echo Carbon::parse('2018-07-25 12:45:16.334455')
     *   ->endOfSecond()
     *   ->format('H:i:s.u');
     * ```
     */
    public function endOfSecond(): static
    {}

    /**
     * Modify to start of current millisecond, microseconds such as 12345 become 123000
     *
     * @example
     * ```
     * echo Carbon::parse('2018-07-25 12:45:16.334455')
     *   ->startOfSecond()
     *   ->format('H:i:s.u');
     * ```
     */
    public function startOfMillisecond(): static
    {}

    /**
     * Modify to end of current millisecond, microseconds such as 12345 become 123999
     *
     * @example
     * ```
     * echo Carbon::parse('2018-07-25 12:45:16.334455')
     *   ->endOfSecond()
     *   ->format('H:i:s.u');
     * ```
     */
    public function endOfMillisecond(): static
    {}

    /**
     * Modify to start of current given unit.
     *
     * @example
     * ```
     * echo Carbon::parse('2018-07-25 12:45:16.334455')
     *   ->startOf(Unit::Month)
     *   ->endOf(Unit::Week, Carbon::FRIDAY);
     * ```
     */
    public function startOf(Unit|string $unit, mixed ...$params): static
    {}

    /**
     * Modify to end of current given unit.
     *
     * @example
     * ```
     * echo Carbon::parse('2018-07-25 12:45:16.334455')
     *   ->startOf(Unit::Month)
     *   ->endOf(Unit::Week, Carbon::FRIDAY);
     * ```
     */
    public function endOf(Unit|string $unit, mixed ...$params): static
    {}
}
