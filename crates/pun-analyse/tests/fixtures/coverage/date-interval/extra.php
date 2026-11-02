<?php

namespace DateIntervalExtra;

use DateInterval as Interval;

class DateInterval
{
	public function __construct(string $s)
	{
	}
}

function f(bool $b, string $s): void
{
	// a class of the namespace, not PHP's
	new DateInterval('1M');
	new Interval('1M');
	new \dateinterval('xyz');
	new \DateInterval(duration: 'R5');
	new \DateInterval($b ? 'P1D' : '2008-03-01T13:00:00Z');
	new \DateInterval($b ? 'P1DT' : 'P2W');
	new \DateInterval(' P1D ');
	new \DateInterval('R5/2008-03-01T13:00:00Z/P1Y2M10DT2H30M');
	new \DateInterval('2008-03-01T13:00:00Z/2009-05-11T15:30:00Z');
	new \DateInterval('P0001-13-03T04:05:06');
	new \DateInterval('P0001-12-03T04:05:06');
	new \DateInterval('   ');
	new \DateInterval($s . 'D');
	new \DateInterval();
	$class = '\DateInterval';
	new $class('1M');
	new class ('1M') extends \DateInterval {};
}
