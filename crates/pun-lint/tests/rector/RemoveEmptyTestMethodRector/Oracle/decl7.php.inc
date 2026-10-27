<?php

namespace App\Tests\Decl7;

use PHPUnit\Framework\TestCase;

final class Decl7Test extends TestCase
{
    /**
     * @_1
     * @dataProvider provide
     * @depends testA
     * @covers \Foo
     * @testWith [1]
     */
    public function testDoctrineUnderscore(): void
    {
    }

    /**
     * text })
     * @dataProvider provide
     * @depends testA
     * @testWith [1]
     */
    public function testParenthesis(): void
    {
    }

    /**
     * @testWith ["__remove_array__"]
     */
    public function testRemoveArray($a): void
    {
    }

    public static function provide(): array
    {
        return [[1]];
    }

    public function testA(): int
    {
        return 1;
    }
}
-----
<?php

namespace App\Tests\Decl7;

use PHPUnit\Framework\TestCase;

final class Decl7Test extends TestCase
{
    public static function provide(): array
    {
        return [[1]];
    }

    public function testA(): int
    {
        return 1;
    }
}
