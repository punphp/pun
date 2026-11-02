<?php declare(strict_types = 1);

namespace CoverageOffsets;

final class Box
{
	public ?array $items = null;

	/** @return array{a: int} */
	public function shape(): array
	{
		return ['a' => 1];
	}
}

/**
 * @template T of int
 * @param T $template
 * @param int|(\Closure(): void) $intOrClosure
 * @param array{a: int, b?: string} $optional
 * @param array{a: int}|array{b: int} $shapes
 * @param list<int> $list
 * @param array<string, int> $map
 * @param int|string $key
 */
function reads(int $int, mixed $mixed, $template, $intOrClosure, array $optional, array $shapes, array $list, array $map, $key, ?Box $box, \stdClass $object): void
{
	$closure = function (): void {
	};
	echo $int['a'];
	echo $closure['a'];
	echo isset($int['a']) ? 1 : 0;
	echo isset($closure['a']) ? 1 : 0;
	echo isset($intOrClosure['a']) ? 1 : 0;
	echo isset($template['a']) ? 1 : 0;
	echo $template['a'];
	if (!is_object($mixed)) {
		echo isset($mixed['a']) ? 1 : 0;
	}
	echo isset($object['a']) ? 1 : 0;
	echo $optional['b'];
	echo $optional['c'];
	echo $shapes['a'];
	echo $shapes[$key];
	echo $list[0];
	echo $list[-1];
	echo $map['x'];
	echo $map[1];
	echo $box?->shape()['a'];
	echo $box?->shape()['z'];
	echo $box?->items['a'];
	echo $list[count($list) - 1];
	if ($list !== []) {
		echo $list[count($list) - 1];
		echo $list[array_key_first($list)];
		echo $list[array_key_last($list)];
		echo $list[array_rand($list)];
		echo $list[array_rand($list, 1)];
	}
	echo Missing::get()['a'];
	echo $map[[]];
}

function writes(int $int, int $other, \stdClass $object): void
{
	$int[] = 1;
	$other['a'] = 1;
	$object['a'] = 1;
	$undefined['a']['b'] = 1;
	$array = ['a' => 1];
	$array['b']['c'] = 1;
	$array['c'] ??= 1;
	$ref = &$array['d'];
	$unknown = new Unknown();
	$unknown[] = 1;
	$unknown['a'] = 1;
}

/**
 * @param array{a: int, b: array{c: int}} $nested
 * @param array<int, string>|null $nullable
 * @param list<array{x: int}> $rows
 * @param int|string|\stdClass|array<int> $mixedKey
 */
function destructuring(array $nested, ?array $nullable, int $int, \ArrayAccess $access, array $rows, $mixedKey): void
{
	['a' => $a, 'z' => $z] = $nested;
	['b' => ['c' => $c, 'd' => $d]] = $nested;
	[, $second, , $fourth] = [1, 2];
	[$x] = $int;
	[$y] = $nullable;
	['k' => $k] = $access;
	list($l1, list($l2, $l3)) = [1, [2]];
	foreach ($rows as ['x' => $rx, 'y' => $ry]) {
	}
	foreach ($rows as [$r0]) {
	}
	$keys = [
		new \DateTimeImmutable() => 1,
		$mixedKey => 2,
		1.5 => 3,
		null => 4,
	];
	[$mixedKey => $v] = [1];
}

/**
 * @param array{a?: int, b?: int, c?: int, d?: int, e?: int, f?: int, g?: int, h?: int, i?: int, j?: int, k?: int} $many
 */
function many(array $many): int
{
	return $many['a'] + $many['z'];
}

/**
 * @param array{first: int, second: string} $record
 */
function narrowed(array $record): void
{
	foreach (['first', 'missing'] as $column) {
		if (array_key_exists($column, $record)) {
			echo $record[$column];
		}
		echo $record[$column];
	}
}
