<?php
namespace FiniteIssetKeys;

/** @param array<int, string> $map */
function scalarIntegerKeys(array $map, bool $boolean, float $float): void {
    echo isset($map[$boolean]);
    echo $map[$boolean] ?? 'absent';
    echo isset($map[$float]);
    echo $map[$float] ?? 'absent';
}

/** @param array<string, string> $map */
function scalarStringKeys(array $map, bool $boolean, float $float): void {
    echo isset($map[$boolean]);
    echo $map[$boolean] ?? 'absent';
    echo isset($map[$float]);
    echo $map[$float] ?? 'absent';
}

function stringOffsets(string $text, bool $boolean, float $float): void {
    echo isset($text[$boolean]);
    echo $text[$boolean] ?? 'absent';
    echo isset($text[$float]);
    echo $text[$float] ?? 'absent';
}

/**
 * @param array{left: string, right: string, unrelated: null} $map
 * @param 'left'|'right' $key
 */
function requiredUnion(array $map, string $key): void {
    echo isset($map[$key]);
    echo $map[$key] ?? 'absent';
}

/**
 * @param array{left: string, right: null, unrelated: string} $map
 * @param 'left'|'right' $key
 */
function nullableUnion(array $map, string $key): void {
    echo isset($map[$key]);
    echo $map[$key] ?? 'absent';
}

/**
 * @param array{left: string, right?: string, unrelated: null} $map
 * @param 'left'|'right' $key
 */
function optionalUnion(array $map, string $key): void {
    echo isset($map[$key]);
    echo $map[$key] ?? 'absent';
}

/**
 * @param array{left: string, unrelated: null} $map
 * @param 'left'|'missing' $key
 */
function missingUnion(array $map, string $key): void {
    echo isset($map[$key]);
    echo $map[$key] ?? 'absent';
}

/**
 * @param array{unrelated: null} $map
 * @param 'left'|'missing' $key
 */
function absentUnion(array $map, string $key): void {
    echo isset($map[$key]);
    echo $map[$key] ?? 'absent';
}

/** @param array{0: string, 1: string, unrelated: null} $map */
function requiredBoolean(array $map, bool $key): void {
    echo isset($map[$key]);
    echo $map[$key] ?? 'absent';
}

/** @param array{0: string, 1?: string, unrelated: null} $map */
function optionalBoolean(array $map, bool $key): void {
    echo isset($map[$key]);
    echo $map[$key] ?? 'absent';
}

/** @param array{0: string, 1: null, unrelated: string} $map */
function nullableBoolean(array $map, bool $key): void {
    echo isset($map[$key]);
    echo $map[$key] ?? 'absent';
}

/**
 * @param array{0: string, 1: string, unrelated: null} $map
 * @param int<0, 1> $key
 */
function requiredRange(array $map, int $key): void {
    echo isset($map[$key]);
    echo $map[$key] ?? 'absent';
}

/**
 * @param array{0: string, 1?: string, unrelated: null} $map
 * @param int<0, 1> $key
 */
function optionalRange(array $map, int $key): void {
    echo isset($map[$key]);
    echo $map[$key] ?? 'absent';
}

/**
 * @param array{0: string, 1: null, unrelated: string} $map
 * @param int<0, 1> $key
 */
function nullableRange(array $map, int $key): void {
    echo isset($map[$key]);
    echo $map[$key] ?? 'absent';
}

/**
 * @param array{0: string, unrelated: null} $map
 * @param int<0, 1> $key
 */
function missingRange(array $map, int $key): void {
    echo isset($map[$key]);
    echo $map[$key] ?? 'absent';
}

/**
 * @param array{2: string, unrelated: null} $map
 * @param int<0, 1> $key
 */
function absentRange(array $map, int $key): void {
    echo isset($map[$key]);
    echo $map[$key] ?? 'absent';
}

/**
 * @param array{0: string, 1: string, 2: string, 3: string, 4: string, 5: string, 6: string, 7: string, 8: string, 9: string,
 * 10: string, 11: string, 12: string, 13: string, 14: string, 15: string, 16: string, 17: string, 18: string, 19: string,
 * 20: string, 21: string, 22: string, 23: string, 24: string, 25: string, 26: string, 27: string, 28: string, 29: string,
 * 30: string, 31: string, 32: string, 33: string, 34: string, 35: string, 36: string, 37: string, 38: string, 39: string,
 * 40: string, 41: string, 42: string, 43: string, 44: string, 45: string, 46: string, 47: string, 48: string, 49: string,
 * 50: string, 51: string, 52: string, 53: string, 54: string, 55: string, 56: string, 57: string, 58: string, 59: string,
 * 60: string, 61: string, 62: string, 63: string, 64: string, 65: string, 66: string, 67: string, 68: string, 69: string,
 * 70: string, 71: string, 72: string, 73: string, 74: string, 75: string, 76: string, 77: string, 78: string, 79: string,
 * 80: string, 81: string, 82: string, 83: string, 84: string, 85: string, 86: string, 87: string, 88: string, 89: string,
 * 90: string, 91: string, 92: string, 93: string, 94: string, 95: string, 96: string, 97: string, 98: string, 99: string,
 * 100: string, 101: string, 102: string, 103: string, 104: string, 105: string, 106: string, 107: string, 108: string, 109: string,
 * 110: string, 111: string, 112: string, 113: string, 114: string, 115: string, 116: string, 117: string, 118: string, 119: string,
 * 120: string, 121: string, 122: string, 123: string, 124: string, 125: string, 126: string, 127: string, 128: string, unrelated: null} $map
 * @param int<0, 128> $inside
 * @param int<0, 129> $outside
 * @param int<0, max> $unbounded
 */
function expansionBoundary(array $map, int $inside, int $outside, int $unbounded): void {
    echo isset($map[$inside]);
    echo $map[$inside] ?? 'absent';
    echo isset($map[$outside]);
    echo $map[$outside] ?? 'absent';
    echo isset($map[$unbounded]);
    echo $map[$unbounded] ?? 'absent';
}

/**
 * @param array{left: null, right: null, unrelated: string} $map
 * @param 'left'|'right' $key
 */
function nullUnion(array $map, string $key): void {
    echo isset($map[$key]);
    echo $map[$key] ?? 'absent';
}

/**
 * @param array{left: string, right: string, ...<string, null>} $map
 * @param 'left'|'right' $key
 */
function requiredUnsealed(array $map, string $key): void {
    echo isset($map[$key]);
    echo $map[$key] ?? 'absent';
}

/**
 * @param array{left: string, ...<string, null>} $map
 * @param 'left'|'extra' $key
 */
function uncertainUnsealed(array $map, string $key): void {
    echo isset($map[$key]);
    echo $map[$key] ?? 'absent';
}

/**
 * @param array{9223372036854775806: string, 9223372036854775807: string, unrelated: null} $map
 * @param int<9223372036854775806, 9223372036854775807> $key
 */
function maximumRange(array $map, int $key): void {
    echo isset($map[$key]);
    echo $map[$key] ?? 'absent';
}

/**
 * @param array{left: string, ...<string, string>} $map
 * @param 'left'|'extra' $key
 */
function specifiedUnsealed(array $map, string $key): void {
    if (array_key_exists($key, $map)) {
        echo isset($map[$key]);
        echo $map[$key] ?? 'absent';
    }
}

/**
 * @param array{left: string, ...<string, null>} $map
 * @param 'left'|'extra' $key
 */
function specifiedNullableUnsealed(array $map, string $key): void {
    if (array_key_exists($key, $map)) {
        echo isset($map[$key]);
        echo $map[$key] ?? 'absent';
    }
}
