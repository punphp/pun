<?php

namespace HelperFunctionDiscard;

use function HelperFunctionDiscard\required as imported;

#[\NoDiscard]
function required(int $value = 1): int { echo $value; return $value; }
function allowed(int $value = 1): int { echo $value; return $value; }

class Required
{
    #[\NoDiscard]
    public function __invoke(): int { echo 'invoke'; return 1; }
    #[\NoDiscard]
    public function method(): int { echo 'method'; return 1; }
    #[\NoDiscard]
    public static function stat(): int { echo 'static'; return 1; }
    public function allowed(): int { echo 'allowed'; return 1; }
}

class Allowed
{
    public function __invoke(): int { echo 'invoke'; return 1; }
    public static function stat(): int { echo 'static'; return 1; }
}

interface Contract
{
    #[\NoDiscard]
    public function __invoke(): int;
}

interface Marker {}

/** @param Required|Allowed $union */
function callbacks(Required $required, Allowed $allowed, \Closure $closure, callable $callback, bool $flag, mixed $mixed, $union): void
{
    imported();
    (void) imported();
    (void) allowed();
    (void) strlen('abc');
    (void) \strlen('abc');
    $required();
    (void) $required();
    $allowed();
    (void) $allowed();
    $closure();
    (void) $closure();
    $callback();
    (void) $callback();
    $mixed();
    (void) $mixed();
    $union();
    (void) $union();

    $pair = [$required, 'method'];
    $pair();
    (void) $pair();
    $pair = [$required, 'allowed'];
    (void) $pair();
    $pair = [Required::class, 'stat'];
    $pair();
    $pair = [Allowed::class, 'stat'];
    (void) $pair();
    $pair = [1 => 'stat', 0 => Required::class];
    $pair();
    $pair = [1 => 'stat', 0 => Allowed::class];
    (void) $pair();
    $method = Required::class . '::stat';
    $method();
    $method = Allowed::class . '::stat';
    (void) $method();
    $method = Required::class . '::method';
    (void) $method();
    $missing = 'NonexistentDiscardFunction';
    (void) $missing();
    $invalid = [Required::class, 'method'];
    (void) $invalid();
    $invalid = [$mixed, 'method'];
    (void) $invalid();
    $invalid = [Required::class];
    (void) $invalid();
    $invalid = [2 => Required::class, 3 => 'stat'];
    (void) $invalid();

    $cb = $required->method(...);
    $cb();
    $cb = Required::stat(...);
    $cb();
    $cb = $required->allowed(...);
    (void) $cb();
    $cb = $flag ? required(...) : allowed(...);
    $cb();
    (void) $cb();
    $cb = #[\NoDiscard] fn () => 2.5;
    $cb();
    $cb = #[\NoDiscard] fn () => ['answer' => true, 3 => null];
    $cb();
    $cb = #[\NoDiscard] function (int &$i, string $s = '', float ...$values): string { return $s; };
    $cb($i);
    $cb = fn () => 17;
    (void) $cb();
    $cb = #[\NoDiscard] fn ($v) => $v;
    $cb(1);
    $cb = 'HelperFunctionDiscard\\required';
    (void) $cb();
    $cb = $flag ? 'HelperFunctionDiscard\\required' : 'NonexistentDiscardFunction';
    (void) $cb();
    $cb = $flag ? 'HelperFunctionDiscard\\required' : 'HelperFunctionDiscard\\allowed';
    $cb();
    (void) $cb();
    3 |> required(...);
    (void) 3 |> allowed(...);
    3 |> required(1);
    3 |> (fn ($x) => required($x));
    (void) 3 |> (fn ($x) => allowed($x));
    required(...);
}

/** @param Contract&Marker $cb */
function intersection($cb): void
{
    $cb();
    (void) $cb();
}

/** @param callable(): int $cb */
function typedCallable(callable $cb): void
{
    (void) $cb();
}

/** @param array{0: Required, 1?: 'method'} $cb */
function optionalPair(array $cb): void
{
    (void) $cb();
}

/** @param list<Required|'method'> $cb */
function genericPair(array $cb): void
{
    (void) $cb();
}

function namespaceFallback(): void
{
    (void) phpversion();
    missing();
}

/** @return array{answer?: bool} */
#[\NoDiscard]
function optionalResult(): array
{
    return random_int(0, 1) ? ['answer' => true] : [];
}

function builtinAndOptionalResult(): void
{
    $cb = 'DateTimeImmutable::createFromFormat';
    (void) $cb('Y', '2026');
    $cb = optionalResult(...);
    $cb();
}
