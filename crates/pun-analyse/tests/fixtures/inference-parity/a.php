<?php

declare(strict_types=1);

namespace App;

enum Suit: string
{
    case A = 'a';
    case B = 'b';
}

enum Plain
{
    case X;
}

interface Marker {}

final class Impl implements Marker {}

/**
 * @template TEnum of \BackedEnum
 *
 * @param  class-string<TEnum>  $c
 * @return TEnum|null
 */
function mk(string $c): ?\BackedEnum
{
    return null;
}

/**
 * @template TEnum of \UnitEnum
 *
 * @param  class-string<TEnum>  $c
 * @return TEnum|null
 */
function mu(string $c): ?\UnitEnum
{
    return null;
}

/**
 * @template T of Marker
 *
 * @param  class-string<T>  $c
 * @return T|null
 */
function mm(string $c): ?Marker
{
    return null;
}

/**
 * @template T of object
 *
 * @param  class-string<T>  $c
 * @return T|null
 */
function mo(string $c): ?object
{
    return null;
}

final class Req
{
    /**
     * @template TEnum of \BackedEnum
     *
     * @param  class-string<TEnum>  $enumClass
     * @param  TEnum|null  $default
     * @return TEnum|null
     */
    public function enum(string $key, string $enumClass, $default = null): ?\BackedEnum
    {
        return $default;
    }
}

function t(Req $r): void
{
    \PHPStan\dumpType(mk(Suit::class));
    \PHPStan\dumpType(mu(Suit::class));
    \PHPStan\dumpType(mu(Plain::class));
    \PHPStan\dumpType(mm(Impl::class));
    \PHPStan\dumpType(mo(Suit::class));
    \PHPStan\dumpType($r->enum('k', Suit::class));
    \PHPStan\dumpType($r->enum('k', Suit::class, Suit::A));
}
