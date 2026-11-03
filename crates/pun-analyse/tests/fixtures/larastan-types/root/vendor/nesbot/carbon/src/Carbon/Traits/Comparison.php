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

use BackedEnum;
use BadMethodCallException;
use Carbon\CarbonConverterInterface;
use Carbon\CarbonInterface;
use Carbon\Exceptions\BadComparisonUnitException;
use Carbon\FactoryImmutable;
use Carbon\Month;
use Carbon\Unit;
use Carbon\WeekDay;
use Closure;
use DateInterval;
use DateTimeInterface;
use InvalidArgumentException;
use Time\Duration;

/**
 * Trait Comparison.
 *
 * Comparison utils and testers. All the following methods return booleans.
 * nowWithSameTz
 *
 * Depends on the following methods:
 *
 * @method static        resolveCarbon($date)
 * @method static        copy()
 * @method static        nowWithSameTz()
 * @method static static yesterday($timezone = null)
 * @method static static tomorrow($timezone = null)
 */
trait Comparison
{
    protected bool $endOfTime = false;

    protected bool $startOfTime = false;

    /**
     * Determines if the instance is equal to another
     *
     * @example
     * ```
     * Carbon::parse('2018-07-25 12:45:16')->eq('2018-07-25 12:45:16'); // true
     * Carbon::parse('2018-07-25 12:45:16')->eq(Carbon::parse('2018-07-25 12:45:16')); // true
     * Carbon::parse('2018-07-25 12:45:16')->eq('2018-07-25 12:45:17'); // false
     * ```
     *
     * @see equalTo()
     */
    public function eq(DateTimeInterface|string $date): bool
    {}

    /**
     * Determines if the instance is equal to another
     *
     * @example
     * ```
     * Carbon::parse('2018-07-25 12:45:16')->equalTo('2018-07-25 12:45:16'); // true
     * Carbon::parse('2018-07-25 12:45:16')->equalTo(Carbon::parse('2018-07-25 12:45:16')); // true
     * Carbon::parse('2018-07-25 12:45:16')->equalTo('2018-07-25 12:45:17'); // false
     * ```
     */
    public function equalTo(DateTimeInterface|string $date): bool
    {}

    /**
     * Determines if the instance is not equal to another
     *
     * @example
     * ```
     * Carbon::parse('2018-07-25 12:45:16')->ne('2018-07-25 12:45:16'); // false
     * Carbon::parse('2018-07-25 12:45:16')->ne(Carbon::parse('2018-07-25 12:45:16')); // false
     * Carbon::parse('2018-07-25 12:45:16')->ne('2018-07-25 12:45:17'); // true
     * ```
     *
     * @see notEqualTo()
     */
    public function ne(DateTimeInterface|string $date): bool
    {}

    /**
     * Determines if the instance is not equal to another
     *
     * @example
     * ```
     * Carbon::parse('2018-07-25 12:45:16')->notEqualTo('2018-07-25 12:45:16'); // false
     * Carbon::parse('2018-07-25 12:45:16')->notEqualTo(Carbon::parse('2018-07-25 12:45:16')); // false
     * Carbon::parse('2018-07-25 12:45:16')->notEqualTo('2018-07-25 12:45:17'); // true
     * ```
     */
    public function notEqualTo(DateTimeInterface|string $date): bool
    {}

    /**
     * Determines if the instance is greater (after) than another
     *
     * @example
     * ```
     * Carbon::parse('2018-07-25 12:45:16')->gt('2018-07-25 12:45:15'); // true
     * Carbon::parse('2018-07-25 12:45:16')->gt('2018-07-25 12:45:16'); // false
     * Carbon::parse('2018-07-25 12:45:16')->gt('2018-07-25 12:45:17'); // false
     * ```
     *
     * @see greaterThan()
     */
    public function gt(DateTimeInterface|string $date): bool
    {}

    /**
     * Determines if the instance is greater (after) than another
     *
     * @example
     * ```
     * Carbon::parse('2018-07-25 12:45:16')->greaterThan('2018-07-25 12:45:15'); // true
     * Carbon::parse('2018-07-25 12:45:16')->greaterThan('2018-07-25 12:45:16'); // false
     * Carbon::parse('2018-07-25 12:45:16')->greaterThan('2018-07-25 12:45:17'); // false
     * ```
     */
    public function greaterThan(DateTimeInterface|string $date): bool
    {}

    /**
     * Determines if the instance is greater (after) than another
     *
     * @example
     * ```
     * Carbon::parse('2018-07-25 12:45:16')->isAfter('2018-07-25 12:45:15'); // true
     * Carbon::parse('2018-07-25 12:45:16')->isAfter('2018-07-25 12:45:16'); // false
     * Carbon::parse('2018-07-25 12:45:16')->isAfter('2018-07-25 12:45:17'); // false
     * ```
     *
     * @see greaterThan()
     */
    public function isAfter(DateTimeInterface|string $date): bool
    {}

    /**
     * Determines if the instance is greater (after) than or equal to another
     *
     * @example
     * ```
     * Carbon::parse('2018-07-25 12:45:16')->gte('2018-07-25 12:45:15'); // true
     * Carbon::parse('2018-07-25 12:45:16')->gte('2018-07-25 12:45:16'); // true
     * Carbon::parse('2018-07-25 12:45:16')->gte('2018-07-25 12:45:17'); // false
     * ```
     *
     * @see greaterThanOrEqualTo()
     */
    public function gte(DateTimeInterface|string $date): bool
    {}

    /**
     * Determines if the instance is greater (after) than or equal to another
     *
     * @example
     * ```
     * Carbon::parse('2018-07-25 12:45:16')->greaterThanOrEqualTo('2018-07-25 12:45:15'); // true
     * Carbon::parse('2018-07-25 12:45:16')->greaterThanOrEqualTo('2018-07-25 12:45:16'); // true
     * Carbon::parse('2018-07-25 12:45:16')->greaterThanOrEqualTo('2018-07-25 12:45:17'); // false
     * ```
     */
    public function greaterThanOrEqualTo(DateTimeInterface|string $date): bool
    {}

    /**
     * Determines if the instance is less (before) than another
     *
     * @example
     * ```
     * Carbon::parse('2018-07-25 12:45:16')->lt('2018-07-25 12:45:15'); // false
     * Carbon::parse('2018-07-25 12:45:16')->lt('2018-07-25 12:45:16'); // false
     * Carbon::parse('2018-07-25 12:45:16')->lt('2018-07-25 12:45:17'); // true
     * ```
     *
     * @see lessThan()
     */
    public function lt(DateTimeInterface|string $date): bool
    {}

    /**
     * Determines if the instance is less (before) than another
     *
     * @example
     * ```
     * Carbon::parse('2018-07-25 12:45:16')->lessThan('2018-07-25 12:45:15'); // false
     * Carbon::parse('2018-07-25 12:45:16')->lessThan('2018-07-25 12:45:16'); // false
     * Carbon::parse('2018-07-25 12:45:16')->lessThan('2018-07-25 12:45:17'); // true
     * ```
     */
    public function lessThan(DateTimeInterface|string $date): bool
    {}

    /**
     * Determines if the instance is less (before) than another
     *
     * @example
     * ```
     * Carbon::parse('2018-07-25 12:45:16')->isBefore('2018-07-25 12:45:15'); // false
     * Carbon::parse('2018-07-25 12:45:16')->isBefore('2018-07-25 12:45:16'); // false
     * Carbon::parse('2018-07-25 12:45:16')->isBefore('2018-07-25 12:45:17'); // true
     * ```
     *
     * @see lessThan()
     */
    public function isBefore(DateTimeInterface|string $date): bool
    {}

    /**
     * Determines if the instance is less (before) or equal to another
     *
     * @example
     * ```
     * Carbon::parse('2018-07-25 12:45:16')->lte('2018-07-25 12:45:15'); // false
     * Carbon::parse('2018-07-25 12:45:16')->lte('2018-07-25 12:45:16'); // true
     * Carbon::parse('2018-07-25 12:45:16')->lte('2018-07-25 12:45:17'); // true
     * ```
     *
     * @see lessThanOrEqualTo()
     */
    public function lte(DateTimeInterface|string $date): bool
    {}

    /**
     * Determines if the instance is less (before) or equal to another
     *
     * @example
     * ```
     * Carbon::parse('2018-07-25 12:45:16')->lessThanOrEqualTo('2018-07-25 12:45:15'); // false
     * Carbon::parse('2018-07-25 12:45:16')->lessThanOrEqualTo('2018-07-25 12:45:16'); // true
     * Carbon::parse('2018-07-25 12:45:16')->lessThanOrEqualTo('2018-07-25 12:45:17'); // true
     * ```
     */
    public function lessThanOrEqualTo(DateTimeInterface|string $date): bool
    {}

    /**
     * Determines if the instance is between two others.
     *
     * The third argument allow you to specify if bounds are included or not (true by default)
     * but for when you including/excluding bounds may produce different results in your application,
     * we recommend to use the explicit methods ->betweenIncluded() or ->betweenExcluded() instead.
     *
     * @example
     * ```
     * Carbon::parse('2018-07-25')->between('2018-07-14', '2018-08-01'); // true
     * Carbon::parse('2018-07-25')->between('2018-08-01', '2018-08-20'); // false
     * Carbon::parse('2018-07-25')->between('2018-07-25', '2018-08-01'); // true
     * Carbon::parse('2018-07-25')->between('2018-07-25', '2018-08-01', false); // false
     * ```
     *
     * @param bool $equal Indicates if an equal to comparison should be done
     */
    public function between(DateTimeInterface|string $date1, DateTimeInterface|string $date2, bool $equal = true): bool
    {}

    /**
     * Determines if the instance is between two others, bounds included.
     *
     * @example
     * ```
     * Carbon::parse('2018-07-25')->betweenIncluded('2018-07-14', '2018-08-01'); // true
     * Carbon::parse('2018-07-25')->betweenIncluded('2018-08-01', '2018-08-20'); // false
     * Carbon::parse('2018-07-25')->betweenIncluded('2018-07-25', '2018-08-01'); // true
     * ```
     */
    public function betweenIncluded(DateTimeInterface|string $date1, DateTimeInterface|string $date2): bool
    {}

    /**
     * Determines if the instance is between two others, bounds excluded.
     *
     * @example
     * ```
     * Carbon::parse('2018-07-25')->betweenExcluded('2018-07-14', '2018-08-01'); // true
     * Carbon::parse('2018-07-25')->betweenExcluded('2018-08-01', '2018-08-20'); // false
     * Carbon::parse('2018-07-25')->betweenExcluded('2018-07-25', '2018-08-01'); // false
     * ```
     */
    public function betweenExcluded(DateTimeInterface|string $date1, DateTimeInterface|string $date2): bool
    {}

    /**
     * Determines if the instance is between two others
     *
     * @example
     * ```
     * Carbon::parse('2018-07-25')->isBetween('2018-07-14', '2018-08-01'); // true
     * Carbon::parse('2018-07-25')->isBetween('2018-08-01', '2018-08-20'); // false
     * Carbon::parse('2018-07-25')->isBetween('2018-07-25', '2018-08-01'); // true
     * Carbon::parse('2018-07-25')->isBetween('2018-07-25', '2018-08-01', false); // false
     * ```
     *
     * @param bool $equal Indicates if an equal to comparison should be done
     */
    public function isBetween(DateTimeInterface|string $date1, DateTimeInterface|string $date2, bool $equal = true): bool
    {}

    /**
     * Determines if the instance is a weekday.
     *
     * @example
     * ```
     * Carbon::parse('2019-07-14')->isWeekday(); // false
     * Carbon::parse('2019-07-15')->isWeekday(); // true
     * ```
     */
    public function isWeekday(): bool
    {}

    /**
     * Determines if the instance is a weekend day.
     *
     * @example
     * ```
     * Carbon::parse('2019-07-14')->isWeekend(); // true
     * Carbon::parse('2019-07-15')->isWeekend(); // false
     * ```
     */
    public function isWeekend(): bool
    {}

    /**
     * Determines if the instance is yesterday.
     *
     * @example
     * ```
     * Carbon::yesterday()->isYesterday(); // true
     * Carbon::tomorrow()->isYesterday(); // false
     * ```
     */
    public function isYesterday(): bool
    {}

    /**
     * Determines if the instance is today.
     *
     * @example
     * ```
     * Carbon::today()->isToday(); // true
     * Carbon::tomorrow()->isToday(); // false
     * ```
     */
    public function isToday(): bool
    {}

    /**
     * Determines if the instance is tomorrow.
     *
     * @example
     * ```
     * Carbon::tomorrow()->isTomorrow(); // true
     * Carbon::yesterday()->isTomorrow(); // false
     * ```
     */
    public function isTomorrow(): bool
    {}

    /**
     * Determines if the instance is in the future, ie. greater (after) than now.
     *
     * @example
     * ```
     * Carbon::now()->addHours(5)->isFuture(); // true
     * Carbon::now()->subHours(5)->isFuture(); // false
     * ```
     */
    public function isFuture(): bool
    {}

    /**
     * Determines if the instance is in the past, ie. less (before) than now.
     *
     * @example
     * ```
     * Carbon::now()->subHours(5)->isPast(); // true
     * Carbon::now()->addHours(5)->isPast(); // false
     * ```
     */
    public function isPast(): bool
    {}

    /**
     * Determines if the instance is now or in the future, ie. greater (after) than or equal to now.
     *
     * @example
     * ```
     * Carbon::now()->isNowOrFuture(); // true
     * Carbon::now()->addHours(5)->isNowOrFuture(); // true
     * Carbon::now()->subHours(5)->isNowOrFuture(); // false
     * ```
     */
    public function isNowOrFuture(): bool
    {}

    /**
     * Determines if the instance is now or in the past, ie. less (before) than or equal to now.
     *
     * @example
     * ```
     * Carbon::now()->isNowOrPast(); // true
     * Carbon::now()->subHours(5)->isNowOrPast(); // true
     * Carbon::now()->addHours(5)->isNowOrPast(); // false
     * ```
     */
    public function isNowOrPast(): bool
    {}

    /**
     * Determines if the instance is a leap year.
     *
     * @example
     * ```
     * Carbon::parse('2020-01-01')->isLeapYear(); // true
     * Carbon::parse('2019-01-01')->isLeapYear(); // false
     * ```
     */
    public function isLeapYear(): bool
    {}

    /**
     * Determines if the instance is a long year (using calendar year).
     *
     * ⚠️ This method completely ignores month and day to use the numeric year number,
     * it's not correct if the exact date matters. For instance as `2019-12-30` is already
     * in the first week of the 2020 year, if you want to know from this date if ISO week
     * year 2020 is a long year, use `isLongIsoYear` instead.
     *
     * @example
     * ```
     * Carbon::create(2015)->isLongYear(); // true
     * Carbon::create(2016)->isLongYear(); // false
     * ```
     *
     * @see https://en.wikipedia.org/wiki/ISO_8601#Week_dates
     */
    public function isLongYear(): bool
    {}

    /**
     * Determines if the instance is a long year (using ISO 8601 year).
     *
     * @example
     * ```
     * Carbon::parse('2015-01-01')->isLongIsoYear(); // true
     * Carbon::parse('2016-01-01')->isLongIsoYear(); // true
     * Carbon::parse('2016-01-03')->isLongIsoYear(); // false
     * Carbon::parse('2019-12-29')->isLongIsoYear(); // false
     * Carbon::parse('2019-12-30')->isLongIsoYear(); // true
     * ```
     *
     * @see https://en.wikipedia.org/wiki/ISO_8601#Week_dates
     */
    public function isLongIsoYear(): bool
    {}

    /**
     * Compares the formatted values of the two dates.
     *
     * @example
     * ```
     * Carbon::parse('2019-06-13')->isSameAs('Y-d', Carbon::parse('2019-12-13')); // true
     * Carbon::parse('2019-06-13')->isSameAs('Y-d', Carbon::parse('2019-06-14')); // false
     * ```
     *
     * @param string                   $format date formats to compare.
     * @param DateTimeInterface|string $date   instance to compare with or null to use current day.
     */
    public function isSameAs(string $format, DateTimeInterface|string $date): bool
    {}

    /**
     * Determines if the instance is in the current unit given.
     *
     * @example
     * ```
     * Carbon::parse('2019-01-13')->isSameUnit('year', Carbon::parse('2019-12-25')); // true
     * Carbon::parse('2018-12-13')->isSameUnit('year', Carbon::parse('2019-12-25')); // false
     * ```
     *
     * @param string                   $unit singular unit string
     * @param DateTimeInterface|string $date instance to compare with or null to use current day.
     *
     * @throws BadComparisonUnitException
     *
     * @return bool
     */
    public function isSameUnit(string $unit, DateTimeInterface|string $date): bool
    {}

    /**
     * Determines if the instance is in the current unit given.
     *
     * @example
     * ```
     * Carbon::now()->isCurrentUnit('hour'); // true
     * Carbon::now()->subHours(2)->isCurrentUnit('hour'); // false
     * ```
     *
     * @param string $unit The unit to test.
     *
     * @throws BadMethodCallException
     */
    public function isCurrentUnit(string $unit): bool
    {}

    /**
     * Checks if the passed in date is in the same quarter as the instance quarter (and year if needed).
     *
     * @example
     * ```
     * Carbon::parse('2019-01-12')->isSameQuarter(Carbon::parse('2019-03-01')); // true
     * Carbon::parse('2019-01-12')->isSameQuarter(Carbon::parse('2019-04-01')); // false
     * Carbon::parse('2019-01-12')->isSameQuarter(Carbon::parse('2018-03-01')); // false
     * Carbon::parse('2019-01-12')->isSameQuarter(Carbon::parse('2018-03-01'), false); // true
     * ```
     *
     * @param DateTimeInterface|string $date       The instance to compare with or null to use current day.
     * @param bool                     $ofSameYear Check if it is the same month in the same year.
     *
     * @return bool
     */
    public function isSameQuarter(DateTimeInterface|string $date, bool $ofSameYear = true): bool
    {}

    /**
     * Checks if the passed in date is in the same month as the instance´s month.
     *
     * @example
     * ```
     * Carbon::parse('2019-01-12')->isSameMonth(Carbon::parse('2019-01-01')); // true
     * Carbon::parse('2019-01-12')->isSameMonth(Carbon::parse('2019-02-01')); // false
     * Carbon::parse('2019-01-12')->isSameMonth(Carbon::parse('2018-01-01')); // false
     * Carbon::parse('2019-01-12')->isSameMonth(Carbon::parse('2018-01-01'), false); // true
     * ```
     *
     * @param DateTimeInterface|string $date       The instance to compare with or null to use the current date.
     * @param bool                     $ofSameYear Check if it is the same month in the same year.
     *
     * @return bool
     */
    public function isSameMonth(DateTimeInterface|string $date, bool $ofSameYear = true): bool
    {}

    /**
     * Checks if this day is a specific day of the week.
     *
     * @example
     * ```
     * Carbon::parse('2019-07-17')->isDayOfWeek(Carbon::WEDNESDAY); // true
     * Carbon::parse('2019-07-17')->isDayOfWeek(Carbon::FRIDAY); // false
     * Carbon::parse('2019-07-17')->isDayOfWeek('Wednesday'); // true
     * Carbon::parse('2019-07-17')->isDayOfWeek('Friday'); // false
     * ```
     *
     * @param int|string $dayOfWeek
     *
     * @return bool
     */
    public function isDayOfWeek($dayOfWeek): bool
    {}

    /**
     * Check if its the birthday. Compares the date/month values of the two dates.
     *
     * @example
     * ```
     * Carbon::now()->subYears(5)->isBirthday(); // true
     * Carbon::now()->subYears(5)->subDay()->isBirthday(); // false
     * Carbon::parse('2019-06-05')->isBirthday(Carbon::parse('2001-06-05')); // true
     * Carbon::parse('2019-06-05')->isBirthday(Carbon::parse('2001-06-06')); // false
     * ```
     *
     * @param DateTimeInterface|string|null $date The instance to compare with or null to use current day.
     *
     * @return bool
     */
    public function isBirthday(DateTimeInterface|string|null $date = null): bool
    {}

    /**
     * Check if today is the last day of the Month
     *
     * @example
     * ```
     * Carbon::parse('2019-02-28')->isLastOfMonth(); // true
     * Carbon::parse('2019-03-28')->isLastOfMonth(); // false
     * Carbon::parse('2019-03-30')->isLastOfMonth(); // false
     * Carbon::parse('2019-03-31')->isLastOfMonth(); // true
     * Carbon::parse('2019-04-30')->isLastOfMonth(); // true
     * ```
     */
    public function isLastOfMonth(): bool
    {}

    /**
     * Check if the instance is start of a given unit (tolerating a given interval).
     *
     * @example
     * ```
     * // Check if a date-time is the first 15 minutes of the hour it's in
     * Carbon::parse('2019-02-28 20:13:00')->isStartOfUnit(Unit::Hour, '15 minutes'); // true
     * ```
     */
    public function isStartOfUnit(
        Unit $unit,
        Unit|DateInterval|Closure|CarbonConverterInterface|Duration|string|null $interval = null,
        mixed ...$params,
    ): bool {}

    /**
     * Check if the instance is end of a given unit (tolerating a given interval).
     *
     * @example
     * ```
     * // Check if a date-time is the last 15 minutes of the hour it's in
     * Carbon::parse('2019-02-28 20:13:00')->isEndOfUnit(Unit::Hour, '15 minutes'); // false
     * ```
     */
    public function isEndOfUnit(
        Unit $unit,
        Unit|DateInterval|Closure|CarbonConverterInterface|Duration|string|null $interval = null,
        mixed ...$params,
    ): bool {}

    /**
     * Determines if the instance is start of millisecond (first microsecond by default but interval can be customized).
     */
    public function isStartOfMillisecond(
        Unit|DateInterval|Closure|CarbonConverterInterface|Duration|string|null $interval = null,
    ): bool {}

    /**
     * Determines if the instance is end of millisecond (last microsecond by default but interval can be customized).
     */
    public function isEndOfMillisecond(
        Unit|DateInterval|Closure|CarbonConverterInterface|Duration|string|null $interval = null,
    ): bool {}

    /**
     * Determines if the instance is start of second (first microsecond by default but interval can be customized).
     */
    public function isStartOfSecond(
        Unit|DateInterval|Closure|CarbonConverterInterface|Duration|string|null $interval = null,
    ): bool {}

    /**
     * Determines if the instance is end of second (last microsecond by default but interval can be customized).
     */
    public function isEndOfSecond(
        Unit|DateInterval|Closure|CarbonConverterInterface|Duration|string|null $interval = null,
    ): bool {}

    /**
     * Determines if the instance is start of minute (first microsecond by default but interval can be customized).
     */
    public function isStartOfMinute(
        Unit|DateInterval|Closure|CarbonConverterInterface|Duration|string|null $interval = null,
    ): bool {}

    /**
     * Determines if the instance is end of minute (last microsecond by default but interval can be customized).
     */
    public function isEndOfMinute(
        Unit|DateInterval|Closure|CarbonConverterInterface|Duration|string|null $interval = null,
    ): bool {}

    /**
     * Determines if the instance is start of hour (first microsecond by default but interval can be customized).
     */
    public function isStartOfHour(
        Unit|DateInterval|Closure|CarbonConverterInterface|Duration|string|null $interval = null,
    ): bool {}

    /**
     * Determines if the instance is end of hour (last microsecond by default but interval can be customized).
     */
    public function isEndOfHour(
        Unit|DateInterval|Closure|CarbonConverterInterface|Duration|string|null $interval = null,
    ): bool {}

    /**
     * Check if the instance is start of day / midnight.
     *
     * @example
     * ```
     * Carbon::parse('2019-02-28 00:00:00')->isStartOfDay(); // true
     * Carbon::parse('2019-02-28 00:00:00.999999')->isStartOfDay(); // true
     * Carbon::parse('2019-02-28 00:00:01')->isStartOfDay(); // false
     * Carbon::parse('2019-02-28 00:00:00.000000')->isStartOfDay(true); // true
     * Carbon::parse('2019-02-28 00:00:00.000012')->isStartOfDay(true); // false
     * ```
     *
     * @param bool                                                                    $checkMicroseconds check time at microseconds precision
     * @param Unit|DateInterval|Closure|CarbonConverterInterface|Duration|string|null $interval          if an interval is specified it will be used as precision
     *                                                                                                   for instance with "15 minutes", it checks if current date-time
     *                                                                                                   is in the last 15 minutes of the day, with Unit::Hour, it
     *                                                                                                   checks if it's in the last hour of the day.
     */
    public function isStartOfDay(
        Unit|DateInterval|Closure|CarbonConverterInterface|Duration|string|bool $checkMicroseconds = false,
        Unit|DateInterval|Closure|CarbonConverterInterface|Duration|string|null $interval = null,
    ): bool {}

    /**
     * Check if the instance is end of day.
     *
     * @example
     * ```
     * Carbon::parse('2019-02-28 23:59:59.999999')->isEndOfDay(); // true
     * Carbon::parse('2019-02-28 23:59:59.123456')->isEndOfDay(); // true
     * Carbon::parse('2019-02-28 23:59:59')->isEndOfDay(); // true
     * Carbon::parse('2019-02-28 23:59:58.999999')->isEndOfDay(); // false
     * Carbon::parse('2019-02-28 23:59:59.999999')->isEndOfDay(true); // true
     * Carbon::parse('2019-02-28 23:59:59.123456')->isEndOfDay(true); // false
     * Carbon::parse('2019-02-28 23:59:59')->isEndOfDay(true); // false
     * ```
     *
     * @param bool                                                                    $checkMicroseconds check time at microseconds precision
     * @param Unit|DateInterval|Closure|CarbonConverterInterface|Duration|string|null $interval          if an interval is specified it will be used as precision
     *                                                                                                   for instance with "15 minutes", it checks if current date-time
     *                                                                                                   is in the last 15 minutes of the day, with Unit::Hour, it
     *                                                                                                   checks if it's in the last hour of the day.
     */
    public function isEndOfDay(
        Unit|DateInterval|Closure|CarbonConverterInterface|Duration|string|bool $checkMicroseconds = false,
        Unit|DateInterval|Closure|CarbonConverterInterface|Duration|string|null $interval = null,
    ): bool {}

    /**
     * Determines if the instance is start of week (first day by default but interval can be customized).
     *
     * @example
     * ```
     * Carbon::parse('2024-08-31')->startOfWeek()->isStartOfWeek(); // true
     * Carbon::parse('2024-08-31')->isStartOfWeek(); // false
     * ```
     */
    public function isStartOfWeek(
        Unit|DateInterval|Closure|CarbonConverterInterface|Duration|string|null $interval = null,
        WeekDay|int|null $weekStartsAt = null,
    ): bool {}

    /**
     * Determines if the instance is end of week (last day by default but interval can be customized).
     *
     * @example
     * ```
     * Carbon::parse('2024-08-31')->endOfWeek()->isEndOfWeek(); // true
     * Carbon::parse('2024-08-31')->isEndOfWeek(); // false
     * ```
     */
    public function isEndOfWeek(
        Unit|DateInterval|Closure|CarbonConverterInterface|Duration|string|null $interval = null,
        WeekDay|int|null $weekEndsAt = null,
    ): bool {}

    /**
     * Determines if the instance is start of month (first day by default but interval can be customized).
     */
    public function isStartOfMonth(
        Unit|DateInterval|Closure|CarbonConverterInterface|Duration|string|null $interval = null,
    ): bool {}

    /**
     * Determines if the instance is end of month (last day by default but interval can be customized).
     */
    public function isEndOfMonth(
        Unit|DateInterval|Closure|CarbonConverterInterface|Duration|string|null $interval = null,
    ): bool {}

    /**
     * Determines if the instance is start of quarter (first day by default but interval can be customized).
     */
    public function isStartOfQuarter(
        Unit|DateInterval|Closure|CarbonConverterInterface|Duration|string|null $interval = null,
    ): bool {}

    /**
     * Determines if the instance is end of quarter (last day by default but interval can be customized).
     */
    public function isEndOfQuarter(
        Unit|DateInterval|Closure|CarbonConverterInterface|Duration|string|null $interval = null,
    ): bool {}

    /**
     * Determines if the instance is start of year (first day by default but interval can be customized).
     */
    public function isStartOfYear(
        Unit|DateInterval|Closure|CarbonConverterInterface|Duration|string|null $interval = null,
    ): bool {}

    /**
     * Determines if the instance is end of year (last day by default but interval can be customized).
     */
    public function isEndOfYear(
        Unit|DateInterval|Closure|CarbonConverterInterface|Duration|string|null $interval = null,
    ): bool {}

    /**
     * Determines if the instance is start of decade (first day by default but interval can be customized).
     */
    public function isStartOfDecade(
        Unit|DateInterval|Closure|CarbonConverterInterface|Duration|string|null $interval = null,
    ): bool {}

    /**
     * Determines if the instance is end of decade (last day by default but interval can be customized).
     */
    public function isEndOfDecade(
        Unit|DateInterval|Closure|CarbonConverterInterface|Duration|string|null $interval = null,
    ): bool {}

    /**
     * Determines if the instance is start of century (first day by default but interval can be customized).
     */
    public function isStartOfCentury(
        Unit|DateInterval|Closure|CarbonConverterInterface|Duration|string|null $interval = null,
    ): bool {}

    /**
     * Determines if the instance is end of century (last day by default but interval can be customized).
     */
    public function isEndOfCentury(
        Unit|DateInterval|Closure|CarbonConverterInterface|Duration|string|null $interval = null,
    ): bool {}

    /**
     * Determines if the instance is start of millennium (first day by default but interval can be customized).
     */
    public function isStartOfMillennium(
        Unit|DateInterval|Closure|CarbonConverterInterface|Duration|string|null $interval = null,
    ): bool {}

    /**
     * Determines if the instance is end of millennium (last day by default but interval can be customized).
     */
    public function isEndOfMillennium(
        Unit|DateInterval|Closure|CarbonConverterInterface|Duration|string|null $interval = null,
    ): bool {}

    /**
     * Check if the instance is start of day / midnight.
     *
     * @example
     * ```
     * Carbon::parse('2019-02-28 00:00:00')->isMidnight(); // true
     * Carbon::parse('2019-02-28 00:00:00.999999')->isMidnight(); // true
     * Carbon::parse('2019-02-28 00:00:01')->isMidnight(); // false
     * ```
     */
    public function isMidnight(): bool
    {}

    /**
     * Check if the instance is midday.
     *
     * @example
     * ```
     * Carbon::parse('2019-02-28 11:59:59.999999')->isMidday(); // false
     * Carbon::parse('2019-02-28 12:00:00')->isMidday(); // true
     * Carbon::parse('2019-02-28 12:00:00.999999')->isMidday(); // true
     * Carbon::parse('2019-02-28 12:00:01')->isMidday(); // false
     * ```
     */
    public function isMidday(): bool
    {}

    /**
     * Checks if the (date)time string is in a given format.
     *
     * @example
     * ```
     * Carbon::hasFormat('11:12:45', 'h:i:s'); // true
     * Carbon::hasFormat('13:12:45', 'h:i:s'); // false
     * ```
     */
    public static function hasFormat(string $date, string $format): bool
    {}

    /**
     * Checks if the (date)time string is in a given format.
     *
     * @example
     * ```
     * Carbon::hasFormatWithModifiers('31/08/2015', 'd#m#Y'); // true
     * Carbon::hasFormatWithModifiers('31/08/2015', 'm#d#Y'); // false
     * ```
     *
     * @param string $date
     * @param string $format
     *
     * @return bool
     */
    public static function hasFormatWithModifiers(?string $date, string $format): bool
    {}

    /**
     * Checks if the (date)time string is in a given format and valid to create a
     * new instance.
     *
     * @example
     * ```
     * Carbon::canBeCreatedFromFormat('11:12:45', 'h:i:s'); // true
     * Carbon::canBeCreatedFromFormat('13:12:45', 'h:i:s'); // false
     * ```
     */
    public static function canBeCreatedFromFormat(?string $date, string $format): bool
    {}

    /**
     * Returns true if the current date matches the given string.
     *
     * @example
     * ```
     * var_dump(Carbon::parse('2019-06-02 12:23:45')->is('2019')); // true
     * var_dump(Carbon::parse('2019-06-02 12:23:45')->is('2018')); // false
     * var_dump(Carbon::parse('2019-06-02 12:23:45')->is('2019-06')); // true
     * var_dump(Carbon::parse('2019-06-02 12:23:45')->is('06-02')); // true
     * var_dump(Carbon::parse('2019-06-02 12:23:45')->is('2019-06-02')); // true
     * var_dump(Carbon::parse('2019-06-02 12:23:45')->is('Sunday')); // true
     * var_dump(Carbon::parse('2019-06-02 12:23:45')->is('June')); // true
     * var_dump(Carbon::parse('2019-06-02 12:23:45')->is('12:23')); // true
     * var_dump(Carbon::parse('2019-06-02 12:23:45')->is('12:23:45')); // true
     * var_dump(Carbon::parse('2019-06-02 12:23:45')->is('12:23:00')); // false
     * var_dump(Carbon::parse('2019-06-02 12:23:45')->is('12h')); // true
     * var_dump(Carbon::parse('2019-06-02 15:23:45')->is('3pm')); // true
     * var_dump(Carbon::parse('2019-06-02 15:23:45')->is('3am')); // false
     * ```
     *
     * @param string $tester day name, month name, hour, date, etc. as string
     */
    public function is(WeekDay|Month|string $tester): bool
    {}

    /**
     * Returns true if the date was created using CarbonImmutable::startOfTime()
     *
     * @return bool
     */
    public function isStartOfTime(): bool
    {}

    /**
     * Returns true if the date was created using CarbonImmutable::endOfTime()
     *
     * @return bool
     */
    public function isEndOfTime(): bool
    {}
}
