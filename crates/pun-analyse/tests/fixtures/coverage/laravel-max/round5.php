<?php

namespace LaravelMax\Round5;

/**
 * `get_class()` of mixed is `class-string|false`, so `new` on it is an ErrorType: the call is on mixed, not an
 * undefined method of `object`.
 */
class Restores
{
    /** @param mixed $collection */
    public function restore($collection): void
    {
        $collectionClass = get_class($collection);

        (new $collectionClass())->loadMissing();
    }
}

/**
 * A key of `class-string|false` writes the `false` member as `0` (toArrayKey()).
 *
 * @return array<int|string, int>
 */
function keyed(mixed $provider): array
{
    $events = [];
    $events[get_class($provider)] = 1;

    return $events;
}

/** `str_replace()` of a subject that is no array is a string. */
function replaced(mixed $notifiable): string
{
    $class = str_replace('\\', '.', get_class($notifiable));

    return $class.'.';
}

/**
 * A template the bound maybe accepts infers the members it accepts: `TKey` is `class-string`.
 *
 * @template TKey of array-key
 * @param callable(): TKey $key
 * @return array<TKey, int>
 */
function group(callable $key): array
{
    return [$key() => 1];
}

function grouped(mixed $model): int
{
    $groups = group(fn () => get_class($model));
    foreach ($groups as $class => $n) {
        return $groups[$class];
    }

    return 0;
}

function out(string $a): void
{
}

/**
 * A truthy or falsy offset of a mixed value has an expression type, so its later fetch is not reported again; the
 * holder itself stays mixed.
 */
class Offsets
{
    public function ternary(mixed $table): void
    {
        out(($table['schema'] ? $table['schema'].' / ' : '').$table['table']);
    }

    public function branches(mixed $t): void
    {
        if ($t['e']) {
            out($t['e']);
        } else {
            out($t['e']);
        }
    }

    public function early(mixed $t): void
    {
        if (! $t['f']) {
            return;
        }
        out($t['f']);
        out($t['g']);
    }

    public function nested(mixed $t): void
    {
        if ($t['g'][0]) {
            out($t['g'][0]);
        }
    }

    public function elseif(mixed $t, int $i): void
    {
        if ($i) {
        } elseif ($t['k']) {
            out($t['k']);
        }
    }

    public function property(mixed $t): void
    {
        if ($t->p) {
            out($t->p);
        }
    }
}
