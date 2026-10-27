<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    public function testB(): int|string
    {
    }

    /**
     * @depends testB
     */
    public function testTwo(&$b)
    {
    }

    /**
     * @depends testB
     */
    public function testThree($b)
    {
    }
}
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    public function testB(): int|string
    {
    }

    /**
     * @depends testB
     */
    public function testTwo(int|string &$b)
    {
    }

    /**
     * @depends testB
     */
    public function testThree(int|string $b)
    {
    }
}
