<?php
declare(strict_types=1);

class DynamicCaller {
    public function method(): int { return 1; }
}
/** @param callable():int $_cb */
function call_it(callable $_cb): void {}
$obj = new DynamicCaller();
$method = 'method';
$closure = $obj->{$method}(...);
call_it($closure);

/** @param callable(string, int):int $_cb */
function with_arguments(callable $_cb): void {}
/** @param Closure():int $_cb */
function closure_only(callable $_cb): void {}
/** @param pure-Closure():int $_cb */
function pure_closure_only(callable $_cb): void {}

function compatibility(DynamicCaller $obj): void {
    $method = 'method';
    $opaque = $obj->{$method}(...);
    with_arguments($opaque);
    closure_only($opaque);
    pure_closure_only($opaque);
    $known = $obj->method(...);
    call_it($known);
}
