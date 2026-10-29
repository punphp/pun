<?php
declare(strict_types=1);

class ControlTarget {
    public function method(): int { return 1; }
}
/** @param callable():int $_cb */
function call_it(callable $_cb): void {}
/** @param Closure():int $_cb */
function closure_only(callable $_cb): void {}
/** @param pure-callable():int $_cb */
function pure_callable_only(callable $_cb): void {}
/** @param callable():string $_cb */
function string_result(callable $_cb): void {}

final class Invokable {
    public function __invoke(): int { return 1; }
}
final class NotCallable {}

function controls(ControlTarget $obj): void {
    $method = 'method';
    $opaque = $obj->{$method}(...);
    pure_callable_only($opaque);
    $known = $obj->method(...);
    string_result($known);
    $invokable = new Invokable();
    call_it($invokable);
    closure_only($invokable);
    call_it(new NotCallable());
}

/**
 * @template T of Closure
 * @param T&(callable():string) $refined
 */
function refined_signature(callable $refined): void {
    call_it($refined);
}
