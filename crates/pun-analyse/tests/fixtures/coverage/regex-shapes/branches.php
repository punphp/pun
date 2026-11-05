<?php

namespace RegexShapes;

use function PHPStan\Testing\assertType;

function nesting(string $s): void
{
	if (preg_match('/^(?:a|(?:(b)c))$/', $s, $m)) {
		assertType("array{0: non-falsy-string, 1?: 'b'}", $m);
	}
	if (preg_match('/(?:x|(?:(b))?)/', $s, $m)) {
		assertType("array{0: string, 1?: 'b'}", $m);
	}
	if (preg_match('/(?:(a)|b)+/', $s, $m)) {
		assertType('array{0: non-empty-string, 1?: non-empty-string}', $m);
	}
	if (preg_match('/((?:x|(?:(y)))+)/', $s, $m)) {
		assertType('array{0: non-empty-string, 1: non-empty-string, 2?: non-empty-string}', $m);
	}
	if (preg_match('/(?:a|(?:b(c)?)|d)/', $s, $m)) {
		assertType("array{0: non-empty-string, 1?: 'c'}", $m);
	}
	if (preg_match('/(a)|(?:(b)|c)?/', $s, $m)) {
		assertType("list{0: string, 1?: ''|'a', 2?: 'b'}", $m);
	}
	if (preg_match('/(a)|(?:(?:(b)|c)d)/', $s, $m)) {
		assertType("list{0: non-empty-string, 1?: ''|'a', 2?: 'b'}", $m);
	}
	if (preg_match('/(?:x|(?:(?:(b)))*)z/', $s, $m)) {
		assertType('array{0: non-falsy-string, 1?: non-empty-string}', $m);
	}
	if (preg_match('/(a)|(b)(?:(c)|(d))/', $s, $m)) {
		assertType("list{0: non-empty-string, 1?: ''|'a', 2?: ''|'b', 3?: ''|'c', 4?: 'd'}", $m);
	}
	if (preg_match('/(?:(?:(a)))/', $s, $m)) {
		assertType("array{non-empty-string, 'a'}", $m);
	}
	if (preg_match('/(?:(?:(a))|b)/', $s, $m)) {
		assertType("array{0: non-empty-string, 1?: 'a'}", $m);
	}
	if (preg_match('/(?:x|(?:y|(z))?)/', $s, $m)) {
		assertType("array{0: string, 1?: 'z'}", $m);
	}
	if (preg_match('/(?:x|(?:(z)|y)?)(w)?/', $s, $m)) {
		assertType("list{0: string, 1?: ''|'z', 2?: 'w'}", $m);
	}
}

function delimiters(string $s): void
{
	if (preg_match('/abc', $s, $m)) {
		assertType('array<string>', $m);
	}
	if (preg_match('abc', $s, $m)) {
		assertType('array<string>', $m);
	}
}

function literals(string $s): void
{
	if (preg_match('/(x(?:a|b))/', $s, $m)) {
		assertType('array{non-falsy-string, non-falsy-string}', $m);
	}
	if (preg_match('/(x(?:a|b){2})/', $s, $m)) {
		assertType('array{non-falsy-string, non-falsy-string}', $m);
	}
	if (preg_match('/((?:a|b)(?:c|d|e|f|g|h|i){1,2})/', $s, $m)) {
		assertType('array{non-falsy-string, non-falsy-string}', $m);
	}
	if (preg_match('/((?:a|b|c|d|e|f|g|h|i|j|k){2})/', $s, $m)) {
		assertType('array{non-falsy-string, non-falsy-string}', $m);
	}
	if (preg_match('/((?:a|b)c)/', $s, $m)) {
		assertType('array{non-falsy-string, non-falsy-string}', $m);
	}
	if (preg_match('/([[:^digit:]])/', $s, $m)) {
		assertType('array{string, string}', $m);
	}
	if (preg_match('/([[:punct:]]|[[:space:]]|[[:cntrl:]]|[[:blank:]])/', $s, $m)) {
		assertType('array{non-empty-string, non-empty-string}', $m);
	}
	if (preg_match('/(a)(?(1)b|c)/', $s, $m)) {
		assertType("array{non-falsy-string, 'a'}", $m);
	}
	if (preg_match('/(?<n>a)(?(<n>)b|c)/', $s, $m)) {
		assertType("'a'", $m[1]);
	}
	if (preg_match('/(a)(?(-1)b|c)/', $s, $m)) {
		assertType("array{non-falsy-string, 'a'}", $m);
	}
	if (preg_match('/(a)(?(?=x)b|c)/', $s, $m)) {
		assertType('array<string>', $m);
	}
}

function resets(string $s): void
{
	if (preg_match('/(?|(a)|(b))/', $s, $m)) {
		assertType('array{non-empty-string, non-empty-string}', $m);
	}
	if (preg_match('/(?|(a)(c)?|(b))/', $s, $m, PREG_UNMATCHED_AS_NULL)) {
		assertType('array{non-empty-string, non-empty-string|null, non-empty-string|null, non-empty-string|null}', $m);
	}
	if (preg_match('/(?J)(?<n>a)|(?<n>b)/', $s, $m)) {
		assertType("'a'|'b'", $m['n']);
	}
}

/** @param array<string> $m */
function sub_expression(array $m): void
{
	if (preg_match('/(\d)/', $m[0], $m)) {
		assertType('array{non-empty-string, numeric-string}', $m);
	}
}

function concat(string $s, string $q): void
{
	if (preg_match('/' . ('a' . preg_quote($q)) . '(b)/', $s, $m)) {
		assertType("array{non-falsy-string, 'b'}", $m);
	}
}

function unions(string $s, bool $a, bool $b): void
{
	$p = $a ? '/a/' : '/a?/';
	if (preg_match($p, $s, $m)) {
		assertType('array{string}', $m);
	}
	$p = $a ? '/(a)/' : ($b ? '/b?/' : '/\d/');
	if (preg_match($p, $s, $m)) {
		assertType("array{0: string, 1?: 'a'}", $m);
	}
	$p = $a ? '/(a)/' : '/(b)(c)/';
	if (preg_match($p, $s, $m)) {
		assertType("array{non-empty-string, 'a'}|array{non-falsy-string, 'b', 'c'}", $m);
	}
	$p = $a ? '/(a)/' : '/(b)/';
	if (preg_match($p, $s, $m)) {
		assertType("array{non-empty-string, 'a'|'b'}", $m);
	}
	$p = $a ? '/(a)(b)/' : '/(c)/';
	if (preg_match($p, $s, $m)) {
		assertType("array{non-empty-string, 'c'}|array{non-falsy-string, 'a', 'b'}", $m);
	}
	$p = $a ? '/(a)(x)?/' : '/(\d)(y)/';
	if (preg_match($p, $s, $m)) {
		assertType("array{0: non-falsy-string, 1: 'a', 2?: 'x'}|array{non-falsy-string, numeric-string, 'y'}", $m);
	}
	$p = $a ? '/(a)/' : '/(b)?/';
	if (preg_match($p, $s, $m, PREG_OFFSET_CAPTURE)) {
		assertType("array{0: array{string, int<-1, max>}, 1?: array{'a'|'b', int<-1, max>}}", $m);
	}
	$p = $a ? '/(a)/' : '/(\d+)/';
	if (preg_match($p, $s, $m, PREG_OFFSET_CAPTURE)) {
		assertType("array{array{non-empty-string, int<-1, max>}, array{'a'|numeric-string, int<-1, max>}}", $m);
	}
	$p = $a ? '/(a)/' : '/(b)?/';
	if (preg_match_all($p, $s, $m)) {
		assertType("array{non-empty-list<string>, non-empty-list<''|'a'|'b'>}", $m);
	}
	$p = $a ? '/(a)/' : '/(\d+)(c)/';
	if (preg_match_all($p, $s, $m)) {
		assertType("array{non-empty-list<string>, non-empty-list<'a'>}|array{non-empty-list<string>, non-empty-list<numeric-string>, non-empty-list<'c'>}", $m);
	}
	$p = $a ? '/(a)/' : '/(\d+)/';
	if (preg_match_all($p, $s, $m, PREG_SET_ORDER)) {
		assertType("non-empty-list<array{string, 'a'|numeric-string}>", $m);
	}
	$p = $a ? '/(a)/' : '/(b)(c)?/';
	if (preg_match_all($p, $s, $m, PREG_SET_ORDER)) {
		assertType("non-empty-list<array{0: string, 1: 'b', 2?: 'c'}|array{string, 'a'}>", $m);
	}
	$p = $a ? '/(a)/' : '/(\d)/';
	if (preg_match_all($p, $s, $m, PREG_OFFSET_CAPTURE)) {
		assertType("array{non-empty-list<array{string, int<-1, max>}>, non-empty-list<array{'a'|numeric-string, int<-1, max>}>}", $m);
	}
	$p = $a ? '/(a)?/' : '/(b)/';
	if (preg_match($p, $s, $m, PREG_UNMATCHED_AS_NULL)) {
		assertType("array{string, 'a'|'b'|null}", $m);
	}
	$p = $a ? '/(a)?/' : '/(\d)/';
	if (preg_match($p, $s, $m, PREG_OFFSET_CAPTURE | PREG_UNMATCHED_AS_NULL)) {
		assertType("array{array{string|null, int<-1, max>}, array{'a'|numeric-string|null, int<-1, max>}}", $m);
	}
}

function branches(string $s, bool $a): void
{
	if (preg_match('/(a)|(?:(b))/', $s, $m)) {
		assertType("array{non-empty-string, '', 'b'}|array{non-empty-string, 'a'}", $m);
	}
	if (preg_match('/(a)|(?:(b))?/', $s, $m)) {
		assertType("array{0: string, 1: '', 2?: 'b'}|array{string, 'a'}", $m);
	}
	if (preg_match('/(a)|(?:(?:(b))?c)/', $s, $m)) {
		assertType("array{0: non-empty-string, 1: '', 2?: 'b'}|array{non-empty-string, 'a'}", $m);
	}
	if (preg_match('/(a)|((b))/', $s, $m)) {
		assertType("array{non-empty-string, '', 'b', 'b'}|array{non-empty-string, 'a'}", $m);
	}
	if (preg_match($s, $s, $m)) {
		assertType('array<string>', $m);
	}
	$p = $a ? '/(a)(b)/' : '/(a)(c)/';
	if (preg_match($p, $s, $m)) {
		assertType("array{non-falsy-string, 'a', 'b'|'c'}", $m);
	}
}

function more_literals(string $s): void
{
	if (preg_match('/(a)|((b)?)/', $s, $m)) {
		assertType("array{0: string, 1: '', 2: ''|'b', 3?: 'b'}|array{string, 'a'}", $m);
	}
	if (preg_match('/(\d*)/', $s, $m)) {
		assertType("array{string, ''|numeric-string}", $m);
	}
	if (preg_match('/(\d?)x/', $s, $m)) {
		assertType("array{non-falsy-string, ''|numeric-string}", $m);
	}
	if (preg_match('/(x(a|b))/', $s, $m)) {
		assertType("array{non-falsy-string, 'xa'|'xb', 'a'|'b'}", $m);
	}
	if (preg_match('/((a|b)(c|d|e|f|g|h|i){1,2})/', $s, $m)) {
		assertType('non-falsy-string', $m[1]);
	}
	if (preg_match('/((a|b|c|d|e|f|g|h|i|j|k){2})/', $s, $m)) {
		assertType('non-falsy-string', $m[1]);
	}
}

function casts(string $s): void
{
	if ((bool) preg_match('/(.+)=(.+)/', $s, $m)) {
		assertType('array{non-falsy-string, non-empty-string, non-empty-string}', $m);
	}
	$r = match (true) {
		(bool) preg_match('/(a)(b)?/', $s, $m) => $m[1],
		default => [],
	};
	assertType("'a'|array{}", $r);
}

/** @param array{0: string, 2?: string} $a */
function coalesce(array $a, ?string $s, ?int $i): void
{
	if (($a[2] ?? '') !== '') {
		assertType('array{0: string, 2: non-empty-string}', $a);
		assertType('non-empty-string', $a[2]);
	}
	if (($a[2] ?? 0) !== 0) {
		assertType('array{0: string, 2: string}', $a);
	}
	if ('x' !== ($a[2] ?? 'x')) {
		assertType('array{0: string, 2: string}', $a);
	}
	if (($a[2] ?? 'y') !== 'x') {
		assertType('array{0: string, 2?: string}', $a);
	}
	if (($s ?? '') !== '') {
		assertType('non-empty-string|null', $s);
	} else {
		assertType('string|null', $s);
	}
	if (($i ?? 0) !== 0) {
		assertType('int<min, -1>|int<1, max>|null', $i);
	} else {
		assertType('int|null', $i);
	}
	if ('' === ($s ?? '')) {
		assertType('string|null', $s);
	} else {
		assertType('non-empty-string|null', $s);
	}
}

function step(string $part): void
{
	if (preg_match('/^(\*|\d+(?:-\d+)?)(?:\/(\w+))?$/', $part, $m) && ($m[2] ?? '') !== '') {
		assertType('non-empty-string', $m[2]);
		assertType('array{non-empty-string, non-empty-string, non-empty-string}', $m);
	}
}

/**
 * @param array{}|array{0: string, 1: string, 2?: string} $n
 * @param array{a?: string} $k
 */
function coalesce_keys(array $n, array $k): void
{
	if (($n[2] ?? '') !== '') {
		assertType('array{string, string, non-empty-string}', $n);
	}
	if (($k['a'] ?? '') !== '') {
		assertType('array{a: non-empty-string}', $k);
	}
}

function nested_resets(string $s): void
{
	if (preg_match('/(?<n>a|b)/', $s, $m)) {
		assertType("'a'|'b'", $m['n']);
	}
	if (preg_match('/((?|(a)|(b)))/', $s, $m)) {
		assertType('list{0: non-empty-string, 1: non-empty-string, 2?: string, 3?: non-empty-string}', $m);
	}
	if (preg_match('/(?J)(?<a>(?<n>x)|(?<n>y))/', $s, $m)) {
		assertType("''|'x'|'y'|'z'", $m['n'] ?? 'z');
	}
}
