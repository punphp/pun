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

use Carbon\Carbon;
use Carbon\CarbonImmutable;
use Carbon\CarbonInterface;
use Carbon\CarbonInterval;
use Carbon\CarbonPeriod;
use Carbon\DurationConverter;
use Carbon\Exceptions\UnknownUnitException;
use Carbon\Unit;
use Closure;
use DateInterval;
use DateTimeInterface;
use Time\Duration;

/**
 * Trait Difference.
 *
 * Depends on the following methods:
 *
 * @method bool lessThan($date)
 * @method static copy()
 * @method static resolveCarbon($date = null)
 */
trait Difference
{
    /**
     * Get the difference as a DateInterval instance.
     * Return relative interval (negative if $absolute flag is not set to true and the given date is before
     * current one).
     *
     * @param \Carbon\CarbonInterface|\DateTimeInterface|string|null $date
     * @param bool                                                   $absolute Get the absolute of the difference
     *
     * @return DateInterval
     */
    public function diffAsDateInterval($date = null, bool $absolute = false): DateInterval
    {}

    /**
     * Get the difference as a \Time\Duration instance.
     * Return relative interval (negative if $absolute flag is not set to true and the given date is before
     * current one).
     *
     * @param \Carbon\CarbonInterface|\DateTimeInterface|string|null $date
     * @param bool                                                   $absolute Get the absolute of the difference
     *
     * @return Duration
     */
    public function diffAsDuration($date = null, bool $absolute = false): Duration
    {}

    /**
     * Get the difference as a CarbonInterval instance.
     * Return relative interval (negative if $absolute flag is not set to true and the given date is before
     * current one).
     *
     * @param \Carbon\CarbonInterface|\DateTimeInterface|string|null $date
     * @param bool                                                   $absolute Get the absolute of the difference
     *
     * @return CarbonInterval
     */
    public function diffAsCarbonInterval($date = null, bool $absolute = false, array $skip = []): CarbonInterval
    {}

    /**
     * @alias diffAsCarbonInterval
     *
     * Get the difference as a CarbonInterval instance.
     * Return relative interval (negative if $absolute flag is not set to true and the given date is before
     * current one).
     *
     * @param \Carbon\CarbonInterface|\DateTimeInterface|string|null $date
     * @param bool                                                   $absolute Get the absolute of the difference
     *
     * @return CarbonInterval
     */
    public function diff($date = null, bool $absolute = false, array $skip = []): CarbonInterval
    {}

    /**
     * @param Unit|string                                            $unit     microsecond, millisecond, second, minute,
     *                                                                         hour, day, week, month, quarter, year,
     *                                                                         century, millennium
     * @param \Carbon\CarbonInterface|\DateTimeInterface|string|null $date
     * @param bool                                                   $absolute Get the absolute of the difference
     * @param bool                                                   $utc      Always convert dates to UTC before comparing (if not set, it will do it only if timezones are different)
     *
     * @return float
     */
    public function diffInUnit(Unit|string $unit, $date = null, bool $absolute = false, bool $utc = false): float
    {}

    /**
     * Get the difference in years
     *
     * @param \Carbon\CarbonInterface|\DateTimeInterface|string|null $date
     * @param bool                                                   $absolute Get the absolute of the difference
     * @param bool                                                   $utc      Always convert dates to UTC before comparing (if not set, it will do it only if timezones are different)
     *
     * @return float
     */
    public function diffInYears($date = null, bool $absolute = false, bool $utc = false): float
    {}

    /**
     * Get the difference in quarters.
     *
     * @param \Carbon\CarbonInterface|\DateTimeInterface|string|null $date
     * @param bool                                                   $absolute Get the absolute of the difference
     * @param bool                                                   $utc      Always convert dates to UTC before comparing (if not set, it will do it only if timezones are different)
     *
     * @return float
     */
    public function diffInQuarters($date = null, bool $absolute = false, bool $utc = false): float
    {}

    /**
     * Get the difference in months.
     *
     * @param \Carbon\CarbonInterface|\DateTimeInterface|string|null $date
     * @param bool                                                   $absolute Get the absolute of the difference
     * @param bool                                                   $utc      Always convert dates to UTC before comparing (if not set, it will do it only if timezones are different)
     *
     * @return float
     */
    public function diffInMonths($date = null, bool $absolute = false, bool $utc = false): float
    {}

    /**
     * Get the difference in weeks.
     *
     * @param \Carbon\CarbonInterface|\DateTimeInterface|string|null $date
     * @param bool                                                   $absolute Get the absolute of the difference
     * @param bool                                                   $utc      Always convert dates to UTC before comparing (if not set, it will do it only if timezones are different)
     *
     * @return float
     */
    public function diffInWeeks($date = null, bool $absolute = false, bool $utc = false): float
    {}

    /**
     * Get the difference in days.
     *
     * @param \Carbon\CarbonInterface|\DateTimeInterface|string|null $date
     * @param bool                                                   $absolute Get the absolute of the difference
     * @param bool                                                   $utc      Always convert dates to UTC before comparing (if not set, it will do it only if timezones are different)
     *
     * @return float
     */
    public function diffInDays($date = null, bool $absolute = false, bool $utc = false): float
    {}

    /**
     * Get the difference in days using a filter closure.
     *
     * @param Closure                                                $callback
     * @param \Carbon\CarbonInterface|\DateTimeInterface|string|null $date
     * @param bool                                                   $absolute Get the absolute of the difference
     *
     * @return int
     */
    public function diffInDaysFiltered(Closure $callback, $date = null, bool $absolute = false): int
    {}

    /**
     * Get the difference in hours using a filter closure.
     *
     * @param Closure                                                $callback
     * @param \Carbon\CarbonInterface|\DateTimeInterface|string|null $date
     * @param bool                                                   $absolute Get the absolute of the difference
     *
     * @return int
     */
    public function diffInHoursFiltered(Closure $callback, $date = null, bool $absolute = false): int
    {}

    /**
     * Get the difference by the given interval using a filter closure.
     *
     * @param CarbonInterval                                         $ci       An interval to traverse by
     * @param Closure                                                $callback
     * @param \Carbon\CarbonInterface|\DateTimeInterface|string|null $date
     * @param bool                                                   $absolute Get the absolute of the difference
     *
     * @return int
     */
    public function diffFiltered(CarbonInterval $ci, Closure $callback, $date = null, bool $absolute = false): int
    {}

    /**
     * Get the difference in weekdays.
     *
     * @param \Carbon\CarbonInterface|\DateTimeInterface|string|null $date
     * @param bool                                                   $absolute Get the absolute of the difference
     *
     * @return int
     */
    public function diffInWeekdays($date = null, bool $absolute = false): int
    {}

    /**
     * Get the difference in weekend days using a filter.
     *
     * @param \Carbon\CarbonInterface|\DateTimeInterface|string|null $date
     * @param bool                                                   $absolute Get the absolute of the difference
     *
     * @return int
     */
    public function diffInWeekendDays($date = null, bool $absolute = false): int
    {}

    /**
     * Get the difference in hours.
     *
     * @param \Carbon\CarbonInterface|\DateTimeInterface|string|null $date
     * @param bool                                                   $absolute Get the absolute of the difference
     *
     * @return float
     */
    public function diffInHours($date = null, bool $absolute = false): float
    {}

    /**
     * Get the difference in minutes.
     *
     * @param \Carbon\CarbonInterface|\DateTimeInterface|string|null $date
     * @param bool                                                   $absolute Get the absolute of the difference
     *
     * @return float
     */
    public function diffInMinutes($date = null, bool $absolute = false): float
    {}

    /**
     * Get the difference in seconds.
     *
     * @param \Carbon\CarbonInterface|\DateTimeInterface|string|null $date
     * @param bool                                                   $absolute Get the absolute of the difference
     *
     * @return float
     */
    public function diffInSeconds($date = null, bool $absolute = false): float
    {}

    /**
     * Get the difference in microseconds.
     *
     * @param \Carbon\CarbonInterface|\DateTimeInterface|string|null $date
     * @param bool                                                   $absolute Get the absolute of the difference
     *
     * @return float
     */
    public function diffInMicroseconds($date = null, bool $absolute = false): float
    {}

    /**
     * Get the difference in milliseconds.
     *
     * @param \Carbon\CarbonInterface|\DateTimeInterface|string|null $date
     * @param bool                                                   $absolute Get the absolute of the difference
     *
     * @return float
     */
    public function diffInMilliseconds($date = null, bool $absolute = false): float
    {}

    /**
     * The number of seconds since midnight.
     *
     * @return float
     */
    public function secondsSinceMidnight(): float
    {}

    /**
     * The number of seconds until 23:59:59.
     *
     * @return float
     */
    public function secondsUntilEndOfDay(): float
    {}

    /**
     * Get the difference in a human readable format in the current locale from current instance to an other
     * instance given (or now if null given).
     *
     * @example
     * ```
     * echo Carbon::tomorrow()->diffForHumans() . "\n";
     * echo Carbon::tomorrow()->diffForHumans(['parts' => 2]) . "\n";
     * echo Carbon::tomorrow()->diffForHumans(['parts' => 3, 'join' => true]) . "\n";
     * echo Carbon::tomorrow()->diffForHumans(Carbon::yesterday()) . "\n";
     * echo Carbon::tomorrow()->diffForHumans(Carbon::yesterday(), ['short' => true]) . "\n";
     * ```
     *
     * @param Carbon|DateTimeInterface|string|array|null $other   if array passed, will be used as parameters array, see $syntax below;
     *                                                            if null passed, now will be used as comparison reference;
     *                                                            if any other type, it will be converted to date and used as reference.
     * @param int|array                                  $syntax  if array passed, parameters will be extracted from it, the array may contains:
     *                                                            ⦿ 'syntax' entry (see below)
     *                                                            ⦿ 'short' entry (see below)
     *                                                            ⦿ 'parts' entry (see below)
     *                                                            ⦿ 'options' entry (see below)
     *                                                            ⦿ 'skip' entry, list of units to skip (array of strings or a single string,
     *                                                            ` it can be the unit name (singular or plural) or its shortcut
     *                                                            ` (y, m, w, d, h, min, s, ms, µs).
     *                                                            ⦿ 'aUnit' entry, prefer "an hour" over "1 hour" if true
     *                                                            ⦿ 'altNumbers' entry, use alternative numbers if available
     *                                                            ` (from the current language if true is passed, from the given language(s)
     *                                                            ` if array or string is passed)
     *                                                            ⦿ 'join' entry determines how to join multiple parts of the string
     *                                                            `  - if $join is a string, it's used as a joiner glue
     *                                                            `  - if $join is a callable/closure, it get the list of string and should return a string
     *                                                            `  - if $join is an array, the first item will be the default glue, and the second item
     *                                                            `    will be used instead of the glue for the last item
     *                                                            `  - if $join is true, it will be guessed from the locale ('list' translation file entry)
     *                                                            `  - if $join is missing, a space will be used as glue
     *                                                            ⦿ 'other' entry (see above)
     *                                                            ⦿ 'minimumUnit' entry determines the smallest unit of time to display can be long or
     *                                                            `  short form of the units, e.g. 'hour' or 'h' (default value: s)
     *                                                            ⦿ 'locale' language in which the diff should be output (has no effect if 'translator' key is set)
     *                                                            ⦿ 'translator' a custom translator to use to translator the output.
     *                                                            if int passed, it adds modifiers:
     *                                                            Possible values:
     *                                                            - CarbonInterface::DIFF_ABSOLUTE          no modifiers
     *                                                            - CarbonInterface::DIFF_RELATIVE_TO_NOW   add ago/from now modifier
     *                                                            - CarbonInterface::DIFF_RELATIVE_TO_OTHER add before/after modifier
     *                                                            Default value: CarbonInterface::DIFF_ABSOLUTE
     * @param bool                                       $short   displays short format of time units
     * @param int                                        $parts   maximum number of parts to display (default value: 1: single unit)
     * @param int                                        $options human diff options
     */
    public function diffForHumans($other = null, $syntax = null, $short = false, $parts = 1, $options = null): string
    {}

    /**
     * @alias diffForHumans
     *
     * Get the difference in a human readable format in the current locale from current instance to an other
     * instance given (or now if null given).
     *
     * @param Carbon|\DateTimeInterface|string|array|null $other   if array passed, will be used as parameters array, see $syntax below;
     *                                                             if null passed, now will be used as comparison reference;
     *                                                             if any other type, it will be converted to date and used as reference.
     * @param int|array                                   $syntax  if array passed, parameters will be extracted from it, the array may contains:
     *                                                             - 'syntax' entry (see below)
     *                                                             - 'short' entry (see below)
     *                                                             - 'parts' entry (see below)
     *                                                             - 'options' entry (see below)
     *                                                             - 'join' entry determines how to join multiple parts of the string
     *                                                             `  - if $join is a string, it's used as a joiner glue
     *                                                             `  - if $join is a callable/closure, it get the list of string and should return a string
     *                                                             `  - if $join is an array, the first item will be the default glue, and the second item
     *                                                             `    will be used instead of the glue for the last item
     *                                                             `  - if $join is true, it will be guessed from the locale ('list' translation file entry)
     *                                                             `  - if $join is missing, a space will be used as glue
     *                                                             - 'other' entry (see above)
     *                                                             if int passed, it add modifiers:
     *                                                             Possible values:
     *                                                             - CarbonInterface::DIFF_ABSOLUTE          no modifiers
     *                                                             - CarbonInterface::DIFF_RELATIVE_TO_NOW   add ago/from now modifier
     *                                                             - CarbonInterface::DIFF_RELATIVE_TO_OTHER add before/after modifier
     *                                                             Default value: CarbonInterface::DIFF_ABSOLUTE
     * @param bool                                        $short   displays short format of time units
     * @param int                                         $parts   maximum number of parts to display (default value: 1: single unit)
     * @param int                                         $options human diff options
     *
     * @return string
     */
    public function from($other = null, $syntax = null, $short = false, $parts = 1, $options = null)
    {}

    /**
     * @alias diffForHumans
     *
     * Get the difference in a human readable format in the current locale from current instance to an other
     * instance given (or now if null given).
     */
    public function since($other = null, $syntax = null, $short = false, $parts = 1, $options = null)
    {}

    /**
     * Get the difference in a human readable format in the current locale from an other
     * instance given (or now if null given) to current instance.
     *
     * When comparing a value in the past to default now:
     * 1 hour from now
     * 5 months from now
     *
     * When comparing a value in the future to default now:
     * 1 hour ago
     * 5 months ago
     *
     * When comparing a value in the past to another value:
     * 1 hour after
     * 5 months after
     *
     * When comparing a value in the future to another value:
     * 1 hour before
     * 5 months before
     *
     * @param Carbon|\DateTimeInterface|string|array|null $other   if array passed, will be used as parameters array, see $syntax below;
     *                                                             if null passed, now will be used as comparison reference;
     *                                                             if any other type, it will be converted to date and used as reference.
     * @param int|array                                   $syntax  if array passed, parameters will be extracted from it, the array may contains:
     *                                                             - 'syntax' entry (see below)
     *                                                             - 'short' entry (see below)
     *                                                             - 'parts' entry (see below)
     *                                                             - 'options' entry (see below)
     *                                                             - 'join' entry determines how to join multiple parts of the string
     *                                                             `  - if $join is a string, it's used as a joiner glue
     *                                                             `  - if $join is a callable/closure, it get the list of string and should return a string
     *                                                             `  - if $join is an array, the first item will be the default glue, and the second item
     *                                                             `    will be used instead of the glue for the last item
     *                                                             `  - if $join is true, it will be guessed from the locale ('list' translation file entry)
     *                                                             `  - if $join is missing, a space will be used as glue
     *                                                             - 'other' entry (see above)
     *                                                             if int passed, it add modifiers:
     *                                                             Possible values:
     *                                                             - CarbonInterface::DIFF_ABSOLUTE          no modifiers
     *                                                             - CarbonInterface::DIFF_RELATIVE_TO_NOW   add ago/from now modifier
     *                                                             - CarbonInterface::DIFF_RELATIVE_TO_OTHER add before/after modifier
     *                                                             Default value: CarbonInterface::DIFF_ABSOLUTE
     * @param bool                                        $short   displays short format of time units
     * @param int                                         $parts   maximum number of parts to display (default value: 1: single unit)
     * @param int                                         $options human diff options
     *
     * @return string
     */
    public function to($other = null, $syntax = null, $short = false, $parts = 1, $options = null)
    {}

    /**
     * @alias to
     *
     * Get the difference in a human readable format in the current locale from an other
     * instance given (or now if null given) to current instance.
     *
     * @param Carbon|\DateTimeInterface|string|array|null $other   if array passed, will be used as parameters array, see $syntax below;
     *                                                             if null passed, now will be used as comparison reference;
     *                                                             if any other type, it will be converted to date and used as reference.
     * @param int|array                                   $syntax  if array passed, parameters will be extracted from it, the array may contains:
     *                                                             - 'syntax' entry (see below)
     *                                                             - 'short' entry (see below)
     *                                                             - 'parts' entry (see below)
     *                                                             - 'options' entry (see below)
     *                                                             - 'join' entry determines how to join multiple parts of the string
     *                                                             `  - if $join is a string, it's used as a joiner glue
     *                                                             `  - if $join is a callable/closure, it get the list of string and should return a string
     *                                                             `  - if $join is an array, the first item will be the default glue, and the second item
     *                                                             `    will be used instead of the glue for the last item
     *                                                             `  - if $join is true, it will be guessed from the locale ('list' translation file entry)
     *                                                             `  - if $join is missing, a space will be used as glue
     *                                                             - 'other' entry (see above)
     *                                                             if int passed, it add modifiers:
     *                                                             Possible values:
     *                                                             - CarbonInterface::DIFF_ABSOLUTE          no modifiers
     *                                                             - CarbonInterface::DIFF_RELATIVE_TO_NOW   add ago/from now modifier
     *                                                             - CarbonInterface::DIFF_RELATIVE_TO_OTHER add before/after modifier
     *                                                             Default value: CarbonInterface::DIFF_ABSOLUTE
     * @param bool                                        $short   displays short format of time units
     * @param int                                         $parts   maximum number of parts to display (default value: 1: single unit)
     * @param int                                         $options human diff options
     *
     * @return string
     */
    public function until($other = null, $syntax = null, $short = false, $parts = 1, $options = null)
    {}

    /**
     * Get the difference in a human readable format in the current locale from current
     * instance to now.
     *
     * @param int|array $syntax  if array passed, parameters will be extracted from it, the array may contains:
     *                           - 'syntax' entry (see below)
     *                           - 'short' entry (see below)
     *                           - 'parts' entry (see below)
     *                           - 'options' entry (see below)
     *                           - 'join' entry determines how to join multiple parts of the string
     *                           `  - if $join is a string, it's used as a joiner glue
     *                           `  - if $join is a callable/closure, it get the list of string and should return a string
     *                           `  - if $join is an array, the first item will be the default glue, and the second item
     *                           `    will be used instead of the glue for the last item
     *                           `  - if $join is true, it will be guessed from the locale ('list' translation file entry)
     *                           `  - if $join is missing, a space will be used as glue
     *                           if int passed, it add modifiers:
     *                           Possible values:
     *                           - CarbonInterface::DIFF_ABSOLUTE          no modifiers
     *                           - CarbonInterface::DIFF_RELATIVE_TO_NOW   add ago/from now modifier
     *                           - CarbonInterface::DIFF_RELATIVE_TO_OTHER add before/after modifier
     *                           Default value: CarbonInterface::DIFF_ABSOLUTE
     * @param bool      $short   displays short format of time units
     * @param int       $parts   maximum number of parts to display (default value: 1: single unit)
     * @param int       $options human diff options
     *
     * @return string
     */
    public function fromNow($syntax = null, $short = false, $parts = 1, $options = null)
    { func_get_args(); }

    /**
     * Get the difference in a human readable format in the current locale from an other
     * instance given to now
     *
     * @param int|array $syntax  if array passed, parameters will be extracted from it, the array may contains:
     *                           - 'syntax' entry (see below)
     *                           - 'short' entry (see below)
     *                           - 'parts' entry (see below)
     *                           - 'options' entry (see below)
     *                           - 'join' entry determines how to join multiple parts of the string
     *                           `  - if $join is a string, it's used as a joiner glue
     *                           `  - if $join is a callable/closure, it get the list of string and should return a string
     *                           `  - if $join is an array, the first item will be the default glue, and the second item
     *                           `    will be used instead of the glue for the last item
     *                           `  - if $join is true, it will be guessed from the locale ('list' translation file entry)
     *                           `  - if $join is missing, a space will be used as glue
     *                           if int passed, it add modifiers:
     *                           Possible values:
     *                           - CarbonInterface::DIFF_ABSOLUTE          no modifiers
     *                           - CarbonInterface::DIFF_RELATIVE_TO_NOW   add ago/from now modifier
     *                           - CarbonInterface::DIFF_RELATIVE_TO_OTHER add before/after modifier
     *                           Default value: CarbonInterface::DIFF_ABSOLUTE
     * @param bool      $short   displays short format of time units
     * @param int       $parts   maximum number of parts to display (default value: 1: single part)
     * @param int       $options human diff options
     *
     * @return string
     */
    public function toNow($syntax = null, $short = false, $parts = 1, $options = null)
    {}

    /**
     * Get the difference in a human readable format in the current locale from an other
     * instance given to now
     *
     * @param int|array $syntax  if array passed, parameters will be extracted from it, the array may contains:
     *                           - 'syntax' entry (see below)
     *                           - 'short' entry (see below)
     *                           - 'parts' entry (see below)
     *                           - 'options' entry (see below)
     *                           - 'join' entry determines how to join multiple parts of the string
     *                           `  - if $join is a string, it's used as a joiner glue
     *                           `  - if $join is a callable/closure, it get the list of string and should return a string
     *                           `  - if $join is an array, the first item will be the default glue, and the second item
     *                           `    will be used instead of the glue for the last item
     *                           `  - if $join is true, it will be guessed from the locale ('list' translation file entry)
     *                           `  - if $join is missing, a space will be used as glue
     *                           if int passed, it add modifiers:
     *                           Possible values:
     *                           - CarbonInterface::DIFF_ABSOLUTE          no modifiers
     *                           - CarbonInterface::DIFF_RELATIVE_TO_NOW   add ago/from now modifier
     *                           - CarbonInterface::DIFF_RELATIVE_TO_OTHER add before/after modifier
     *                           Default value: CarbonInterface::DIFF_ABSOLUTE
     * @param bool      $short   displays short format of time units
     * @param int       $parts   maximum number of parts to display (default value: 1: single part)
     * @param int       $options human diff options
     *
     * @return string
     */
    public function ago($syntax = null, $short = false, $parts = 1, $options = null)
    { func_get_args(); }

    /**
     * Get the difference in a human-readable format in the current locale from current instance to another
     * instance given (or now if null given).
     *
     * @return string
     */
    public function timespan($other = null, $timezone = null): string
    {}

    /**
     * Returns either day of week + time (e.g. "Last Friday at 3:30 PM") if reference time is within 7 days,
     * or a calendar date (e.g. "10/29/2017") otherwise.
     *
     * Language, date and time formats will change according to the current locale.
     *
     * @param Carbon|\DateTimeInterface|string|null $referenceTime
     * @param array                                 $formats
     *
     * @return string
     */
    public function calendar($referenceTime = null, array $formats = [])
    {}

    private function getIntervalDayDiff(DateInterval $interval): int
    {}
}
