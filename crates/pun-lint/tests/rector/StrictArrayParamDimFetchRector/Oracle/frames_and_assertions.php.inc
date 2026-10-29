<?php

namespace Probe;

class Arr
{
    /**
     * @template TKey
     * @template TValue
     *
     * @param  iterable<TKey, TValue>  $array
     * @param  (callable(TValue, TKey): bool)|null  $callback
     * @return TValue|null
     */
    public static function first($array, ?callable $callback = null)
    {
        return null;
    }
}

function backtrace()
{
    return Arr::first(debug_backtrace(DEBUG_BACKTRACE_IGNORE_ARGS), function ($trace) {
        return $trace['function'] !== 'x';
    });
}

function backtraceNoFlag()
{
    return Arr::first(debug_backtrace(), function ($trace) {
        return $trace['function'] !== 'x';
    });
}

/**
 * @param list<array{function: string, line?: int}> $frames
 */
function shapes(array $frames)
{
    return Arr::first($frames, function ($frame) {
        return $frame['function'] !== 'x';
    });
}

/**
 * @param list<array<string, int>> $rows
 */
function general(array $rows)
{
    return Arr::first($rows, function ($row) {
        return $row['a'] !== 1;
    });
}
-----
<?php

namespace Probe;

class Arr
{
    /**
     * @template TKey
     * @template TValue
     *
     * @param  iterable<TKey, TValue>  $array
     * @param  (callable(TValue, TKey): bool)|null  $callback
     * @return TValue|null
     */
    public static function first($array, ?callable $callback = null)
    {
        return null;
    }
}

function backtrace()
{
    return Arr::first(debug_backtrace(DEBUG_BACKTRACE_IGNORE_ARGS), function ($trace) {
        return $trace['function'] !== 'x';
    });
}

function backtraceNoFlag()
{
    return Arr::first(debug_backtrace(), function ($trace) {
        return $trace['function'] !== 'x';
    });
}

/**
 * @param list<array{function: string, line?: int}> $frames
 */
function shapes(array $frames)
{
    return Arr::first($frames, function ($frame) {
        return $frame['function'] !== 'x';
    });
}

/**
 * @param list<array<string, int>> $rows
 */
function general(array $rows)
{
    return Arr::first($rows, function (array $row) {
        return $row['a'] !== 1;
    });
}
