<?php

namespace ClosureFromCallableSignatures;

function optional(string $value = ''): string { return $value; }
function variadic(string ...$values): string { return implode('', $values); }
function mutate(string &$value): void { $value = strtoupper($value); }

/**
 * @template T
 * @param T $value
 * @return T
 */
function identity(mixed $value): mixed { return $value; }

/** @return ($value is string ? string : int) */
function conditional(string|int $value): string|int { return $value; }

class Factory {
    /**
     * @template T
     * @param T $value
     * @return T
     */
    public static function identity(mixed $value): mixed { return $value; }
}

function signatures(string $value): void {
    \PHPStan\dumpType(\Closure::fromCallable('ClosureFromCallableSignatures\\optional'));
    $optional = \Closure::fromCallable('ClosureFromCallableSignatures\\optional');
    \PHPStan\dumpType($optional());
    \PHPStan\dumpType($optional(value: 'value'));
    $optional(123);
    $optional('value', 'extra');
    \PHPStan\dumpType(\Closure::fromCallable('ClosureFromCallableSignatures\\variadic'));
    $variadic = \Closure::fromCallable('ClosureFromCallableSignatures\\variadic');
    \PHPStan\dumpType($variadic());
    \PHPStan\dumpType($variadic('one', 'two'));
    $variadic('one', 123);
    \PHPStan\dumpType(\Closure::fromCallable('ClosureFromCallableSignatures\\mutate'));
    $mutate = \Closure::fromCallable('ClosureFromCallableSignatures\\mutate');
    $mutate($value);
    \PHPStan\dumpType($mutate($value));
    $mutate('literal');
    $identity = \Closure::fromCallable('ClosureFromCallableSignatures\\identity');
    \PHPStan\dumpType($identity('value'));
    \PHPStan\dumpType($identity(123));
    $staticIdentity = \Closure::fromCallable([Factory::class, 'identity']);
    \PHPStan\dumpType($staticIdentity('value'));
    \PHPStan\dumpType($staticIdentity(123));
    $conditional = \Closure::fromCallable('ClosureFromCallableSignatures\\conditional');
    \PHPStan\dumpType($conditional('value'));
    \PHPStan\dumpType($conditional(123));
}
