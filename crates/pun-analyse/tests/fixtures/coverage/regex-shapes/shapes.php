<?php

namespace RegexShapes;

use function PHPStan\Testing\assertType;

function basics(string $s, int $flags): void
{
	if (preg_match('/Price: (£|€)\d+/', $s, $m)) {
		assertType("array{non-falsy-string, '£'|'€'}", $m);
	} else {
		assertType('array{}', $m);
	}
	assertType("array{}|array{non-falsy-string, '£'|'€'}", $m);
	if (preg_match('/(a)(b)*(c)(d)*/', $s, $m)) {
		assertType("array{0: non-falsy-string, 1: 'a', 2: string, 3: 'c', 4?: non-empty-string}", $m);
	}
	if (preg_match('/(foo)(bar)(baz){0,3}(x{2,})(y{2})(z{2,3}?)/', $s, $m)) {
		assertType("array{non-falsy-string, 'foo', 'bar', string, non-falsy-string, 'yy', 'zz'|'zzz'}", $m);
	}
	if (preg_match('/(foo)(bar)/', $s, $m, PREG_OFFSET_CAPTURE)) {
		assertType("array{array{non-falsy-string, int<-1, max>}, array{'foo', int<-1, max>}, array{'bar', int<-1, max>}}", $m);
	}
	if (preg_match('/(foo)(bar)?/', $s, $m, PREG_UNMATCHED_AS_NULL)) {
		assertType("array{non-falsy-string, 'foo', 'bar'|null}", $m);
	}
	if (preg_match('/(foo)/', $s, $m, $flags)) {
		assertType('array<array{string|null, int<-1, max>}|string|null>', $m);
	}
	if (preg_match('/(foo)/', $s, $m, 4096)) {
		assertType('array<array{string|null, int<-1, max>}|string|null>', $m);
	}
	if (preg_match('/(?|(a)|(b)(c))/', $s, $m)) {
		assertType('array{0: non-empty-string, 1: non-empty-string, 2?: non-empty-string}', $m);
	}
	if (preg_match('/(?<num>\d+)-(\w)/', $s, $m)) {
		assertType('numeric-string', $m['num']);
		assertType('non-empty-string', $m[2]);
	}
	if (preg_match('/(?<num>\d+)-(\w)/n', $s, $m)) {
		assertType('numeric-string', $m[1]);
	}
	if (preg_match('/(a) # a comment
		(b) \# (?#inline)/x', $s, $m)) {
		assertType("array{non-falsy-string, 'a', 'b'}", $m);
	}
	if (preg_match('/(*:A)(a)|(*:B)b/', $s, $m)) {
		assertType("array{0: non-falsy-string, 1?: 'a', MARK?: 'A'|'B'}", $m);
	}
	if (preg_match('/(a)(?=b)(?!c)(?<=d)(?<!e)(?#c)(?i)(?i:f)(?>g)/', $s, $m)) {
		assertType("array{non-falsy-string, 'a'}", $m);
	}
	if (preg_match('/(?<n>a)?(?(1)b|c)(?(<n>)x)(?(+1)y)(z)/', $s, $m)) {
		assertType("''|'a'", $m['n']);
		assertType("'z'", $m[2]);
	}
	if (preg_match('/([^a][]a][[:alpha:]\d-])([a-z-])([[:digit:]])([[:blank:][:cntrl:][:space:][:punct:][:upper:]])/', $s, $m)) {
		assertType('array{non-falsy-string, non-falsy-string, non-empty-string, numeric-string, non-empty-string}', $m);
	}
	if (preg_match('/(a)\K(b)/', $s, $m)) {
		assertType("array{string, 'a', 'b'}", $m);
	}
	if (preg_match('/(a|b)(c)/i', $s, $m)) {
		assertType('array{non-falsy-string, non-empty-string, non-empty-string}', $m);
	}
	if (preg_match('/(a)|(b)/', $s, $m)) {
		assertType("array{non-empty-string, '', 'b'}|array{non-empty-string, 'a'}", $m);
	}
	if (preg_match('/(a)|b/', $s, $m)) {
		assertType("array{0: non-empty-string, 1?: 'a'}", $m);
	}
	if (preg_match('/(a)|(b)/', $s, $m, PREG_UNMATCHED_AS_NULL)) {
		assertType("array{non-empty-string, 'a'|null, 'b'|null}", $m);
	}
	if (preg_match('/x(a)?/', $s, $m)) {
		assertType("array{0: non-falsy-string, 1?: 'a'}", $m);
	}
	if (preg_match('/(' . preg_quote($s) . ')x/', $s, $m)) {
		assertType('array{non-falsy-string, string}', $m);
	}
	if (preg_match('(a(b))i', $s, $m)) {
		assertType('array{non-falsy-string, non-empty-string}', $m);
	}
	if (preg_match('{a(b)}', $s, $m)) {
		assertType("array{non-falsy-string, 'b'}", $m);
	}
	if (preg_match('<a(b)>', $s, $m)) {
		assertType("array{non-falsy-string, 'b'}", $m);
	}
	if (preg_match('[a(b)]', $s, $m)) {
		assertType("array{non-falsy-string, 'b'}", $m);
	}
	if (preg_match('/(a)/J', $s, $m)) {
		assertType('array<string>', $m);
	}
	if (preg_match('/(a/', $s, $m)) {
		assertType('array<string>', $m);
	}
	if (preg_match('/(\0|\.|.)(\x41\x{263a}\101\cA\p{Lu}\n)/u', $s, $m)) {
		assertType('array{non-falsy-string, string, non-falsy-string}', $m);
	}
	if (preg_match('/((ab?){0,7})((ab?c?){1,3})(0|1)(\d{2})(1|)(|2)()/', $s, $m)) {
		assertType("array{non-falsy-string, string, string, 'a'|'aa'|'aaa'|'aaab'|'aaabc'|'aaac'|'aab'|'aaba'|'aabab'|'aababc'|'aabac'|'aabc'|'aabca'|'aabcab'|'aabcabc'|'aabcac'|'aac'|'aaca'|'aacab'|'aacabc'|'aacac'|'ab'|'aba'|'abaa'|'abaab'|'abaabc'|'abaac'|'abab'|'ababa'|'ababab'|'abababc'|'ababac'|'ababc'|'ababca'|'ababcab'|'ababcabc'|'ababcac'|'abac'|'abaca'|'abacab'|'abacabc'|'abacac'|'abc'|'abca'|'abcaa'|'abcaab'|'abcaabc'|'abcaac'|'abcab'|'abcaba'|'abcabab'|'abcababc'|'abcabac'|'abcabc'|'abcabca'|'abcabcab'|'abcabcabc'|'abcabcac'|'abcac'|'abcaca'|'abcacab'|'abcacabc'|'abcacac'|'ac'|'aca'|'acaa'|'acaab'|'acaabc'|'acaac'|'acab'|'acaba'|'acabab'|'acababc'|'acabac'|'acabc'|'acabca'|'acabcab'|'acabcabc'|'acabcac'|'acac'|'acaca'|'acacab'|'acacabc'|'acacac', non-falsy-string, '0'|'1', non-falsy-string&numeric-string, ''|'1', ''|'2', ''}", $m);
	}
	if (preg_match('/a(b(c|d)?)?e/', $s, $m)) {
		assertType("list{0: non-falsy-string, 1?: 'b'|'bc'|'bd', 2?: 'c'|'d'}", $m);
	}
	if (preg_match('/((a)|(b))(c)/', $s, $m)) {
		assertType("array{non-falsy-string, 'a'|'b', ''|'a', ''|'b', 'c'}", $m);
	}
	if (preg_match('/(?:(a)|b)?(c)/', $s, $m)) {
		assertType("array{non-falsy-string, ''|'a', 'c'}", $m);
	}
	if (preg_match('/(?:(a)|(b))?/', $s, $m)) {
		assertType("array{0: string, 1?: 'a'}|array{string, '', 'b'}", $m);
	}
	if (preg_match('/(?:x(a)|(b))/', $s, $m)) {
		assertType("array{non-empty-string, '', 'b'}|array{non-empty-string, 'a'}", $m);
	}
	if (preg_match('/[\b](?:[0-9]+)(\d+|0)/', $s, $m)) {
		assertType('array{non-falsy-string, numeric-string}', $m);
	}
}

function subjects(string $s, string $t, int $i): void
{
	if (preg_match('/a+/', $s)) {
		assertType('non-empty-string', $s);
	}
	if (preg_match('/^(a)b$/', $t, $m)) {
		assertType('non-falsy-string', $t);
	}
	if (preg_match('/a?/', $s)) {
		assertType('string', $s);
	}
	if (preg_match('/a\Kb/', $s)) {
		assertType('string', $s);
	}
	if (preg_match('/a/', $i)) {
		assertType('int', $i);
	}
}

function comparisons(string $s): void
{
	if (preg_match('/(a)/', $s, $m) === 1) {
		assertType("array{non-empty-string, 'a'}", $m);
	} else {
		assertType("array{}|array{non-empty-string, 'a'}", $m);
	}
	if (preg_match('/(a)/', $s, $m) !== 1) {
		assertType("array{}|array{non-empty-string, 'a'}", $m);
	} else {
		assertType("array{non-empty-string, 'a'}", $m);
	}
	if (preg_match('/(a)/', $s, $m) > 0) {
		assertType("array{non-empty-string, 'a'}", $m);
	}
	if (preg_match('/(a)/', $s, $m) >= 1) {
		assertType("array{non-empty-string, 'a'}", $m);
	}
	if (0 < preg_match('/(a)/', $s, $m)) {
		assertType("array{non-empty-string, 'a'}", $m);
	}
	if (1 <= preg_match('/(a)/', $s, $m)) {
		assertType("array{non-empty-string, 'a'}", $m);
	}
	if (preg_match('/(a)/', $s, $m) >= 0) {
		assertType("array{}|array{non-empty-string, 'a'}", $m);
	}
	if (preg_match('/(a)/', $s, $m) == true) {
		assertType("array{non-empty-string, 'a'}", $m);
	} else {
		assertType('array{}', $m);
	}
	if (preg_match('/(a)/', $s, $m) != false) {
		assertType("array{non-empty-string, 'a'}", $m);
	}
	if (1 == preg_match('/(a)/', $s, $m)) {
		assertType("array{non-empty-string, 'a'}", $m);
	}
	if (1 != preg_match('/(a)/', $s, $m)) {
		assertType("array{}|array{non-empty-string, 'a'}", $m);
	}
}

function all(string $s): void
{
	preg_match_all('/a(b)(\d+)?/', $s, $m);
	assertType("array{list<string>, list<'b'>, list<''|numeric-string>}", $m);
	preg_match_all('/a(b)(\d+)?/', $s, $m, PREG_SET_ORDER);
	assertType("list<array{0: string, 1: 'b', 2?: numeric-string}>", $m);
	preg_match_all('/a(b)?(\d+)?/', $s, $m, PREG_SET_ORDER | PREG_OFFSET_CAPTURE);
	assertType("list<array{0: array{string, int<-1, max>}, 1?: array{'b', int<-1, max>}, 2?: array{numeric-string, int<-1, max>}}>", $m);
	preg_match_all('/a(b)(\d+)?/', $s, $m, PREG_UNMATCHED_AS_NULL);
	assertType("array{list<string>, list<'b'>, list<numeric-string|null>}", $m);
	preg_match_all('/a(b)(\d+)?/', $s, $m, PREG_UNMATCHED_AS_NULL | PREG_SET_ORDER);
	assertType("list<array{string, 'b', numeric-string|null}>", $m);
	if (preg_match_all('/a(b)(\d+)?/', $s, $m)) {
		assertType("array{non-empty-list<string>, non-empty-list<'b'>, non-empty-list<''|numeric-string>}", $m);
	} else {
		assertType('array{}', $m);
	}
	if (preg_match_all('/a(b)/', $s, $m, PREG_SET_ORDER)) {
		assertType("non-empty-list<array{string, 'b'}>", $m);
	}
	if (preg_match_all('/a(b)/', $s, $m) != false) {
		assertType("array{non-empty-list<string>, non-empty-list<'b'>}", $m);
	}
	preg_match_all('~<a\s+[^>]*href="/hashtag/([a-z0-9]+)"[^>]*>#\1</a>~i', $s, $m);
	assertType('array{list<string>, list<non-empty-string>}', $m);
}

function callbacks(string $s): void
{
	preg_replace_callback('/(foo)?(bar)/', function ($m) {
		assertType("array{non-falsy-string, ''|'foo', 'bar'}", $m);
		return '';
	}, $s);
	preg_replace_callback('/(foo)/', function (array $m): string {
		assertType("array{array{non-falsy-string, int<-1, max>}, array{'foo', int<-1, max>}}", $m);
		return '';
	}, $s, -1, $count, PREG_OFFSET_CAPTURE);
	preg_replace_callback('/(foo)/', fn ($m) => assertType("array{non-falsy-string, 'foo'}", $m), $s, flags: PREG_UNMATCHED_AS_NULL);
}

/** @param '/(a)/'|'/(b)(c)?/' $p */
function unions(string $s, string $p, string $q): void
{
	if (preg_match($p, $s, $m)) {
		assertType("array{0: non-falsy-string, 1: 'b', 2?: 'c'}|array{non-empty-string, 'a'}", $m);
	}
	if (preg_match($q, $s, $m)) {
		assertType('array<string>', $m);
	}
}

/** @param 256|512 $either */
function stubFallback(string $s, string $q, int $either): void
{
	preg_match($q, $s, $m, PREG_OFFSET_CAPTURE);
	assertType('array<array{string, int<-1, max>}>', $m);
	preg_match($q, $s, $m, $either);
	assertType('array<array{string, int<-1, max>}|string|null>', $m);
	preg_match_all($q, $s, $m);
	assertType('array<list<string>>', $m);
	preg_match_all($q, $s, $m, PREG_SET_ORDER);
	assertType('list<array<string>>', $m);
	preg_match_all($q, $s, $m, 4096);
	assertType('array<mixed>', $m);
	if (!preg_match_all($q, $s, $m)) {
		assertType('array{}', $m);
	}
	if (preg_match_all('/(a)/', $s, $m) !== 1) {
		return;
	}
	assertType("array{list<string>, list<'a'>}", $m);
}
