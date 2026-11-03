<?php

namespace Illuminate\Console\Concerns;

use Illuminate\Console\Signals;
use Illuminate\Support\Collection;

trait InteractsWithSignals
{
    /**
     * The signal registrar instance.
     *
     * @var \Illuminate\Console\Signals|null
     */
    protected $signals;

    /**
     * Define a callback to be run when the given signal(s) occurs.
     *
     * @template TSignals of iterable<array-key, int>|int
     *
     * @param  (\Closure():(TSignals))|TSignals  $signals
     * @param  callable(int $signal): void  $callback
     * @return void
     */
    public function trap($signals, $callback)
    {}

    /**
     * Untrap signal handlers set within the command's handler.
     *
     * @return void
     *
     * @internal
     */
    public function untrap()
    {}
}
