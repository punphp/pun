<?php

namespace IssetEmptyExtra;

class Holder
{
	public int $initialized;
	public ?int $nullable = null;
	public int $withDefault = 1;
	/** @var non-empty-array<int> */
	public array $nonEmpty = [1];

	public function __construct()
	{
		$this->initialized = 1;
		if (empty($this->initialized)) {
		}
		if (empty($this->nullable)) {
		}
	}

	public function properties(): void
	{
		if (empty($this->withDefault)) {
		}
		if (empty($this->nonEmpty)) {
		}
	}
}

final class Value
{
}

/**
 * @param non-empty-string $nonEmptyString
 * @param non-falsy-string $nonFalsyString
 * @param numeric-string $numericString
 * @param int<0, 10> $fromZero
 * @param int<1, 10> $positive
 * @param int<-5, 5> $aroundZero
 * @param array{a: int, b?: int} $shape
 * @param array{a?: int} $optionalShape
 * @param array{} $emptyShape
 * @param non-empty-array<int> $nonEmptyArray
 * @param list<int> $list
 * @param class-string $classString
 * @param resource $resource
 * @param callable $callable
 * @param iterable<int> $iterable
 * @param \Closure(): void $closure
 * @param object{a: int} $objectShape
 * @param 0|1 $zeroOrOne
 * @param 1|2 $oneOrTwo
 * @param true $true
 * @param false $false
 */
function scalars(
	int $int,
	float $float,
	string $string,
	bool $bool,
	string $nonEmptyString,
	string $nonFalsyString,
	string $numericString,
	int $fromZero,
	int $positive,
	int $aroundZero,
	array $shape,
	array $optionalShape,
	array $emptyShape,
	array $nonEmptyArray,
	array $list,
	string $classString,
	$resource,
	callable $callable,
	iterable $iterable,
	\Closure $closure,
	object $objectShape,
	object $object,
	Value $value,
	\GMP $gmp,
	\SimpleXMLElement $xml,
	int $zeroOrOne,
	int $oneOrTwo,
	bool $true,
	bool $false,
	mixed $mixed,
	?Value $maybeValue,
): void {
	$null = null;
	$zero = 0;
	$one = 1;
	$zeroFloat = 0.0;
	$half = 0.5;
	$emptyString = '';
	$zeroString = '0';
	$text = 'text';
	$emptyArray = [];
	$array = [1];

	empty($int);
	empty($float);
	empty($string);
	empty($bool);
	empty($nonEmptyString);
	empty($nonFalsyString);
	empty($numericString);
	empty($fromZero);
	empty($positive);
	empty($aroundZero);
	empty($shape);
	empty($optionalShape);
	empty($emptyShape);
	empty($nonEmptyArray);
	empty($list);
	empty($classString);
	empty($resource);
	empty($callable);
	empty($iterable);
	empty($closure);
	empty($objectShape);
	empty($object);
	empty($value);
	empty($gmp);
	empty($xml);
	empty($zeroOrOne);
	empty($oneOrTwo);
	empty($true);
	empty($false);
	empty($mixed);
	empty($maybeValue);
	empty($null);
	empty($zero);
	empty($one);
	empty($zeroFloat);
	empty($half);
	empty($emptyString);
	empty($zeroString);
	empty($text);
	empty($emptyArray);
	empty($array);
	empty($undefined);
	empty($array[0]);
	empty($array[5]);
	empty($shape['a']);
	empty($shape['c']);
	empty($value->missing);
	empty(scalarsHelper());
	empty($value?->property);

	if (rand(0, 1)) {
		$maybe = 1;
	}
	empty($maybe);
	isset($maybe);
	isset($one, $null);
	isset($undefined);
	isset($array[0], $array[1]);
}

function scalarsHelper(): int
{
	return 1;
}

/**
 * @template T
 * @template TObject of object
 * @param T $template
 * @param TObject $objectTemplate
 */
function templates($template, $objectTemplate): void
{
	rand();
	empty($template);
	empty($objectTemplate);
	isset($template);
	isset($objectTemplate);
}

function subtracted(mixed $value): void
{
	rand();
	if ($value) {
		empty($value);
	}
	if ($value !== null) {
		empty($value);
		isset($value);
	}
}

trait InTrait
{
	public function check(int $value): void
	{
		empty($value);
		isset($value);
	}
}

class UsesTrait
{
	use InTrait;
}

enum Suit
{
	case Hearts;
}

final class Narrowed
{
	public function run(array $array, object $object, \BcMath\Number $number): void
	{
		empty($this);
		empty(Suit::Hearts);
		$case = Suit::Hearts;
		empty($case);
		empty($number);
		if (isset($array['key'])) {
			empty($array);
		}
		if (!$object instanceof Value) {
			empty($object);
		}
	}
}
