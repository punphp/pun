<?php

namespace Illuminate\Console\Scheduling;

use Illuminate\Support\Carbon;
use InvalidArgumentException;

use function Illuminate\Support\enum_value;

trait ManagesFrequencies
{
    /**
     * The Cron expression representing the event's frequency.
     *
     * @param  string  $expression
     * @return $this
     */
    public function cron($expression)
    {}

    /**
     * Schedule the event to run between start and end time.
     *
     * @param  string  $startTime
     * @param  string  $endTime
     * @return $this
     */
    public function between($startTime, $endTime)
    {}

    /**
     * Schedule the event to not run between start and end time.
     *
     * @param  string  $startTime
     * @param  string  $endTime
     * @return $this
     */
    public function unlessBetween($startTime, $endTime)
    {}

    /**
     * Schedule the event to run between start and end time.
     *
     * @param  string  $startTime
     * @param  string  $endTime
     * @return \Closure
     */
    private function inTimeInterval($startTime, $endTime)
    {}

    /**
     * Schedule the event to run every second.
     *
     * @return $this
     */
    public function everySecond()
    {}

    /**
     * Schedule the event to run every two seconds.
     *
     * @return $this
     */
    public function everyTwoSeconds()
    {}

    /**
     * Schedule the event to run every five seconds.
     *
     * @return $this
     */
    public function everyFiveSeconds()
    {}

    /**
     * Schedule the event to run every ten seconds.
     *
     * @return $this
     */
    public function everyTenSeconds()
    {}

    /**
     * Schedule the event to run every fifteen seconds.
     *
     * @return $this
     */
    public function everyFifteenSeconds()
    {}

    /**
     * Schedule the event to run every twenty seconds.
     *
     * @return $this
     */
    public function everyTwentySeconds()
    {}

    /**
     * Schedule the event to run every thirty seconds.
     *
     * @return $this
     */
    public function everyThirtySeconds()
    {}

    /**
     * Schedule the event to run multiple times per minute.
     *
     * @param  int<1, 59>  $seconds
     * @return $this
     *
     * @throws \InvalidArgumentException
     */
    protected function repeatEvery($seconds)
    {}

    /**
     * Schedule the event to run every minute.
     *
     * @return $this
     */
    public function everyMinute()
    {}

    /**
     * Schedule the event to run every two minutes.
     *
     * @return $this
     */
    public function everyTwoMinutes()
    {}

    /**
     * Schedule the event to run every three minutes.
     *
     * @return $this
     */
    public function everyThreeMinutes()
    {}

    /**
     * Schedule the event to run every four minutes.
     *
     * @return $this
     */
    public function everyFourMinutes()
    {}

    /**
     * Schedule the event to run every five minutes.
     *
     * @return $this
     */
    public function everyFiveMinutes()
    {}

    /**
     * Schedule the event to run every ten minutes.
     *
     * @return $this
     */
    public function everyTenMinutes()
    {}

    /**
     * Schedule the event to run every fifteen minutes.
     *
     * @return $this
     */
    public function everyFifteenMinutes()
    {}

    /**
     * Schedule the event to run every thirty minutes.
     *
     * @return $this
     */
    public function everyThirtyMinutes()
    {}

    /**
     * Schedule the event to run hourly.
     *
     * @return $this
     */
    public function hourly()
    {}

    /**
     * Schedule the event to run hourly at a given offset in the hour.
     *
     * @param  array|string|int<0, 59>|int<0, 59>[]  $offset
     * @return $this
     */
    public function hourlyAt($offset)
    {}

    /**
     * Schedule the event to run every odd hour.
     *
     * @param  array|string|int  $offset
     * @return $this
     */
    public function everyOddHour($offset = 0)
    {}

    /**
     * Schedule the event to run every two hours.
     *
     * @param  array|string|int  $offset
     * @return $this
     */
    public function everyTwoHours($offset = 0)
    {}

    /**
     * Schedule the event to run every three hours.
     *
     * @param  array|string|int  $offset
     * @return $this
     */
    public function everyThreeHours($offset = 0)
    {}

    /**
     * Schedule the event to run every four hours.
     *
     * @param  array|string|int  $offset
     * @return $this
     */
    public function everyFourHours($offset = 0)
    {}

    /**
     * Schedule the event to run every six hours.
     *
     * @param  array|string|int  $offset
     * @return $this
     */
    public function everySixHours($offset = 0)
    {}

    /**
     * Schedule the event to run daily.
     *
     * @return $this
     */
    public function daily()
    {}

    /**
     * Schedule the command at a given time.
     *
     * @param  string  $time
     * @return $this
     */
    public function at($time)
    {}

    /**
     * Schedule the event to run daily at a given time (10:00, 19:30, etc).
     *
     * @param  string  $time
     * @return $this
     */
    public function dailyAt($time)
    {}

    /**
     * Schedule the event to run twice daily.
     *
     * @param  int<0, 23>  $first
     * @param  int<0, 23>  $second
     * @return $this
     */
    public function twiceDaily($first = 1, $second = 13)
    {}

    /**
     * Schedule the event to run twice daily at a given offset.
     *
     * @param  int<0, 23>  $first
     * @param  int<0, 23>  $second
     * @param  int<0, 59>  $offset
     * @return $this
     */
    public function twiceDailyAt($first = 1, $second = 13, $offset = 0)
    {}

    /**
     * Schedule the event to run at the given minutes and hours.
     *
     * @param  array|string|int<0, 59>  $minutes
     * @param  array|string|int<0, 23>  $hours
     * @return $this
     */
    protected function hourBasedSchedule($minutes, $hours)
    {}

    /**
     * Schedule the event to run only on weekdays.
     *
     * @return $this
     */
    public function weekdays()
    {}

    /**
     * Schedule the event to run only on weekends.
     *
     * @return $this
     */
    public function weekends()
    {}

    /**
     * Schedule the event to run only on Mondays.
     *
     * @return $this
     */
    public function mondays()
    {}

    /**
     * Schedule the event to run only on Tuesdays.
     *
     * @return $this
     */
    public function tuesdays()
    {}

    /**
     * Schedule the event to run only on Wednesdays.
     *
     * @return $this
     */
    public function wednesdays()
    {}

    /**
     * Schedule the event to run only on Thursdays.
     *
     * @return $this
     */
    public function thursdays()
    {}

    /**
     * Schedule the event to run only on Fridays.
     *
     * @return $this
     */
    public function fridays()
    {}

    /**
     * Schedule the event to run only on Saturdays.
     *
     * @return $this
     */
    public function saturdays()
    {}

    /**
     * Schedule the event to run only on Sundays.
     *
     * @return $this
     */
    public function sundays()
    {}

    /**
     * Schedule the event to run weekly.
     *
     * @return $this
     */
    public function weekly()
    {}

    /**
     * Schedule the event to run weekly on a given day and time.
     *
     * @param  mixed  $dayOfWeek
     * @param  string  $time
     * @return $this
     */
    public function weeklyOn($dayOfWeek, $time = '0:0')
    {}

    /**
     * Schedule the event to run monthly.
     *
     * @return $this
     */
    public function monthly()
    {}

    /**
     * Schedule the event to run monthly on a given day and time.
     *
     * @param  int<1, 31>  $dayOfMonth
     * @param  string  $time
     * @return $this
     */
    public function monthlyOn($dayOfMonth = 1, $time = '0:0')
    {}

    /**
     * Schedule the event to run twice monthly at a given time.
     *
     * @param  int<1, 31>  $first
     * @param  int<1, 31>  $second
     * @param  string  $time
     * @return $this
     */
    public function twiceMonthly($first = 1, $second = 16, $time = '0:0')
    {}

    /**
     * Schedule the event to run on the last day of the month.
     *
     * @param  string  $time
     * @return $this
     */
    public function lastDayOfMonth($time = '0:0')
    {}

    /**
     * Schedule the event to run on specific days of the month.
     *
     * @param  array<int<1, 31>>|int<1, 31>  ...$days
     * @return $this
     */
    public function daysOfMonth(...$days)
    {}

    /**
     * Schedule the event to run quarterly.
     *
     * @return $this
     */
    public function quarterly()
    {}

    /**
     * Schedule the event to run quarterly on a given day and time.
     *
     * @param  int  $dayOfQuarter
     * @param  string  $time
     * @return $this
     */
    public function quarterlyOn($dayOfQuarter = 1, $time = '0:0')
    {}

    /**
     * Schedule the event to run yearly.
     *
     * @return $this
     */
    public function yearly()
    {}

    /**
     * Schedule the event to run yearly on a given month, day, and time.
     *
     * @param  int  $month
     * @param  int<1, 31>|string  $dayOfMonth
     * @param  string  $time
     * @return $this
     */
    public function yearlyOn($month = 1, $dayOfMonth = 1, $time = '0:0')
    {}

    /**
     * Set the days of the week the command should run on.
     *
     * @param  mixed  $days
     * @return $this
     */
    public function days($days)
    { func_get_args(); }

    /**
     * Set the timezone the date should be evaluated on.
     *
     * @param  \UnitEnum|\DateTimeZone|string  $timezone
     * @return $this
     */
    public function timezone($timezone)
    {}

    /**
     * Splice the given value into the given position of the expression.
     *
     * @param  int  $position
     * @param  string|int  $value
     * @return $this
     */
    protected function spliceIntoPosition($position, $value)
    {}
}
