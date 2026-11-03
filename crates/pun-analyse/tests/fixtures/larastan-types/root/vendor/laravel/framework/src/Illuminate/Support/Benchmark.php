<?php

namespace Illuminate\Support;

use Closure;
use Illuminate\Support\Traits\Macroable;

class Benchmark
{
    use Macroable;

    /**
     * Measure a callable or array of callables over the given number of iterations.
     *
     * @param  \Closure|array<callable>  $benchmarkables
     * @param  int  $iterations
     * @return array<float|int|null>|float
     */
    public static function measure(Closure|array $benchmarkables, int $iterations = 1): array|float
    {}

    /**
     * Measure a callable once and return the result and duration in milliseconds.
     *
     * @template TReturn of mixed
     *
     * @param  (callable(): TReturn)  $callback
     * @return array{0: TReturn, 1: float}
     */
    public static function value(callable $callback): array
    {}

    /**
     * Measure a callable or array of callables over the given number of iterations, then dump and die.
     *
     * @param  \Closure|array<callable>  $benchmarkables
     * @param  int  $iterations
     * @return never
     */
    public static function dd(Closure|array $benchmarkables, int $iterations = 1): never
    {}
}
