<?php

namespace PrintfArgumentExtra;

class Plain {}

/**
 * @param int|string $intOrString
 * @param int|Plain $intOrObject
 * @param mixed $mixed
 * @param '%d'|'%s' $twoFormats
 * @param \UnknownClass $unknown
 * @param array<int> $values
 * @param resource $resource
 */
function check(string $format, $intOrString, $intOrObject, $mixed, string $twoFormats, $unknown, array $values, $resource, ?int $nullableInt, float $float): void
{
	printf($format, 'a');
	printf($twoFormats, 'a');
	printf('%d', $intOrString);
	printf('%d', $intOrObject);
	printf('%d', $mixed);
	printf('%f', $mixed);
	printf('%s', $mixed);
	printf('%d', $unknown);
	printf('%d', ...$values);
	printf('%d %s', 'x', 'y');
	printf('%5%', 'x');
	printf('%u %c %o %x %X %b', 'a', 'b', 'c', 'd', 'e', 'f');
	printf('%e %E %F %g %G', 'a', 'b', 'c', 'd', 'e');
	printf('%h %H', 'a', 'b');
	printf('%d', $resource);
	printf('%f', $resource);
	printf('%d', $nullableInt);
	printf('%f', $nullableInt);
	printf('%d', $float);
	printf('%s', $float);
	printf('%*.*f', 'a', 'b', 'c');
	printf('%1$*2$d', 'a', 'b');
	printf('%-10s|%05d|%+.2f', 'a', 'b', 'c');
	printf('%ld', 'a');
	printf('%y', 'a');
	printf(format: '%d', values: 'a');
	echo sprintf('%d');
	fprintf(STDOUT, '%d', 'a');
	fprintf(STDOUT);
	\sprintf('%d', 'a');
	$f = 'printf';
	$r = $f('%d', 'a');
	vprintf('%d', ['a']);
	printf("%d\x80", 'a');
	printf('%d', 1, 'unused');
}
