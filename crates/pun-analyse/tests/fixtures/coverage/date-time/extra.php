<?php

namespace DateTimeExtra;

use DateTimeImmutable as Immutable;

class DateTime
{
	public function __construct(string $s)
	{
	}
}

function f(bool $b, string $s): void
{
	// a class of the namespace, not PHP's
	new DateTime('foo');
	new Immutable('foo');
	new \datetime('2020-13-45');
	new \DATETIMEIMMUTABLE(datetime: '10:61');
	new \DateTime($b ? 'now' : 'tomorrow 25:00');
	new \DateTimeImmutable($b ? '+1 dayz' : 'next mon');
	new \DateTime('');
	new \DateTime('   ');
	new \DateTime('2021-02-30 10:00:00 Europe/Nowhere');
	new \DateTime('@1234567890 UTC');
	new \DateTime('2021-01-01 ++ 3 weeks');
	new \DateTime('last day of next month noon');
	new \DateTime('1 Jan 2020 GMT+05:30 x');
	new \DateTime('2020-01-01 01:02:03.456789 ago ago');
	new \DateTime('  @-5.5 ');
	new \DateTime('Mon, 15 Aug 2005 15:52:01 +0000 ' . "\x00");
	new \DateTime($s . 'x');
	new \DateTime();
	$class = '\DateTime';
	new $class('foo');
	new class ('foo') extends \DateTime {};
}
