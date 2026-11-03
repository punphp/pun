<?php

namespace Illuminate\Support;

use Throwable;

class Timebox
{
    /**
     * Indicates if the timebox is allowed to return early.
     *
     * @var bool
     */
    public $earlyReturn = false;

    /**
     * Invoke the given callback within the specified timebox minimum.
     *
     * @template TCallReturnType
     *
     * @param  (callable($this): TCallReturnType)  $callback
     * @param  int  $microseconds
     * @return TCallReturnType
     *
     * @throws \Throwable
     */
    public function call(callable $callback, int $microseconds)
    {}

    /**
     * Indicate that the timebox can return early.
     *
     * @return $this
     */
    public function returnEarly()
    {}

    /**
     * Indicate that the timebox cannot return early.
     *
     * @return $this
     */
    public function dontReturnEarly()
    {}

    /**
     * Sleep for the specified number of microseconds.
     *
     * @param  int  $microseconds
     * @return void
     */
    protected function usleep(int $microseconds)
    {}
}
