<?php declare(strict_types = 1);

namespace CastableToNumberExtra;

class Plain {}

/**
 * @param array<int, Plain> $objects
 * @param array<int, \GMP> $gmp
 * @param array<int, \SimpleXMLElement> $xml
 * @param array<int, int|Plain> $maybeObject
 * @param array<int, mixed> $mixed
 * @param array<int, numeric-string> $numeric
 * @param array<int, non-empty-string> $nonEmpty
 * @param array<int, resource> $resources
 * @param array<int, int>|null $nullable
 * @param array<int, int>|int $notArray
 * @param list<array<int>> $nested
 * @param array<int, object> $object
 * @param array<int, \Stringable> $stringable
 * @param array<int, bool> $bools
 * @param array<int, float|null> $floats
 * @param array<int, \UnknownClass> $unknown
 * @param array<int, int> ...$variadic
 */
function check(
	array $objects,
	array $gmp,
	array $xml,
	array $maybeObject,
	array $mixed,
	array $numeric,
	array $nonEmpty,
	array $resources,
	?array $nullable,
	$notArray,
	array $nested,
	array $object,
	array $stringable,
	array $bools,
	array $floats,
	array $unknown,
	array ...$variadic,
): void {
	array_sum($objects);
	array_product($objects);
	array_sum($gmp);
	array_product($xml);
	array_sum($maybeObject);
	array_sum($mixed);
	array_sum($numeric);
	array_sum($nonEmpty);
	array_product($resources);
	array_sum($nullable);
	array_sum($notArray);
	array_sum($nested);
	array_sum($object);
	array_sum($stringable);
	array_sum($bools);
	array_sum($floats);
	array_sum($unknown);
	array_sum(...$variadic);
	array_sum($objects, $objects);
	array_sum(array: $objects);
	array_product(array: $nonEmpty);
	\array_sum(['a', 'b']);
	array_sum(['1', '2.5', 3]);
	array_sum([]);
	$f = 'array_sum';
	$r = $f($objects);
}

/**
 * @template T of object
 * @template U
 * @param array<int, T> $templates
 * @param array<int, U> $mixedTemplates
 */
function templates(array $templates, array $mixedTemplates): void
{
	array_sum($templates);
	array_sum($mixedTemplates);
}
