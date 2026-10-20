<?php

namespace DeadCode\SwitchLoose;

enum Suit
{
	case Hearts;
	case Spades;
}

class Holder
{
}

enum Single
{
	case Only;
}

/**
 * The duplicates of loose `==` between constants.
 */
function duplicates(mixed $m): void
{
	switch ($m) {
		case null:
			break;
		case '':
			break;
		case 0:
			break;
		case 'abc':
			break;
		case '1e1':
			break;
		case '10':
			break;
		case ' 10 ':
			break;
		case 10.0:
			break;
		case '.5':
			break;
		case 0.5:
			break;
		case '+7':
			break;
		case 7:
			break;
		case '1e':
			break;
		case '.':
			break;
		case 'abc':
			break;
		case 1.5:
			break;
		case '1.5':
			break;
		case -0.0:
			break;
	}
	switch ($m) {
		case 'x':
			break;
		case null:
			break;
		case 'x':
			break;
	}
	switch ($m) {
		case 3:
			break;
		case '3abc':
			break;
		case null:
			break;
		case false:
			break;
	}
}

/**
 * @param positive-int $positive
 * @param int<-5, -1> $negative
 * @param non-empty-string $nonEmpty
 * @param non-falsy-string $nonFalsy
 * @param lowercase-string $lower
 * @param uppercase-string $upper
 * @param non-empty-array<int> $nonEmptyArray
 * @param list<int> $list
 * @param array<string, int> $map
 * @param literal-string $literal
 * @param 1|'a' $union
 * @param array{} $empty
 * @param array{a: int} $shape
 */
function types(
	int $i,
	float $f,
	string $s,
	bool $b,
	array $a,
	Holder $o,
	Suit $suit,
	object $anyObject,
	int $positive,
	int $negative,
	string $nonEmpty,
	string $nonFalsy,
	string $lower,
	string $upper,
	array $nonEmptyArray,
	array $list,
	array $map,
	string $literal,
	int|string $union,
	array $empty,
	array $shape,
): void {
	switch ($i) {
		case [1]:
			break;
	}
	switch ($i) {
		case 'abc':
			break;
		case '12':
			break;
	}
	switch ($f) {
		case 'abc':
			break;
	}
	switch ($s) {
		case [1]:
			break;
	}
	switch ($s) {
		case 1:
			break;
	}
	switch ($a) {
		case 1:
			break;
		case 'a':
			break;
	}
	switch ($positive) {
		case true:
			break;
		case 1:
			break;
	}
	switch ($positive) {
		case false:
			break;
	}
	switch ($positive) {
		case 0:
			break;
		case 1:
			break;
	}
	switch ($negative) {
		case 3:
			break;
		case -3:
			break;
	}
	switch ($negative) {
		case 'abc':
			break;
	}
	switch ($i) {
		case $positive:
			break;
	}
	switch ($o) {
		case true:
			break;
		case 1:
			break;
	}
	switch ($o) {
		case false:
			break;
	}
	switch ($anyObject) {
		case false:
			break;
	}
	switch (true) {
		case $o:
			break;
		case $i:
			break;
	}
	switch (false) {
		case $o:
			break;
	}
	switch ($nonEmpty) {
		case null:
			break;
	}
	switch ($nonEmpty) {
		case '':
			break;
	}
	switch ($nonFalsy) {
		case false:
			break;
	}
	switch ($nonFalsy) {
		case '':
			break;
		case 'a':
			break;
	}
	switch ($lower) {
		case 'ABC':
			break;
		case '1E5':
			break;
		case 'abc':
			break;
	}
	switch ($upper) {
		case 'abc':
			break;
		case 'ABC':
			break;
	}
	switch ($literal) {
		case 'abc':
			break;
	}
	switch ($nonEmptyArray) {
		case []:
			break;
		case [1]:
			break;
	}
	switch ($list) {
		case 1:
			break;
	}
	switch ($map) {
		case 1:
			break;
	}
	switch ($union) {
		case [1]:
			break;
		case 1:
			break;
	}
	switch ($empty) {
		case 1:
			break;
	}
	switch ($empty) {
		case [1]:
			break;
	}
	switch ($empty) {
		case null:
			break;
		case false:
			break;
	}
	switch ($empty) {
		case 'a':
			break;
		case $s:
			break;
	}
	switch ($shape) {
		case 1:
			break;
		case 'a':
			break;
	}
	switch ($b) {
		case true:
			break;
		case false:
			break;
	}
	switch ($b) {
		case 'a':
			break;
		case $o:
			break;
	}
	switch ($suit) {
		case Suit::Hearts:
			break;
		case Suit::Spades:
			break;
		default:
			break;
	}
	switch ($suit) {
		case 'Hearts':
			break;
	}
	switch ($f) {
		case 1.5:
			break;
		case 2.5:
			break;
	}
}

/**
 * The cases above that left the switch narrow the subject.
 *
 * @param 'a'|'b'|'c' $s
 * @param 1|2|3 $i
 * @param array{} $empty
 * @param non-falsy-string $nonFalsy
 */
function narrowing(string $s, int $i, mixed $m, bool $b, string $string, ?string $nullable, int|string $mixedKinds, Holder $o, Single $single, array $empty, string $nonFalsy): void
{
	switch ($b) {
		case null:
			break;
		case false:
			break;
	}
	switch ($b) {
		case 1.5:
			break;
		case true:
			break;
	}
	switch ($b) {
		case $o:
			break;
		case true:
			break;
	}
	switch ($b) {
		case []:
			break;
		case true:
			break;
	}
	switch ($single) {
		case Single::Only:
			break;
	}
	switch ($i) {
		case $b ? 'a' : 'b':
			break;
	}
	switch ($empty) {
		case $b ? null : false:
			break;
		case 'x':
			break;
	}
	switch ($nonFalsy) {
		case $b ? null : '':
			break;
	}
	switch ($s) {
		case 'a':
			echo 1;
		case 'b':
			break;
		default:
		case 'c':
			break;
	}
	switch ($s) {
		case 'a':
			echo 1;
			break;
		default:
			break;
		case 'b':
			break;
		case 'c':
			break;
	}
	switch ($i) {
		case 1:
			return;
		case 2:
			break;
		case 3:
			break;
		case 4:
			break;
	}
	switch ($m) {
		case 1:
			break;
		case 2:
			break;
	}
	switch ($b) {
		case true:
			break;
		case 1:
			break;
	}
	switch ($nullable) {
		case null:
			break;
		case 'a':
			break;
	}
	switch ($string) {
		case '':
			break;
		case 'a':
			break;
	}
	switch ($mixedKinds) {
		case 0:
			break;
		case 'a':
			break;
		case 1:
			break;
	}
	switch ($mixedKinds) {
		case 1:
			break;
		case 'a':
			break;
	}
	switch ($s) {
		case unknown():
			break;
		case 'a':
			break;
	}
	switch ($s) {
		case $string:
			break;
		case 'a':
			break;
	}
	switch ('a') {
		case $s:
			break;
		case 'b':
			break;
	}
	switch (1.5) {
		case 1.5:
			break;
		case 2.5:
			break;
	}
	switch (unknown()) {
		case 1:
			break;
	}
}
