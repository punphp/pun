<?php

namespace ExplicitMixedProducers;

function declared(): mixed { return 1; }
function unknown() {}
/** @return mixed */
function documented() { return 1; }
/** @return non-empty-mixed */
function documentedTruthy() { return 1; }

function nativeParameters(mixed $explicit, $implicit, bool $flag): void
{
    $explicit; // mixed: native-parameter
    $implicit; // mixed: untyped-parameter
    $copy = $explicit;
    $copy; // mixed: assignment-copy
    $explicit = 1;
    $explicit; // mixed: overwritten
    $flag ? $copy : $implicit; // mixed: explicit-implicit
    $flag ? $implicit : $copy; // mixed: implicit-explicit
    $flag ? $copy : 1; // mixed: explicit-integer
    $flag ? 1 : $copy; // mixed: integer-explicit
}

/** @param mixed $explicit */
function documentedParameter($explicit, $implicit): void
{
    $explicit; // mixed: documented-parameter
    $implicit; // mixed: undocumented-parameter
}

function calls(string $text): void
{
    declared(); // mixed: declared-call
    unknown(); // mixed: unknown-call
    documented(); // mixed: documented-call
    documentedTruthy(); // mixed: documented-truthy-call
    json_decode($text); // mixed: json-decode
    unserialize($text); // mixed: unserialize
}

/** @param array<string, mixed> $explicitItems
 * @param array<string> $implicitItems
 * @param array<string, *> $wildcardItems
 * @param array<mixed, mixed> $mixedKeyItems
 */
function offsets(mixed $explicit, $implicit, array $explicitItems, array $implicitItems, array $wildcardItems, array $mixedKeyItems): void
{
    $explicit['key']; // mixed: explicit-offset
    $explicit[0][1]; // mixed: nested-explicit-offset
    $implicit['key']; // mixed: implicit-offset
    $explicitItems['key']; // mixed: explicit-item
    $implicitItems['key']; // mixed: implicit-item
    $wildcardItems['key']; // mixed: wildcard-item
    $mixedKeyItems['key']; // mixed: mixed-key-explicit-item
    $explicit->unknown; // mixed: unknown-property
    $explicit->unknown(); // mixed: unknown-method
    if ($explicit) {
        $explicit[0]; // mixed: truthy-offset-resets-constraints
    }
}

function captures(mixed $explicit, $implicit): void
{
    $a = fn () => $explicit; // mixed: arrow-explicit
    $b = fn () => $implicit; // mixed: arrow-implicit
    $c = fn (mixed $value) => $value; // mixed: arrow-declared
    $d = fn ($value) => $value; // mixed: arrow-untyped
    $copy = $explicit;
    $e = function () use ($copy, $implicit) {
        $copy; // mixed: closure-copy
        $implicit; // mixed: closure-implicit
        return $copy;
    };
    $f = fn () => $copy; // mixed: arrow-copy
    $explicit = 1;
    $g = fn () => $explicit; // mixed: arrow-overwritten
}

/** @param mixed|int $left
 * @param int|mixed $right
 */
function documentedUnions($left, $right): void
{
    $left; // mixed: documented-mixed-integer
    $right; // mixed: documented-integer-mixed
}

class DeclaredProperties
{
    public mixed $explicit;
    public $implicit;
    /** @var mixed */
    public $documented;

    public function read(): void
    {
        $this->explicit; // mixed: declared-property
        $this->implicit; // mixed: untyped-property
        $this->documented; // mixed: documented-property
    }
}

function annotated($implicit): void
{
    /** @var non-empty-mixed $copy */
    $copy = $implicit;
    $copy; // mixed: inline-truthy-declaration
    $copy[0]; // mixed: inline-truthy-offset
}
