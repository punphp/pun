<?php

namespace CountShapes;

use function PHPStan\Testing\assertType;

/**
 * @param array{0: string, 1?: int, 2?: bool} $list
 * @param array{short: string}|array{long: string, details: string} $either
 */
function narrow(array $list, array $either, string $s): void
{
	if (count($list) === 2) {
		assertType('array{0: string, 1?: int, 2?: bool}', $list);
	}
	if (count($list) >= 2) {
		assertType('array{0: string, 1?: int, 2?: bool}', $list);
	}
	if (count($either) === 2) {
		assertType('array{long: string, details: string}', $either);
	}
	if (count($either) >= 2) {
		assertType('array{long: string, details: string}', $either);
	}
	if (preg_match('/^(a)(b)?$/', $s, $m) && count($m) === 3) {
		assertType("array{non-falsy-string, 'a', 'b'}", $m);
	}
}

/** @param array{0: string, 1?: int} $pair */
function pair(array $pair): void
{
	if (count($pair) === 2) {
		assertType('array{string, int}', $pair);
	}
	if (count($pair) === 1) {
		assertType('array{string}', $pair);
	}
}
