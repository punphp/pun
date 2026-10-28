<?php
namespace CallableProjection;

/** @param callable(int): string $callback */
function views(callable $callback, int $index): void {
    if (is_array($callback)) {
        \PHPStan\dumpType($callback[$index]);
        \PHPStan\dumpType($callback['1']);
        foreach ($callback as $key => $value) {
            \PHPStan\dumpType($key);
            \PHPStan\dumpType($value);
        }
        if (array_is_list($callback)) {
            \PHPStan\dumpType($callback);
            \PHPStan\dumpType($callback(1));
            $callback('bad');
            $callback['extra'] = false;
            \PHPStan\dumpType($callback);
            $callback('bad');
            unset($callback[0]);
            \PHPStan\dumpType($callback);
            $callback('bad');
        }
    }
}

/** @param callable(int): string $callback */
function closures(callable $callback): void {
    if ($callback instanceof \Closure) {
        $copy = $callback;
        $callback = false;
        \PHPStan\dumpType($copy);
        \PHPStan\dumpType($copy(1));
        $copy('bad');
        $callback();
    }
}

/**
 * @param callable(int): string $first
 * @param callable(string): int $second
 */
function alternatives(callable $first, callable $second, bool $flag): void {
    if (is_array($first) && is_array($second)) {
        $selected = $flag ? $first : $second;
        \PHPStan\dumpType($selected);
        $selected(false);
        $selected = 123;
        $selected();
    }
}

/** @param array<int, mixed>&callable(int): string $callback */
function declaredArray(array $callback): void {
    \PHPStan\dumpType($callback);
    \PHPStan\dumpType($callback[0]);
    \PHPStan\dumpType($callback[1]);
    \PHPStan\dumpType($callback(1));
    $callback('bad');
}
