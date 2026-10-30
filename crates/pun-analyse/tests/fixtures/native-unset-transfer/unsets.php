<?php

namespace NativeUnsetTransfer;

function reported(): void
{
    $shape = ['present' => 1];
    $shape; // observe
    unset($shape['present']);
    $shape; // observe
    isset($shape['present']); // observe
    $shape['present'] ?? 7; // observe
}

function locals(int $keep): void
{
    $removed = 1;
    unset($removed);
    $keep; // observe
    $removed = 99;
    $removed; // observe
    unset($removed);
    isset($removed); // observe
    $keep; // observe
}

function removedRead(int $keep): void
{
    $removed = 1;
    unset($removed);
    $removed; // unavailable
    $keep; // unavailable
}

function orderedOperands(): void
{
    $shape = ['first' => 1, 'second' => 2, 'third' => 3];
    $key = 'first';
    unset($shape[$key], $shape[$key = 'second']);
    $shape; // observe
    $key; // observe
    unset($key, $shape['third']);
    $shape; // observe
    $key = 'reassigned';
    $key; // observe
}

function missingAndNumericKeys(): void
{
    $shape = ['first' => 1, 2 => 'two', '03' => 3];
    unset($shape['absent']);
    $shape; // observe
    unset($shape['2']);
    $shape; // observe
    unset($shape['03']);
    $shape; // observe
}

function lists(): void
{
    $tail = [1, 2, 3];
    unset($tail[2]);
    $tail; // observe
    $middle = [1, 2, 3];
    unset($middle[1]);
    $middle; // observe
    unset($middle[-1]);
    $middle; // observe
    $single = [1];
    unset($single[0]);
    $single; // observe
    unset($single[0]);
    $single; // observe
}

function optional(bool $flag): void
{
    if ($flag) {
        $shape = ['first' => 1, 'second' => 2];
    } else {
        $shape = ['second' => 2];
    }
    unset($shape['first']);
    $shape; // observe
}

function unionKey(bool $flag): void
{
    $key = $flag ? 'first' : 'second';
    $shape = ['first' => 1, 'second' => 2, 'third' => 3];
    unset($shape[$key]);
    $shape; // observe
    $key; // observe
}

function integerKey(int $key): void
{
    $shape = [1, 2, 3];
    unset($shape[$key]);
    $shape; // observe
}

/** @param array{first: int, second: string} $shape */
function documentedArray(array $shape): void
{
    $shape; // observe
    unset($shape['first']);
    $shape; // observe
    $shape['second']; // observe
}

function rootWrittenByKey(): void
{
    $shape = ['old' => 1];
    unset($shape[($shape = ['new' => 2])[0] ?? 'new']);
    $shape; // observe
}

function nestedBoundary(int $keep): void
{
    $shape = ['outer' => ['leaf' => 1]];
    unset($shape['outer']['leaf']);
    $shape; // unavailable
    $keep; // unavailable
    $shape = [];
    $shape; // unavailable
}

function stringKeyBoundary(string $key): void
{
    $shape = [1, 2];
    unset($shape[$key]);
    $shape; // unavailable
}

function scalarKeyBoundary(): void
{
    $shape = [1, 2];
    unset($shape[true]);
    $shape; // unavailable
}

function annotatedBoundary(): void
{
    /** @var array{first: int} $shape */
    $shape = ['first' => 1];
    unset($shape['first']);
    $shape; // unavailable
}

function propertyBoundary(Box $box, int $keep): void
{
    unset($box->value);
    $keep; // unavailable
}

function dynamicBoundary(int $keep): void
{
    $name = 'keep';
    unset($$name);
    $keep; // unavailable
}

function referenceBoundary(int $keep): void
{
    $alias =& $keep;
    unset($alias);
    $keep; // unavailable
}

function unavailableKey(int $keep): void
{
    $shape = [1, 2];
    unset($shape[strlen('first')]);
    $keep; // unavailable
}

function specialBoundary(int $_local, int $keep): void
{
    unset($_local);
    $keep; // unavailable
}

function keyWithWrite(int $value): int
{
    return $value;
}

function argumentWrites(): void
{
    $shape = [1, 2];
    unset($shape[keyWithWrite($written = 0)]);
    $written; // observe
    $shape; // observe
}

$global = 1;
$other = 2;
unset($global);
$other; // observe
$global; // unavailable

class Box
{
    public int $value = 1;
}
