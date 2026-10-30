<?php

namespace RefinedIssetKeys;

/** @param array<string, string> $map */
function castInteger(array $map, int $key): void {
    if (isset($map[(string) $key])) { echo 'found'; }
    echo $map[(string) $key] ?? 'missing';
}

/**
 * @param array<string, string> $map
 * @param decimal-int-string $key
 */
function decimalString(array $map, string $key): void {
    if (isset($map[$key])) { echo 'found'; }
    echo $map[$key] ?? 'missing';
}

/**
 * @param array<int, string> $map
 * @param decimal-int-string $key
 */
function decimalInteger(array $map, string $key): void {
    if (isset($map[$key])) { echo 'found'; }
    echo $map[$key] ?? 'missing';
}

/**
 * @param array<string, string> $map
 * @param non-decimal-int-string $key
 */
function nonDecimalString(array $map, string $key): void {
    if (isset($map[$key])) { echo 'found'; }
    echo $map[$key] ?? 'missing';
}

/**
 * @param array<int, string> $map
 * @param non-decimal-int-string $key
 */
function nonDecimalInteger(array $map, string $key): void {
    if (isset($map[$key])) { echo 'found'; }
    echo $map[$key] ?? 'missing';
}

/**
 * @param array<string, string> $map
 * @param numeric-string $key
 */
function numericString(array $map, string $key): void {
    if (isset($map[$key])) { echo 'found'; }
    echo $map[$key] ?? 'missing';
}

/**
 * @param array<int, string> $map
 * @param numeric-string $key
 */
function numericInteger(array $map, string $key): void {
    if (isset($map[$key])) { echo 'found'; }
    echo $map[$key] ?? 'missing';
}

/**
 * @param array<string, string> $map
 * @param non-empty-string $key
 */
function nonEmptyString(array $map, string $key): void {
    if (isset($map[$key])) { echo 'found'; }
    echo $map[$key] ?? 'missing';
}

/**
 * @param array<string, string> $map
 * @param decimal-int-string&non-empty-string $key
 */
function intersectedDecimal(array $map, string $key): void {
    if (isset($map[$key])) { echo 'found'; }
    echo $map[$key] ?? 'missing';
}

/**
 * @param array<int|string, string> $map
 * @param decimal-int-string $key
 */
function eitherKey(array $map, string $key): void {
    if (isset($map[$key])) { echo 'found'; }
    echo $map[$key] ?? 'missing';
}

/**
 * @param array{left: string} $map
 * @param decimal-int-string $key
 */
function stringShape(array $map, string $key): void {
    if (isset($map[$key])) { echo 'found'; }
    echo $map[$key] ?? 'missing';
}

/**
 * @param array{0: string} $map
 * @param decimal-int-string $key
 */
function integerShape(array $map, string $key): void {
    if (isset($map[$key])) { echo 'found'; }
    echo $map[$key] ?? 'missing';
}

/**
 * @param array<string, string> $map
 * @param decimal-int-string|'left' $key
 */
function unionKey(array $map, string $key): void {
    if (isset($map[$key])) { echo 'found'; }
    echo $map[$key] ?? 'missing';
}

/**
 * @param array<string, ?string> $map
 * @param decimal-int-string $key
 */
function nullableValue(array $map, string $key): void {
    if (isset($map[$key])) { echo 'found'; }
    echo $map[$key] ?? 'missing';
}

/** @param decimal-int-string $key */
function stringReceiver(string $text, string $key): void {
    if (isset($text[$key])) { echo 'found'; }
    echo $text[$key] ?? 'missing';
}

/** @param array<string, string> $map */
function literalKeys(array $map): void {
    if (isset($map['0'])) { echo 'found'; }
    if (isset($map['01'])) { echo 'found'; }
    if (isset($map['+1'])) { echo 'found'; }
    if (isset($map['1.0'])) { echo 'found'; }
    if (isset($map['-1'])) { echo 'found'; }
    if (isset($map['-0'])) { echo 'found'; }
    if (isset($map['9223372036854775808'])) { echo 'found'; }
    if (isset($map[''])) { echo 'found'; }
}

/**
 * @template T of decimal-int-string
 * @param array<string, string> $map
 * @param T $key
 */
function templateKey(array $map, string $key): void {
    if (isset($map[$key])) { echo 'found'; }
    echo $map[$key] ?? 'missing';
}

/**
 * @param array<string, string> $map
 * @param int|decimal-int-string $key
 */
function integerUnion(array $map, int|string $key): void {
    if (isset($map[$key])) { echo 'found'; }
    echo $map[$key] ?? 'missing';
}

/**
 * @param array<string, string> $map
 * @param '0'|'1' $key
 */
function literalUnion(array $map, string $key): void {
    if (isset($map[$key])) { echo 'found'; }
    echo $map[$key] ?? 'missing';
}

/**
 * @param array<string, string> $map
 * @param decimal-int-string|null $key
 */
function nullableUnion(array $map, ?string $key): void {
    if (isset($map[$key])) { echo 'found'; }
    echo $map[$key] ?? 'missing';
}
