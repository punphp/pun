<?php

namespace VoidValues;

function sink(): void {}
function takesNull(null $value): void {}
final class Receiver {
    public function sink(): void {}
    public static function staticSink(): void {}
}

function calls(Receiver $receiver): void {
    $name = 'sink';
    $function = __NAMESPACE__ . '\\sink';
    $firstClass = sink(...);
    $named = sink();
    $method = $receiver->sink();
    $dynamic = $receiver->$name();
    $static = Receiver::staticSink();
    $string = $function();
    $closure = $firstClass();
    $forwarded = call_user_func($firstClass);
    takesNull(sink());
    takesNull($receiver->sink());
    takesNull(Receiver::staticSink());
    takesNull($function());
    takesNull($firstClass());
    sink();
    $receiver->sink();
    Receiver::staticSink();
}

function matches(bool $flag): void {
    $all = match ($flag) { true => sink(), false => sink() };
    $some = match ($flag) { true => sink(), false => 1 };
    match ($flag) { true => sink(), false => sink() };
    takesNull(match ($flag) { true => sink(), false => sink() });
}
