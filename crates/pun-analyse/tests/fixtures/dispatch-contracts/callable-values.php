<?php

declare(strict_types=1);

namespace DispatchCallables;

function ordinary(\Closure $callback): mixed
{
    return $callback(1);
}

/** @param pure-Closure $callback */
function pure(\Closure $callback): mixed
{
    return $callback(1);
}

function narrowed(object $callback): mixed
{
    if ($callback instanceof \Closure) {
        return $callback(1);
    }
    return null;
}

function closurePipe(\Closure $callback): mixed
{
    return 1 |> $callback;
}

/**
 * @template T of \Closure
 * @param T $callback
 * @return T
 */
function genericClosure(\Closure $callback): \Closure
{
    1 |> $callback;
    return $callback;
}

function discovered(object $callback): void
{
    if (method_exists($callback, '__invoke')) {
        1 |> $callback;
    }
}

final class Reference
{
    public function __invoke(int &$value): int
    {
        return $value;
    }
}

final class NotCallable
{
}

function controls(Reference $reference, NotCallable $wrong): mixed
{
    $result = 1 |> $reference;
    return $result |> $wrong;
}
