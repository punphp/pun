<?php declare(strict_types=1);

namespace FailedStringOffsets;

/** @param non-empty-string $value */
function indexes(string $value, int $valid, string $invalid, int|string $uncertain, mixed $unknown): void
{
    \PHPStan\dumpType($value[0]);
    \PHPStan\dumpType($value[-1]);
    \PHPStan\dumpType($value[$valid]);
    \PHPStan\dumpType($value['foo']);
    \PHPStan\dumpType($value['1']);
    \PHPStan\dumpType($value[1.5]);
    \PHPStan\dumpType($value[false]);
    \PHPStan\dumpType($value[null]);
    \PHPStan\dumpType($value[$invalid]);
    \PHPStan\dumpType($value[$uncertain]);
    \PHPStan\dumpType($value[$unknown]);
    $copy = $value['missing'];
    \PHPStan\dumpType($copy);
}

function literalParent(): void
{
    $value = 'Foo';
    \PHPStan\dumpType($value['foo']);
    $value['foo'] += null;
}

/**
 * @template T of int|string
 * @param non-empty-string $value
 * @param T $index
 */
function uncertainTemplate(string $value, int|string $index): void
{
    \PHPStan\dumpType($value[$index]);
}

/**
 * @template T of string
 * @param non-empty-string $value
 * @param T $index
 */
function invalidTemplate(string $value, string $index): void
{
    \PHPStan\dumpType($value[$index]);
}

/** @param non-empty-string $value */
function indexOnce(string $value): void
{
    $index = 0.5;
    $result = $value[$index++];
    \PHPStan\dumpType($result);
    \PHPStan\dumpType($index);
    \PHPStan\dumpType($value);
}

/** @param non-empty-string $value */
function coalesceControl(string $value): void
{
    \PHPStan\dumpType($value['missing'] ?? 'fallback');
    \PHPStan\dumpType($value[0] ?? 'fallback');
}

/**
 * @param non-empty-string $value
 * @param numeric-string $numeric
 */
function otherScalarIndexes(string $value, float $float, bool $bool, string $numeric): void
{
    \PHPStan\dumpType($value[$float]);
    \PHPStan\dumpType($value[$bool]);
    \PHPStan\dumpType($value[$numeric]);
    \PHPStan\dumpType($value[1.0]);
}

/**
 * @param non-empty-string $value
 * @param scalar $scalar
 * @param numeric $numeric
 * @param array-key $key
 */
function integerCapableIndexes(string $value, mixed $scalar, int|float|string $numeric, int|string $key, int|float $number): void
{
    \PHPStan\dumpType($value[$scalar]);
    \PHPStan\dumpType($value[$numeric]);
    \PHPStan\dumpType($value[$key]);
    \PHPStan\dumpType($value[$number]);
}

/**
 * @template T of float
 * @param non-empty-string $value
 * @param T $index
 */
function invalidFloatTemplate(string $value, float $index): void
{
    \PHPStan\dumpType($value[$index]);
}

/**
 * @param int<1, 2> $bounded
 * @param int<-4, -2> $negative
 * @param int<2, max> $from
 * @param int<min, -3> $to
 * @param int<-5, -3> $partial
 * @param int<3, 9> $singleton
 * @param int<-6, -5> $below
 * @param int<4, 7> $above
 */
function literalRanges(int $general, int $bounded, int $negative, int $from, int $to, int $partial, int $singleton, int $below, int $above): void
{
    $value = 'abcd';
    \PHPStan\dumpType($value[$general]);
    \PHPStan\dumpType($value[$bounded]);
    \PHPStan\dumpType($value[$negative]);
    \PHPStan\dumpType($value[$from]);
    \PHPStan\dumpType($value[$to]);
    \PHPStan\dumpType($value[$partial]);
    \PHPStan\dumpType($value[$singleton]);
    \PHPStan\dumpType($value[$below]);
    \PHPStan\dumpType($value[$above]);
    $value = 'aba';
    \PHPStan\dumpType($value[$general]);
}

/**
 * @param 0|1 $adjacent
 * @param 0|7 $partlyValid
 * @param 5|7 $invalid
 * @param -2|1 $separate
 * @param int<-4, -1>|int<0, 3> $joined
 * @param int<-4, -2>|int<1, 3> $disjoint
 * @param int<0, 1>|7 $rangeAndMissing
 * @param 0|int<1, max> $constantAndRange
 */
function literalIntegerUnions(int $adjacent, int $partlyValid, int $invalid, int $separate, int $joined, int $disjoint, int $rangeAndMissing, int $constantAndRange): void
{
    $value = 'abcd';
    \PHPStan\dumpType($value[$adjacent]);
    \PHPStan\dumpType($value[$partlyValid]);
    \PHPStan\dumpType($value[$invalid]);
    \PHPStan\dumpType($value[$separate]);
    \PHPStan\dumpType($value[$joined]);
    \PHPStan\dumpType($value[$disjoint]);
    \PHPStan\dumpType($value[$rangeAndMissing]);
    \PHPStan\dumpType($value[$constantAndRange]);
}

/**
 * @param int<2, 3> $multibyte
 * @param int<-2, -1> $negative
 * @param int<1, 2> $binary
 * @param int<0, 1> $zero
 */
function literalRangeBytes(int $multibyte, int $negative, int $binary, int $zero): void
{
    $value = 'éab';
    \PHPStan\dumpType($value[$multibyte]);
    \PHPStan\dumpType($value[$negative]);
    $value = "\xffab";
    \PHPStan\dumpType($value[$binary]);
    $value = "\0a";
    \PHPStan\dumpType($value[$zero]);
}

/** @param int<1, 2> $range */
function literalEmptyFallbacks(int $general, int $range, int|string $uncertain, string $invalid, mixed $unknown, string $generic): void
{
    $value = '';
    \PHPStan\dumpType($value[$general]);
    \PHPStan\dumpType($value[$range]);
    \PHPStan\dumpType($value[$uncertain]);
    \PHPStan\dumpType($value[$invalid]);
    \PHPStan\dumpType($value[$unknown]);
    \PHPStan\dumpType($generic[$general]);
}

/**
 * @param int<0, 128> $at
 * @param int<0, 129> $over
 * @param int<-65, 63> $negativeAt
 * @param int<-65, 64> $negativeOver
 * @param int<0, 1> $short
 */
function finiteEnumerationLimits(int $general, int $at, int $over, int $negativeAt, int $negativeOver, int $short): void
{
    $value = 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab';
    \PHPStan\dumpType($value[$at]);
    \PHPStan\dumpType($value[$over]);
    $value = 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab';
    \PHPStan\dumpType($value[$over]);
    \PHPStan\dumpType($value[$short]);
    $value = 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab';
    \PHPStan\dumpType($value[$general]);
    $value = 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab';
    \PHPStan\dumpType($value[$general]);
    \PHPStan\dumpType($value[$negativeAt]);
    \PHPStan\dumpType($value[$negativeOver]);
}

/**
 * @template T of int
 * @param T $index
 */
function literalGeneralTemplate(int $index): void
{
    $value = 'abcd';
    \PHPStan\dumpType($value[$index]);
}

/**
 * @template T of int<1, 2>
 * @param T $index
 */
function literalRangeTemplate(int $index): void
{
    $value = 'abcd';
    \PHPStan\dumpType($value[$index]);
}

/**
 * @template T of 0
 * @param T $index
 */
function literalConstantTemplate(int $index): void
{
    $value = 'abcd';
    \PHPStan\dumpType($value[$index]);
}

/**
 * @template T of 0|1
 * @param T $index
 */
function literalUnionTemplate(int $index): void
{
    $value = 'abcd';
    \PHPStan\dumpType($value[$index]);
}

/**
 * @template T of int<8, 9>
 * @param T $index
 */
function literalInvalidTemplate(int $index): void
{
    $value = 'abcd';
    \PHPStan\dumpType($value[$index]);
}

/** @param int<1, 5> $index */
function partialRangeCoalesce(int $index): void
{
    $value = 'abc';
    \PHPStan\dumpType($value[$index] ?? 'rangeFallback');
}

function emptyRangeCoalesce(int $index): void
{
    $value = '';
    \PHPStan\dumpType($value[$index] ?? 'emptyRangeFallback');
}

function genericStringOffsetFallback(string $value, int $index): void
{
    \PHPStan\dumpType($value[$index] ?? '');
}

/** @param int<3, 9> $index */
function singletonRangeFallback(int $index): void
{
    $value = 'abcd';
    \PHPStan\dumpType($value[$index] ?? '');
}

/** @param 0|1 $index */
function validIntegerUnionFallback(int $index): void
{
    $value = 'abcd';
    \PHPStan\dumpType($value[$index] ?? '');
}

/** @param 0|7 $index */
function partialIntegerUnionFallback(int $index): void
{
    $value = 'abcd';
    \PHPStan\dumpType($value[$index] ?? '');
}

/**
 * @template T of int<1, 2>
 * @param T $index
 */
function validIntegerTemplateFallback(int $index): void
{
    $value = 'abcd';
    \PHPStan\dumpType($value[$index] ?? '');
}

/**
 * @template T of int<8, 9>
 * @param T $index
 */
function invalidIntegerTemplateFallback(int $index): void
{
    $value = 'abcd';
    \PHPStan\dumpType($value[$index] ?? '');
}

function cachedEmptyStringReads(int $index): void
{
    $value = '';
    \PHPStan\dumpType($value[$index]);
    \PHPStan\dumpType($value[$index] ?? 'cachedStringFallback');
    \PHPStan\dumpType($value[$index]);
    \PHPStan\dumpType(isset($value[$index]));
    \PHPStan\dumpType($value[$index]);
    \PHPStan\dumpType(empty($value[$index]));
    \PHPStan\dumpType($value[$index]);
}

function cachedMissingArrayReads(): void
{
    $value = ['kept' => 1];
    $key = 'missing';
    \PHPStan\dumpType($value[$key]);
    \PHPStan\dumpType($value[$key] ?? 'cachedArrayFallback');
    \PHPStan\dumpType($value[$key]);
    \PHPStan\dumpType(isset($value[$key]));
    \PHPStan\dumpType($value[$key]);
    \PHPStan\dumpType(empty($value[$key]));
    \PHPStan\dumpType($value[$key]);
}

function literalMissingReads(): void
{
    $value = 'foo';
    \PHPStan\dumpType($value[4]);
    \PHPStan\dumpType($value[4] ?? 'literalStringFallback');
    \PHPStan\dumpType(isset($value[4]));
    \PHPStan\dumpType(empty($value[4]));
    \PHPStan\dumpType($value[4]);
    $value = ['kept' => 1];
    \PHPStan\dumpType($value['missing']);
    \PHPStan\dumpType($value['missing'] ?? 'literalArrayFallback');
    \PHPStan\dumpType(isset($value['missing']));
    \PHPStan\dumpType(empty($value['missing']));
    \PHPStan\dumpType($value['missing']);
}

/** @param array<string, int|null> $value */
function validNarrowedCache(array $value, string $key): void
{
    if (isset($value[$key])) {
        \PHPStan\dumpType($value[$key]);
        \PHPStan\dumpType($value[$key] ?? 'unusedNarrowedFallback');
        \PHPStan\dumpType(isset($value[$key]));
        \PHPStan\dumpType($value[$key]);
    }
}

/** @param array<string, int> $value */
function possiblyAbsentCache(array $value, string $key): void
{
    \PHPStan\dumpType($value[$key]);
    \PHPStan\dumpType($value[$key] ?? 'possiblyAbsentFallback');
    \PHPStan\dumpType($value[$key]);
    \PHPStan\dumpType(isset($value[$key]));
    \PHPStan\dumpType(empty($value[$key]));
}

function cachedReceiverAndIndexChanges(int $general): void
{
    $value = '';
    \PHPStan\dumpType($value[$general]);
    $value = 'foo';
    \PHPStan\dumpType($value[$general]);
    \PHPStan\dumpType($value[$general] ?? 'newReceiverFallback');
    $index = 4;
    \PHPStan\dumpType($value[$index]);
    $index = 0;
    \PHPStan\dumpType($value[$index]);
    \PHPStan\dumpType($value[$index] ?? 'unusedNewIndexFallback');
    $array = [];
    $key = 'missing';
    \PHPStan\dumpType($array[$key]);
    $array[$key] = 'written';
    \PHPStan\dumpType($array[$key]);
    \PHPStan\dumpType($array[$key] ?? 'unusedWrittenFallback');
}

function cachedPostfixFailure(): void
{
    $value = 'foo';
    $index = 4;
    \PHPStan\dumpType($value[$index]);
    \PHPStan\dumpType($value[$index++] ?? 'postfixStringFallback');
    \PHPStan\dumpType($index);
    \PHPStan\dumpType($value[$index]);
    $array = [1];
    $index = 2;
    \PHPStan\dumpType($array[$index]);
    \PHPStan\dumpType($array[$index++] ?? 'postfixArrayFallback');
    \PHPStan\dumpType($index);
    \PHPStan\dumpType($array[$index]);
}

/** @param array<int, int|null> $value */
function cachedPostfixNarrowedValue(array $value): void
{
    $index = 0;
    if (isset($value[$index])) {
        \PHPStan\dumpType($value[$index++] ?? 'unusedPostfixFallback');
        \PHPStan\dumpType($index);
    }
}

function freshAbsentProbes(int $index): void
{
    $value = '';
    \PHPStan\dumpType(isset($value[$index]));
    \PHPStan\dumpType(empty($value[$index]));
    \PHPStan\dumpType($value[$index] ?? 'freshStringFallback');
    \PHPStan\dumpType($value[$index]);
    $value = [];
    \PHPStan\dumpType(isset($value[$index]));
    \PHPStan\dumpType(empty($value[$index]));
    \PHPStan\dumpType($value[$index] ?? 'freshArrayFallback');
    \PHPStan\dumpType($value[$index]);
}

/** @param array<string, int|string|null> $value */
function cachedValueNarrowing(array $value, string $key): void
{
    if (is_string($value[$key])) {
        \PHPStan\dumpType($value[$key] ?? false);
        \PHPStan\dumpType($value[$key]);
    }
}

function cachedWrittenStringByte(): void
{
    $value = 'foo';
    $index = 0;
    $value[$index] = 'b';
    \PHPStan\dumpType($value[$index]);
    \PHPStan\dumpType($value[$index] ?? 'unusedWrittenStringFallback');
    \PHPStan\dumpType(isset($value[$index]));
    \PHPStan\dumpType(empty($value[$index]));
}

function presentKeyWithFailedValue(): void
{
    $missing = [];
    $value = ['present' => $missing['absent']];
    $key = 'present';
    \PHPStan\dumpType($value[$key]);
    \PHPStan\dumpType($value[$key] ?? 'failedValueFallback');
    \PHPStan\dumpType($value[$key]);
}

/** @param array<string, int> $value */
function absentNullPlaceholder(array $value, string $key): void
{
    if (!isset($value[$key])) {
        \PHPStan\dumpType($value[$key] ?? 'falseFallback');
        \PHPStan\dumpType($value[$key]);
    }
}

/** @param array<string, int|null> $value */
function genuineNullCache(array $value, string $key): void
{
    if ($value[$key] === null) {
        \PHPStan\dumpType($value[$key]);
        \PHPStan\dumpType($value[$key] ?? 'nullFallback');
        \PHPStan\dumpType($value[$key]);
    }
}

function evaluatedForBound(): void
{
    $chars = './-';
    $n = strlen($chars);
    for ($i = 0; $i < $n; $i++) {
        \PHPStan\dumpType($i);
        \PHPStan\dumpType($chars[$i]);
        explode($chars[$i], 'Mercedes-Benz');
    }
}

function parameterForBound(int $n): void
{
    $chars = './-';
    for ($i = 0; $i < $n; $i++) {
        \PHPStan\dumpType($i);
        \PHPStan\dumpType($chars[$i]);
    }
}

function zeroForBound(): void
{
    $chars = './-';
    $n = strlen('');
    for ($i = 0; $i < $n; $i++) {
        return;
    }
    \PHPStan\dumpType($i);
    \PHPStan\dumpType($n);
    \PHPStan\dumpType($chars[$i]);
}

function bodyChangesForBound(): void
{
    $chars = './-';
    $n = 8;
    for ($i = 0; $i < ($n = strlen($chars)); $i++) {
        \PHPStan\dumpType($i);
        \PHPStan\dumpType($chars[$i]);
        \PHPStan\dumpType($n);
        $n = 8;
    }
}

function commaChangesForBound(): void
{
    $chars = './-';
    $n = 8;
    for ($i = 0; $n = strlen($chars), $bound = $n--, $i < $bound; $i++) {
        \PHPStan\dumpType($i);
        \PHPStan\dumpType($chars[$i]);
        \PHPStan\dumpType($bound);
    }
}

function evaluatedWhileBound(): void
{
    $chars = './-';
    $n = strlen($chars);
    $i = 0;
    while ($i < $n) {
        \PHPStan\dumpType($i);
        \PHPStan\dumpType($chars[$i]);
        $i++;
    }
}

function evaluatedDoWhileBound(): void
{
    $chars = './-';
    $n = strlen($chars);
    $i = 0;
    do {
        \PHPStan\dumpType($i);
        \PHPStan\dumpType($chars[$i]);
        $i++;
    } while ($i < $n);
}

function growingBodyBound(): void
{
    $chars = './-';
    $n = strlen($chars);
    for ($i = 0; $i < $n; $i++) {
        \PHPStan\dumpType($chars[$i]);
        $n = 4;
    }
}
