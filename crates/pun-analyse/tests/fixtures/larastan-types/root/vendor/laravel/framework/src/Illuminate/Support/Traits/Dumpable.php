<?php

namespace Illuminate\Support\Traits;

trait Dumpable
{
    /**
     * Dump the given arguments and terminate execution.
     *
     * @param  mixed  ...$args
     * @return never
     */
    public function dd(...$args)
    {}

    /**
     * Dump the given arguments.
     *
     * @param  mixed  ...$args
     * @return $this
     */
    public function dump(...$args)
    {}
}
