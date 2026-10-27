<?php

namespace App\Tests\Declined;

use PHPUnit\Framework\TestCase;

final class DeclinedTest extends TestCase
{
	/**
	 * @test
	 * @covers \stdClass::method
	 * @ticket 123
	 * @requires PHP 8.0
	 * @testWith [1, 2]
	 * @dataProvider provideData
	 * @depends testOther
	 * @backupGlobals enabled
	 */
	public function tabbed(int $a, int $b): void
	{
	}
}
