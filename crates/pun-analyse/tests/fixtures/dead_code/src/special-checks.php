<?php

namespace DeadCode\SpecialChecks;

enum Suit
{
	case Hearts;
	case Spades;
}

interface SomeInterface
{
}

trait SomeTrait
{
}

/**
 * @method void magic()
 * @property int $tagged
 */
class Open
{
	public int $declared = 1;

	public function native(): void
	{
	}
}

final class Closed
{
	public static int $counter = 0;

	public function native(): void
	{
	}
}

#[\AllowDynamicProperties]
final class Dynamic
{
}

class InArray
{
	/**
	 * @param array<string> $strings
	 * @param non-empty-array<int, 'a'> $as
	 * @param non-empty-array<int, string> $nonEmptyStrings
	 * @param 'a'|'b' $ab
	 * @param array{}|array{'a'} $maybeA
	 * @param array{0: 'a', 1?: 'b'} $optional
	 * @param array<never> $nothing
	 * @param list<Suit> $suits
	 */
	public function doFoo(
		mixed $mixed,
		string $string,
		int $int,
		array $strings,
		array $as,
		array $nonEmptyStrings,
		string $ab,
		array $maybeA,
		array $optional,
		array $nothing,
		Suit $suit,
		array $suits,
	): void
	{
		in_array(1, $strings);
		in_array(1, $mixed, true);
		in_array('a', $strings, true);
		in_array($string, $as, true);
		in_array('a', $nonEmptyStrings, true);
		in_array('a', [], true);
		in_array($ab, ['a', 'c'], true);
		in_array('a', ['a'], true);
		in_array($string, ['a'], true);
		in_array('a', $maybeA, true);
		in_array('b', $optional, true);
		in_array($int, $nothing, true);
		in_array($suit, ['a']);
		in_array($suit, [Suit::Hearts, Suit::Spades]);
	}
}

class Exists
{
	/**
	 * @param class-string<Closed> $closedClass
	 * @param class-string<Open> $openClass
	 * @param object{foo: int, bar?: string} $shape
	 * @param Open|Closed $either
	 */
	public function doFoo(
		Open $open,
		Closed $closed,
		Dynamic $dynamic,
		string $string,
		string $closedClass,
		string $openClass,
		object $shape,
		object $object,
		$either,
	): void
	{
		method_exists('DeadCode\SpecialChecks\Missing', 'native');
		method_exists($open, $string);
		method_exists($open, 'native');
		method_exists($open, 'magic');
		method_exists($open, 'missing');
		method_exists($closed, 'native');
		method_exists($closed, 'missing');
		method_exists(Closed::class, 'missing');
		method_exists($openClass, 'missing');
		method_exists($object, 'native');
		method_exists(Suit::Hearts, 'cases');

		property_exists($open, $string);
		property_exists($open, '');
		property_exists($open, 'declared');
		property_exists($open, 'tagged');
		property_exists($open, 'missing');
		property_exists($closed, 'counter');
		property_exists($closed, 'missing');
		property_exists($dynamic, 'missing');
		property_exists($shape, 'foo');
		property_exists($either, 'missing');
		property_exists($string, 'missing');
		property_exists(Closed::class, 'missing');

		class_exists($string);
		class_exists(Open::class);
		class_exists(SomeInterface::class);
		class_exists(SomeTrait::class);
		class_exists(Suit::class);
		interface_exists(Open::class);
		interface_exists(SomeInterface::class);
		trait_exists(Open::class);
		trait_exists(SomeTrait::class);
		enum_exists(Open::class);
		enum_exists(Suit::class);
		class_exists('DeadCode\SpecialChecks\Missing');
	}
}

class Child extends Open
{
	public function doFoo(bool $flag, \DeadCode\SpecialChecks\Unknown $unknown, int $int): void
	{
		if ($this instanceof self) {
		}
		if ($this instanceof static) {
		}
		if ($unknown instanceof $int) {
		}
		class_exists($flag ? SomeInterface::class : SomeTrait::class);
		class_exists($flag ? SomeInterface::class : $int);
		method_exists($unknown, 'native');
	}
}
