<?php
namespace CallableBranch;

function native(callable $callback): mixed {
    if ($callback instanceof \Closure) {
        \PHPStan\dumpType($callback);
        call_user_func_array($callback, []);
    } elseif (is_array($callback)) {
        \PHPStan\dumpType($callback);
        \PHPStan\dumpType($callback[0]);
        \PHPStan\dumpType($callback[1]);
        \PHPStan\dumpType(count($callback));
        call_user_func_array($callback, []);
    }
    \PHPStan\dumpType($callback);
    return call_user_func_array($callback, []);
}

function arrayFirst(callable $callback): mixed {
    if (is_array($callback)) {
        \PHPStan\dumpType($callback);
        call_user_func_array($callback, []);
    } elseif ($callback instanceof \Closure) {
        \PHPStan\dumpType($callback);
    }
    \PHPStan\dumpType($callback);
    return call_user_func_array($callback, []);
}

/** @param callable(int): string $callback */
function signature(callable $callback): void {
    if ($callback instanceof \Closure) {
        \PHPStan\dumpType($callback);
        \PHPStan\dumpType($callback(1));
        $callback('bad');
    } elseif (is_array($callback)) {
        \PHPStan\dumpType($callback);
        \PHPStan\dumpType($callback(1));
        $callback('bad');
        \PHPStan\dumpType($callback[0]);
        \PHPStan\dumpType($callback[1]);
    }
    \PHPStan\dumpType($callback);
    \PHPStan\dumpType($callback(1));
    $callback('bad');
}

function replaceArray(callable $callback): void {
    if (is_array($callback)) {
        $callback = 123;
    }
    \PHPStan\dumpType($callback);
    call_user_func_array($callback, []);
}

function replaceClosure(callable $callback): void {
    if ($callback instanceof \Closure) {
        $callback = 123;
    }
    \PHPStan\dumpType($callback);
    call_user_func_array($callback, []);
}

function writeOffset(callable $callback): void {
    if (is_array($callback)) {
        $callback[1] = 123;
        \PHPStan\dumpType($callback);
        call_user_func_array($callback, []);
    }
}

function unsetOffset(callable $callback): void {
    if (is_array($callback)) {
        unset($callback[1]);
        \PHPStan\dumpType($callback);
        call_user_func_array($callback, []);
    }
}

/** @param array{class-string|object, non-empty-string} $callback */
function ordinaryTuple(array $callback): void {
    \PHPStan\dumpType($callback);
    call_user_func_array($callback, []);
    $callback();
}

/** @param callable(int): string $callback */
function copySignature(callable $callback): void {
    if (is_array($callback)) {
        $copy = $callback;
        \PHPStan\dumpType($copy);
        \PHPStan\dumpType($copy(1));
        $copy('bad');
        $callback = 123;
        \PHPStan\dumpType($copy(1));
        call_user_func_array($callback, []);
    }
}

function branchReplacement(callable $callback, bool $flag): void {
    if (is_array($callback)) {
        if ($flag) {
            $callback = [new \stdClass(), 'missing'];
        }
        \PHPStan\dumpType($callback);
        call_user_func_array($callback, []);
    }
}

function noOpArrayFacts(callable $callback): void {
    if (is_array($callback)) {
        if (count($callback) === 2) {
            \PHPStan\dumpType($callback);
            call_user_func_array($callback, []);
        }
        if (is_string($callback[1])) {
            \PHPStan\dumpType($callback);
            call_user_func_array($callback, []);
        }
    }
}
