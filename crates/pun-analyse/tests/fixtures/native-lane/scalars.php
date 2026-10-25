<?php
namespace NativeLane;

/** @return non-empty-string */
function documented(): string { return 'value'; }

/** @return non-empty-string */
function untypedReturn() { return 'value'; }

/** @param positive-int $integer
 * @param non-empty-string $text
 * @param 'yes' $untyped
 */
function scalars(int $integer, string $text, bool $flag, float $decimal, $untyped, ?int $nullable): void
{
    $integer; // native
    $text; // native
    $flag; // native
    $decimal; // native
    $untyped; // native
    $nullable; // native
    $copy = $integer;
    $copy; // native
    $integer = 7;
    $integer; // native
    $chain = ($copy = 9);
    $chain; // native
    $copy; // native
    $text = 'constant';
    $text; // native
    $flag = false;
    $flag; // native
    $decimal = 1.5;
    $decimal; // native
    $nullable = null;
    $nullable; // native
    $result = documented();
    $result; // native
    $result = untypedReturn();
    $result; // native
    /** @var non-empty-string $annotated */
    $annotated = documented(); // native
    $annotated; // native
    /** @var non-empty-string */
    $unnamed = 'literal'; // native
    $unnamed; // native
    $preserved = 'before';
    /** @var non-empty-string $preserved */
    $other = 1;
    $preserved; // native
    /** @var non-empty-string $preserved */
    $preserved; // native
    true; // native
    ''; // native
    /** @var non-empty-string */
    $outer = ($inner = 'nested');
    $outer; // native
    $inner; // native
    /** @var non-empty-string */
    $outer = ($outer = 'same');
    $outer; // native
}

/** @param non-empty-string $value */
function implicitNullable(string $value = null): void
{
    $value; // native
}

$top = 17;
$top; // native
