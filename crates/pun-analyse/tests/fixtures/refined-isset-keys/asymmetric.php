<?php
namespace RefinedIssetAsymmetric;

/**
 * @param array{0: string} $shape
 * @param non-decimal-int-string $inverse
 * @param numeric-string $numeric
 * @param decimal-int-string $decimal
 */
function integerShape(array $shape, string $inverse, string $numeric, string $decimal): void {
    echo isset($shape[$inverse]);
    echo $shape[$inverse] ?? 'absent';
    echo isset($shape[$numeric]);
    echo $shape[$numeric] ?? 'absent';
    echo isset($shape[$decimal]);
    echo $shape[$decimal] ?? 'absent';
}

function stringOffsets(string $numeric, string $integer, string $boolean): void {
    echo isset($numeric['0']);
    echo $numeric['0'] ?? 'absent';
    echo isset($integer[0]);
    echo $integer[0] ?? 'absent';
    echo isset($boolean[false]);
    echo $boolean[false] ?? 'absent';
}

/**
 * @param array<string, string>|string $value
 * @param decimal-int-string $key
 */
function mixedReceivers(array|string $value, string $key): void {
    echo isset($value[$key]);
    echo $value[$key] ?? 'absent';
}

/** @param non-empty-list<string> $list */
function listOffsets(array $list): void {
    echo isset($list['0']);
    echo $list['0'] ?? 'absent';
    echo isset($list[0]);
    echo $list[0] ?? 'absent';
}

/**
 * @param array{0: string} $shape
 * @param numeric-string&non-decimal-int-string $key
 */
function inverseNumericKey(array $shape, string $key): void {
    echo isset($shape[$key]);
    echo $shape[$key] ?? 'absent';
}
