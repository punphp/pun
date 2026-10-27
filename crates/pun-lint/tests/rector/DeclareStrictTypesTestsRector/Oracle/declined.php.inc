<?php

namespace App\Tests\Declined;

use PHPUnit\Framework\TestCase;

final class DeclinedTest extends TestCase
{
    // a comment before the doc
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
    public function commented(int $a, int $b): void
    {
    }

    /**
     * @test
     * @covers \stdClass::method
     * @ticket 123
     * @requires PHP 8.0
     * @testWith [1, 2]
     * @dataProvider provideData
     * @depends testOther
     * @backupGlobals enabled
     * @\Some\Annotation
     * @_internal
     */
    public function escaped(int $a, int $b): void
    {
    }

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

    public static function provideData(): iterable
    {
        yield [1, 2];
    }

    public function testOther(): int
    {
        return 1;
    }
}
-----
<?php

declare(strict_types=1);

namespace App\Tests\Declined;

use PHPUnit\Framework\TestCase;

final class DeclinedTest extends TestCase
{
    // a comment before the doc
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
    public function commented(int $a, int $b): void
    {
    }

    /**
     * @test
     * @covers \stdClass::method
     * @ticket 123
     * @requires PHP 8.0
     * @testWith [1, 2]
     * @dataProvider provideData
     * @depends testOther
     * @backupGlobals enabled
     * @\Some\Annotation
     * @_internal
     */
    public function escaped(int $a, int $b): void
    {
    }

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

    public static function provideData(): iterable
    {
        yield [1, 2];
    }

    public function testOther(): int
    {
        return 1;
    }
}
