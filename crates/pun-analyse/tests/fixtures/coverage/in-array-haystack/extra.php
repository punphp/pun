<?php declare(strict_types = 1);

namespace InArrayHaystackExtra;

enum Suit
{
	case Hearts;
	case Spades;
}

enum Color
{
	case Red;
}

function maybeStrict(int $i, bool $strict): void
{
	if (in_array($i, [Suit::Hearts, 1, 2], $strict)) {
		echo 'yes';
	}
}

function looseStrings(int $i): void
{
	if (in_array($i, ['abc', 1, '2', null, true])) {
		echo 'yes';
	}
	if (in_array($i, ['abc', 'def'])) {
		echo 'yes';
	}
}

function looseEnum(Suit $s): void
{
	if (in_array($s, [Suit::Hearts, Color::Red, true, false])) {
		echo 'yes';
	}
}

function nestedArrays(int $i): void
{
	in_array([1, 2], [[1, 2], [1, 3], [1], [1, 2, 3]], true);
	in_array([1, 2], [[1, 2], [1, 3], [1], [1, 2, 3]]);
	in_array($i, [[1, 2], 1], true);
}

function duplicates(int $i): void
{
	array_search($i, [Suit::Hearts, Suit::Hearts, 'x', 'x', 1], true);
}

function notEnoughArguments(int $i): void
{
	array_keys([Suit::Hearts, 1]);
	array_keys([Suit::Hearts, 1], $i);
	in_array($i);
}

function dynamicName(int $i, string $f): void
{
	$f($i, [Suit::Hearts, 1], true);
	\in_array($i, [Suit::Hearts, 1], true);
	IN_ARRAY($i, [Suit::Hearts, 1], true);
}

function namedArguments(int $i): void
{
	in_array(haystack: [Suit::Hearts, 1], needle: $i, strict: true);
}

/**
 * @param 'a'|'b' $s
 */
function phpDocNeedle(string $s): void
{
	if (in_array($s, ['a', 'c', 1], true)) {
		echo 'yes';
	}
}

/**
 * @param list<'a'|'b'> $values
 */
function phpDocHaystack(int $i, array $values): void
{
	$haystack = $values === [] ? [1, 'x'] : [2, Suit::Hearts];
	if (in_array($i, $haystack, true)) {
		echo 'yes';
	}
}

function eliminated(mixed $m): void
{
	if ($m === 1 || $m === 'b') {
		return;
	}
	if (in_array($m, [1, 'b', 'c'], true)) {
		echo 'yes';
	}
	if (in_array($m, [1, 'b', 'c'])) {
		echo 'yes';
	}
}

function floats(float $f, ?int $i): void
{
	in_array($f, [1.5, 'x', 2.0], true);
	in_array($i, [null, 'x', 0.0, false], true);
	in_array($i, [null, 'x', 0.0, false]);
}

function nothingMatches(int $i): void
{
	in_array($i, ['a', 'b'], true);
	array_keys(['a', 'b'], $i, true);
}
