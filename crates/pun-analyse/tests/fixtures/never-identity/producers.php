<?php
namespace ArrowNeverProbe;

function declaredStop(): never { exit(); }
function directExit(): void { exit(); }
function directDie(): void { die(); }
function directThrow(): void { throw new \RuntimeException(); }
function directNamed(): void { declaredStop(); }

function arrows(): void {
    $exit = fn () => exit();
    $exitHint = fn (): never => exit();
    $throw = fn () => throw new \RuntimeException();
    $throwHint = fn (): never => throw new \RuntimeException();
    $named = fn () => declaredStop();
    $namedHint = fn (): never => declaredStop();
    $closure = function () { exit(); };
    $implicit = fn () => implicitBottom();
    $accepting = fn () => acceptingBottom();
    $nonaccepting = fn () => nonacceptingBottom();
}

function forwarded(): void { \call_user_func(fn () => exit()); }
function implicitBottom(): mixed { return 1; }
function acceptingBottom(): mixed { return 1; }
function nonacceptingBottom(): mixed { return 1; }

function dieArrows(): void {
    $die = fn () => die();
    $dieHint = fn (): never => die();
}
function forwardedDie(): void { \call_user_func(fn () => die()); }
function forwardedThrow(): void { \call_user_func(fn () => throw new \RuntimeException()); }
function forwardedNamed(): void { \call_user_func(fn () => declaredStop()); }

function consume(mixed $value): void {}
function afterExit(): void { exit(); echo 'after-exit'; }
function afterDie(): void { die(); echo 'after-die'; }
function afterThrow(): void { throw new \RuntimeException(); echo 'after-throw'; }
function afterNamed(): void { declaredStop(); echo 'after-named'; }
function afterForwarded(): void { \call_user_func(fn () => exit()); echo 'after-forwarded'; }
function nestedForwarded(): void { consume(\call_user_func(fn () => exit())); echo 'after-nested-forwarded'; }
function afterImplicit(): void { implicitBottom(); echo 'after-implicit'; }
function afterAccepting(): void { acceptingBottom(); echo 'after-accepting'; }
function afterNonaccepting(): void { nonacceptingBottom(); echo 'after-nonaccepting'; }

function afterCurrentFunction(): void {
    $current = \Closure::getCurrent();
    $current();
    echo 'after-current-function';
}
final class CurrentMethod {
    public function afterCurrentMethod(): void {
        $current = \Closure::getCurrent();
        $current();
        echo 'after-current-method';
    }
}
$current = \Closure::getCurrent();
$current();
echo 'after-current-global';
