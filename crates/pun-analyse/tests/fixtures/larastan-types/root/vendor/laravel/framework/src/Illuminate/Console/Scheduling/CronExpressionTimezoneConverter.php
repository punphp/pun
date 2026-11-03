<?php

namespace Illuminate\Console\Scheduling;

use DateTimeZone;
use Illuminate\Support\Carbon;
use Illuminate\Support\Collection;
use Throwable;

class CronExpressionTimezoneConverter
{
    /**
     * Convert an event cron expression to the display timezone.
     *
     * Returns one or more expressions when values straddle a day boundary.
     *
     * @param  \Illuminate\Console\Scheduling\Event  $event
     * @param  \DateTimeZone  $timezone
     * @return array<string>
     */
    public static function forEvent(Event $event, DateTimeZone $timezone)
    {}

    /**
     * Resolve the timezone used by the given event.
     *
     * @param  \Illuminate\Console\Scheduling\Event  $event
     * @param  \DateTimeZone  $defaultTimezone
     * @return \DateTimeZone
     */
    protected static function resolveEventTimezone(Event $event, DateTimeZone $defaultTimezone)
    {}

    /**
     * Get offset components between the event and display timezones.
     *
     * @return array{int, int, int}
     */
    protected static function offsetComponents(Event $event, DateTimeZone $eventTimezone, DateTimeZone $displayTimezone)
    {}

    /**
     * Convert the expression segments by the given offsets.
     *
     * @param  array<int, string>  $segments
     * @param  int  $hourOffset
     * @param  int  $minuteOffset
     * @return array<int, string>|null
     */
    protected static function convert(array $segments, int $hourOffset, int $minuteOffset)
    {}

    /**
     * Build expressions for the given hour carry direction.
     *
     * @param  array<int, string>  $segments
     * @param  array<int, string>  $parts
     * @return array<int, string>|null
     */
    protected static function expressionsForHourCarry(array $segments, array $parts, int $hourCarry)
    {}

    /**
     * Shift the day-of-month field by the given carry, respecting month lengths.
     *
     * @param  string  $domField
     * @param  string  $monthField
     * @param  int  $carry
     * @return array<int, array{string, string}>|null
     */
    protected static function shiftDaysOfMonth(string $domField, string $monthField, int $carry)
    {}

    /**
     * Get the number of days in the given month of a non-leap year.
     *
     * @param  int  $month
     * @return int
     */
    protected static function daysInMonth(int $month)
    {}

    /**
     * Shift a cron field by the given offset and group it by carry direction.
     *
     * @param  string  $field
     * @param  int  $offset
     * @param  int  $min
     * @param  int  $max
     * @param  bool  $mergeCarries
     * @return array<int, string>|null
     */
    protected static function shiftedGroups(string $field, int $offset, int $min, int $max, bool $mergeCarries)
    {}

    /**
     * Expand a cron field into the sorted list of values it matches.
     *
     * @param  string  $field
     * @param  int  $min
     * @param  int  $max
     * @return array<int, int>|null
     */
    protected static function expand(string $field, int $min, int $max)
    {}

    /**
     * Shift values in a cron field and group them by carry direction.
     *
     * @param  array<int, int>  $values
     * @param  int  $offset
     * @param  int  $mod
     * @param  int  $min
     * @return array<int, array<int, int>>
     */
    protected static function shiftAndGroupValues(array $values, int $offset, int $mod, int $min = 0)
    {}

    /**
     * Shift values within a cron field, wrapping around its domain.
     *
     * @param  array<int, int>  $values
     * @param  int  $offset
     * @param  int  $mod
     * @param  int  $min
     * @return array<int, int>
     */
    protected static function shiftWrapped(array $values, int $offset, int $mod, int $min = 0)
    {}

    /**
     * Merge grouped values into a single sorted group.
     *
     * @param  array<int, array<int, int>>  $groups
     * @return array<int, int>
     */
    protected static function mergeGroups(array $groups)
    {}

    /**
     * Collapse a sorted list of values into the most compact cron field syntax.
     *
     * @param  array<int, int>  $values
     * @param  int  $min
     * @param  int  $max
     * @return string
     */
    protected static function collapse(array $values, int $min, int $max)
    {}

    /**
     * Collapse consecutive runs of three or more values into ranges.
     *
     * @param  array<int, int>  $values
     * @return string
     */
    protected static function collapseRuns(array $values)
    {}
}
