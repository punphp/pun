<?php

namespace App;

use Illuminate\Support\Traits\Tappable;

/**
 * @template T
 */
class Box
{
    use Tappable;
}

interface I {}
interface J {}

/**
 * @param I&J $ij
 * @param list<int> $l
 * @param Box<int> $g
 */
function f(mixed $m, $untyped, Box $b, $ij, $l, $g, object $ob)
{
    // PHPStan tells a declared `mixed` (Rector writes `mixed`) from an inferred one (Rector takes the type away);
    // Mago does not, so Pun leaves a `mixed` value alone.
    tap($m, function ($x) {
    });
    tap($untyped, function (Box $x) {
    });
    // Box is generic: whether PHPStan's type of `$b` has generics decides if `Box` is its type; Mago fills them in.
    $b->tap(function (Box $x) {
    });
    tap($g, function (Box $x) {
    });
    // Pun does not compare an intersection hint, nor another case of `array` (Rector may keep either).
    tap($ij, function (I&J $x) {
    });
    tap($l, function (ARRAY $x) {
    });
    // Mago gives `object` where PHPStan resolves a conditional return type to `mixed` (Rector takes the type away),
    // so Pun leaves an `object` value alone.
    tap($ob, function ($x) {
    });
    // a class the codebase does not know
    tap(new Missing(), function ($x) {
    });
}
