<?php

namespace Illuminate\Foundation\Testing;

use Illuminate\Support\Carbon;

class Wormhole
{
    /**
     * The amount of time to travel.
     *
     * @var int
     */
    public $value;

    /**
     * Create a new wormhole instance.
     *
     * @param  int  $value
     */
    public function __construct($value)
    {}

    /**
     * @template TReturn of mixed
     *
     * Travel forward the given number of microseconds.
     *
     * @param  (callable(): TReturn)|null  $callback
     * @return ($callback is null ? void : TReturn)
     */
    public function microsecond($callback = null)
    {}

    /**
     * @template TReturn of mixed
     *
     * Travel forward the given number of microseconds.
     *
     * @param  (callable(): TReturn)|null  $callback
     * @return ($callback is null ? void : TReturn)
     */
    public function microseconds($callback = null)
    {}

    /**
     * @template TReturn of mixed
     *
     * Travel forward the given number of milliseconds.
     *
     * @param  (callable(): TReturn)|null  $callback
     * @return ($callback is null ? void : TReturn)
     */
    public function millisecond($callback = null)
    {}

    /**
     * @template TReturn of mixed
     *
     * Travel forward the given number of milliseconds.
     *
     * @param  (callable(): TReturn)|null  $callback
     * @return ($callback is null ? void : TReturn)
     */
    public function milliseconds($callback = null)
    {}

    /**
     * @template TReturn of mixed
     *
     * Travel forward the given number of seconds.
     *
     * @param  (callable(): TReturn)|null  $callback
     * @return ($callback is null ? void : TReturn)
     */
    public function second($callback = null)
    {}

    /**
     * @template TReturn of mixed
     *
     * Travel forward the given number of seconds.
     *
     * @param  (callable(): TReturn)|null  $callback
     * @return ($callback is null ? void : TReturn)
     */
    public function seconds($callback = null)
    {}

    /**
     * @template TReturn of mixed
     *
     * Travel forward the given number of minutes.
     *
     * @param  (callable(): TReturn)|null  $callback
     * @return ($callback is null ? void : TReturn)
     */
    public function minute($callback = null)
    {}

    /**
     * @template TReturn of mixed
     *
     * Travel forward the given number of minutes.
     *
     * @param  (callable(): TReturn)|null  $callback
     * @return ($callback is null ? void : TReturn)
     */
    public function minutes($callback = null)
    {}

    /**
     * @template TReturn of mixed
     *
     * Travel forward the given number of hours.
     *
     * @param  (callable(): TReturn)|null  $callback
     * @return ($callback is null ? void : TReturn)
     */
    public function hour($callback = null)
    {}

    /**
     * @template TReturn of mixed
     *
     * Travel forward the given number of hours.
     *
     * @param  (callable(): TReturn)|null  $callback
     * @return ($callback is null ? void : TReturn)
     */
    public function hours($callback = null)
    {}

    /**
     * @template TReturn of mixed
     *
     * Travel forward the given number of days.
     *
     * @param  (callable(): TReturn)|null  $callback
     * @return ($callback is null ? void : TReturn)
     */
    public function day($callback = null)
    {}

    /**
     * @template TReturn of mixed
     *
     * Travel forward the given number of days.
     *
     * @param  (callable(): TReturn)|null  $callback
     * @return ($callback is null ? void : TReturn)
     */
    public function days($callback = null)
    {}

    /**
     * @template TReturn of mixed
     *
     * Travel forward the given number of weeks.
     *
     * @param  (callable(): TReturn)|null  $callback
     * @return ($callback is null ? void : TReturn)
     */
    public function week($callback = null)
    {}

    /**
     * @template TReturn of mixed
     *
     * Travel forward the given number of weeks.
     *
     * @param  (callable(): TReturn)|null  $callback
     * @return ($callback is null ? void : TReturn)
     */
    public function weeks($callback = null)
    {}

    /**
     * @template TReturn of mixed
     *
     * Travel forward the given number of months.
     *
     * @param  (callable(): TReturn)|null  $callback
     * @return ($callback is null ? void : TReturn)
     */
    public function month($callback = null)
    {}

    /**
     * @template TReturn of mixed
     *
     * Travel forward the given number of months.
     *
     * @param  (callable(): TReturn)|null  $callback
     * @return ($callback is null ? void : TReturn)
     */
    public function months($callback = null)
    {}

    /**
     * @template TReturn of mixed
     *
     * Travel forward the given number of years.
     *
     * @param  (callable(): TReturn)|null  $callback
     * @return ($callback is null ? void : TReturn)
     */
    public function year($callback = null)
    {}

    /**
     * @template TReturn of mixed
     *
     * Travel forward the given number of years.
     *
     * @param  (callable(): TReturn)|null  $callback
     * @return ($callback is null ? void : TReturn)
     */
    public function years($callback = null)
    {}

    /**
     * Travel back to the current time.
     *
     * @return \DateTimeInterface
     */
    public static function back()
    {}

    /**
     * @template TReturn of mixed
     *
     * Handle the given optional execution callback.
     *
     * @param  (callable(): TReturn)|null  $callback
     * @return ($callback is null ? void : TReturn)
     */
    protected function handleCallback($callback)
    {}
}
