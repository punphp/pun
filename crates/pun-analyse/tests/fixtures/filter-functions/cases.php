<?php

namespace FilterProviders;

use function PHPStan\dumpType;

/**
 * @param array<string, mixed> $map
 * @param int<17, 19> $inside
 * @param int<1, 5> $outside
 * @param array{flags?: 134217728, options?: array{default: 'bad'}} $optional
 * @param 0|134217728 $nullFlag
 * @param 0|16777216 $arrayFlag
 * @param 0|67108864 $forceFlag
 */
function cases(mixed $input, array $map, int $int, int $inside, int $outside, int $flags, int $filter, array $optional, int $nullFlag, int $arrayFlag, int $forceFlag, string $text): void
{
    dumpType(filter_input(INPUT_POST, 'row_id', FILTER_VALIDATE_INT));
    dumpType(filter_input(INPUT_GET, 'row_id', FILTER_VALIDATE_INT, FILTER_NULL_ON_FAILURE));
    dumpType(filter_input(INPUT_COOKIE, 'row_id', FILTER_VALIDATE_INT, ['options' => ['default' => 'bad']]));
    dumpType(filter_input(INPUT_SERVER, 'row_id', FILTER_VALIDATE_INT, FILTER_FORCE_ARRAY));
    dumpType(filter_input(INPUT_ENV, 'row_id', FILTER_VALIDATE_INT, FILTER_REQUIRE_ARRAY));
    dumpType(filter_input(INPUT_POST, 'row_id', FILTER_VALIDATE_INT, FILTER_REQUIRE_ARRAY | FILTER_NULL_ON_FAILURE));
    dumpType(filter_input(INPUT_POST, 'row_id', FILTER_VALIDATE_INT, ['options' => ['default' => 0, 'min_range' => 17, 'max_range' => 19]]));
    dumpType(filter_input(INPUT_POST, 17, FILTER_VALIDATE_INT));
    dumpType(filter_input(INPUT_POST, 17, FILTER_VALIDATE_INT, FILTER_NULL_ON_FAILURE));
    dumpType(filter_input(INPUT_POST, $text, FILTER_VALIDATE_INT, $nullFlag));
    dumpType(filter_input($int, 'x', FILTER_VALIDATE_INT));
    dumpType(filter_var($input, FILTER_VALIDATE_INT));
    dumpType(filter_var($input, FILTER_VALIDATE_INT, FILTER_NULL_ON_FAILURE));
    dumpType(filter_var($input, FILTER_VALIDATE_INT, $nullFlag));
    dumpType(filter_var($input, FILTER_VALIDATE_INT, $arrayFlag));
    dumpType(filter_var($input, FILTER_VALIDATE_INT, $forceFlag));
    dumpType(filter_var($input, FILTER_VALIDATE_INT, $flags));
    dumpType(filter_var($input, $filter));
    dumpType(filter_var($input, 987654321));
    dumpType(filter_var($input, FILTER_VALIDATE_INT, $optional));
    dumpType(filter_var(17, FILTER_VALIDATE_INT));
    dumpType(filter_var($int, FILTER_VALIDATE_INT));
    dumpType(filter_var(true, FILTER_VALIDATE_INT));
    dumpType(filter_var(false, FILTER_VALIDATE_INT));
    dumpType(filter_var(null, FILTER_VALIDATE_INT, FILTER_NULL_ON_FAILURE));
    dumpType(filter_var(17.0, FILTER_VALIDATE_INT));
    dumpType(filter_var(17.5, FILTER_VALIDATE_INT));
    dumpType(filter_var('17', FILTER_VALIDATE_INT));
    dumpType(filter_var('+17', FILTER_VALIDATE_INT));
    dumpType(filter_var('-17', FILTER_VALIDATE_INT));
    dumpType(filter_var('017', FILTER_VALIDATE_INT));
    dumpType(filter_var('17.0', FILTER_VALIDATE_INT));
    dumpType(filter_var(' 17', FILTER_VALIDATE_INT));
    dumpType(filter_var('0x1f', FILTER_VALIDATE_INT, FILTER_FLAG_ALLOW_HEX));
    dumpType(filter_var('0o17', FILTER_VALIDATE_INT, FILTER_FLAG_ALLOW_OCTAL));
    dumpType(filter_var('0x8000000000000000', FILTER_VALIDATE_INT, FILTER_FLAG_ALLOW_HEX));
    dumpType(filter_var('0o777777777777777777777777777777', FILTER_VALIDATE_INT, FILTER_FLAG_ALLOW_OCTAL));
    dumpType(filter_var('9223372036854775808', FILTER_VALIDATE_INT));
    dumpType(filter_var('0', FILTER_VALIDATE_INT));
    dumpType(filter_var($map, FILTER_VALIDATE_INT));
    dumpType(filter_var(new \stdClass(), FILTER_VALIDATE_INT));
    dumpType(filter_var(17, FILTER_VALIDATE_INT, FILTER_REQUIRE_ARRAY));
    dumpType(filter_var(17, FILTER_VALIDATE_INT, FILTER_REQUIRE_ARRAY | FILTER_NULL_ON_FAILURE));
    dumpType(filter_var($input, FILTER_VALIDATE_INT, FILTER_REQUIRE_ARRAY));
    dumpType(filter_var($input, FILTER_VALIDATE_INT, FILTER_FORCE_ARRAY));
    dumpType(filter_var($map, FILTER_VALIDATE_INT, FILTER_REQUIRE_ARRAY));
    dumpType(filter_var($map, FILTER_VALIDATE_INT, FILTER_FORCE_ARRAY | FILTER_NULL_ON_FAILURE));
    dumpType(filter_var(17, FILTER_VALIDATE_INT, FILTER_FORCE_ARRAY));
    dumpType(filter_var(17, FILTER_VALIDATE_INT, FILTER_FORCE_ARRAY | FILTER_REQUIRE_ARRAY));
    dumpType(filter_var(['0.0.0.0', 'bad'], FILTER_VALIDATE_IP, FILTER_REQUIRE_ARRAY));
    dumpType(filter_var($map, FILTER_VALIDATE_IP, FILTER_FORCE_ARRAY));
    dumpType(filter_var('127.0.0.1', FILTER_VALIDATE_IP));
    dumpType(filter_var('0', FILTER_VALIDATE_IP));
    dumpType(filter_var('', FILTER_VALIDATE_IP, FILTER_NULL_ON_FAILURE));
    dumpType(filter_var($input, FILTER_VALIDATE_INT, ['options' => ['min_range' => 17, 'max_range' => 19]]));
    dumpType(filter_var($int, FILTER_VALIDATE_INT, ['options' => ['min_range' => 17, 'max_range' => 19]]));
    dumpType(filter_var(18, FILTER_VALIDATE_INT, ['options' => ['min_range' => 17, 'max_range' => 19]]));
    dumpType(filter_var(9, FILTER_VALIDATE_INT, ['options' => ['min_range' => 17, 'max_range' => 19]]));
    dumpType(filter_var($inside, FILTER_VALIDATE_INT, ['options' => ['min_range' => 17, 'max_range' => 19]]));
    dumpType(filter_var($outside, FILTER_VALIDATE_INT, ['options' => ['min_range' => 17, 'max_range' => 19]]));
    dumpType(filter_var($int, FILTER_VALIDATE_INT, ['options' => ['min_range' => 19, 'max_range' => 17]]));
    dumpType(filter_var($int, FILTER_VALIDATE_INT, ['options' => ['min_range' => '17', 'max_range' => '19']]));
    dumpType(filter_var($int, FILTER_VALIDATE_INT, ['options' => ['min_range' => null, 'max_range' => null]]));
    dumpType(filter_var($int, FILTER_VALIDATE_INT, ['options' => ['min_range' => $int, 'max_range' => 19]]));
    dumpType(filter_var($int, FILTER_VALIDATE_INT, ['options' => ['min_range' => 17, 'max_range' => $int]]));
    dumpType(filter_var($int, FILTER_VALIDATE_INT, ['options' => ['min_range' => $int, 'max_range' => $int]]));
    dumpType(filter_var('bad', FILTER_VALIDATE_INT, ['options' => ['default' => 'bad']]));
    dumpType(filter_var(value: $input, options: FILTER_NULL_ON_FAILURE, filter: FILTER_VALIDATE_INT));
    dumpType(filter_input(var_name: 'x', options: FILTER_FORCE_ARRAY, type: INPUT_POST, filter: FILTER_VALIDATE_INT));
    dumpType(filter_var($input, FILTER_VALIDATE_INT, ['options' => ['default' => $input]]));
    dumpType(filter_var($input, FILTER_VALIDATE_IP, ['options' => ['default' => $input]]));
    dumpType(filter_var($input, FILTER_VALIDATE_IP, ['options' => ['default' => $text]]));
}

function invalid(): void
{
    dumpType(filter_input(999, 'x', FILTER_VALIDATE_INT));
}

/**
 * @param 0|1 $octal
 * @param 0|2 $hex
 * @param 17|'x' $union
 * @param array{options: array{min_range?: 17, default?: 'bad'}} $options
 * @param array{flags: 0, options: array<string, mixed>} $generalOptions
 * @param int<0, 3> $finite
 * @param int<17, 19> $bound
 * @param 17|19 $bounds
 */
function boundaries(mixed $input, int $octal, int $hex, int|string $union, array $options, array $generalOptions, int $finite, int $bound, int $bounds, bool $boolean): void
{
    dumpType(filter_var('17', FILTER_VALIDATE_INT, $octal));
    dumpType(filter_var('17', FILTER_VALIDATE_INT, $hex));
    dumpType(filter_var($union, FILTER_VALIDATE_INT));
    dumpType(filter_var($input, FILTER_VALIDATE_INT, $options));
    dumpType(filter_var($input, FILTER_VALIDATE_INT, $generalOptions));
    dumpType(filter_var($input, FILTER_VALIDATE_INT, $finite));
    dumpType(filter_var($input, FILTER_VALIDATE_INT, ['flags' => true]));
    dumpType(filter_var($input, FILTER_VALIDATE_INT, ['flags' => $boolean]));
    dumpType(filter_var($input, FILTER_VALIDATE_INT, ['flags' => 134217728.0]));
    dumpType(filter_var($input, FILTER_VALIDATE_INT, ['flags' => '134217728suffix']));
    dumpType(filter_var($input, FILTER_VALIDATE_INT, ['flags' => null]));
    dumpType(filter_var($input, FILTER_VALIDATE_INT, []));
    dumpType(filter_var($input, FILTER_VALIDATE_INT, [1, 2]));
    dumpType(filter_var($input, FILTER_VALIDATE_INT, ['options' => ['min_range' => $bound, 'max_range' => $bound]]));
    dumpType(filter_var($input, FILTER_VALIDATE_INT, ['options' => ['min_range' => 17, 'max_range' => 17]]));
    dumpType(filter_var($input, FILTER_VALIDATE_INT, ['options' => ['min_range' => 17.5, 'max_range' => 19.5]]));
    dumpType(filter_var($input, FILTER_VALIDATE_INT, ['options' => ['min_range' => false, 'max_range' => true]]));
    dumpType(filter_var($input, FILTER_VALIDATE_INT, ['options' => ['min_range' => $boolean]]));
    dumpType(filter_var('-9223372036854775809', FILTER_VALIDATE_INT));
    dumpType(filter_var(9223372036854775808.0, FILTER_VALIDATE_INT));
    dumpType(filter_var(-9223372036854775808.0, FILTER_VALIDATE_INT));
    dumpType(filter_input(INPUT_POST, 'x', $finite));
    dumpType(filter_input(INPUT_POST, 17, $finite));
    dumpType(('filter_var')($input, FILTER_VALIDATE_INT));
    dumpType((\filter_var(...))($input, FILTER_VALIDATE_INT));
    $callable = \filter_var(...);
    dumpType($callable($input, FILTER_VALIDATE_INT));
    dumpType(filter_var($input, 260));
    dumpType(filter_var([], 260));
    dumpType(filter_var($input, FILTER_VALIDATE_INT, ['flags' => new \stdClass()]));
    dumpType(filter_var($input, FILTER_VALIDATE_INT, ['flags' => 'none']));
    dumpType(filter_var($input, FILTER_VALIDATE_INT, ['flags' => "\v134217728\xff"]));
    dumpType(filter_var($input, FILTER_VALIDATE_INT, ['flags' => 9223372036854775808.0]));
    dumpType(filter_var($input, FILTER_VALIDATE_INT, ['flags' => -18446744073709549568.0]));
    dumpType(filter_var($input, FILTER_VALIDATE_INT, ['flags' => 18446744073709551616.0]));
    dumpType(filter_var($input, FILTER_VALIDATE_INT, ['options' => ['min_range' => '17e0', 'max_range' => '19.9']]));
    dumpType(filter_var($input, FILTER_VALIDATE_INT, ['options' => ['min_range' => $bounds, 'max_range' => $bounds]]));
}

/**
 * @template T of array<string, mixed>
 * @param T $input
 */
function templateArray(array $input): void
{
    dumpType(filter_var($input, FILTER_VALIDATE_INT, FILTER_REQUIRE_ARRAY));
}

/** @param never $input */
function bottom($input): void
{
    dumpType(filter_var($input, FILTER_VALIDATE_INT, FILTER_REQUIRE_ARRAY));
}
