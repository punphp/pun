<?php

namespace App\Tests;

use PHPUnit\Framework\Attributes\Depends;
use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    public function testA(): ?Foo
    {
    }

    public function testB(): int|string
    {
    }

    public function testC()
    {
    }

    #[Depends('testA()')]
    public function testOne($a)
    {
    }

    /**
     * @depends testB
     */
    public function testTwo(&$b)
    {
    }

    #[Depends(TESTB)]
    /**
     * @depends TESTB()
     */
    public function testThree(...$b)
    {
    }

    /** @depends testC */
    public function testFour($c)
    {
    }

    /** @depends testMissing */
    public function testFive($c)
    {
    }

    /** @depends testA */
    public function testSix(int $c)
    {
    }

    /** @depends testA */
    public function testSeven($a, $b)
    {
    }

    /** @depends testA */
    private function testEight($a)
    {
    }

    #[Depends(method: 'testA')]
    public function testNine($a)
    {
    }

    #[Depends]
    /** @depends testA */
    public function testTen($a)
    {
    }

    /** @depends */
    public function testEleven($a)
    {
    }
}
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\Attributes\Depends;
use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    public function testA(): ?Foo
    {
    }

    public function testB(): int|string
    {
    }

    public function testC()
    {
    }

    #[Depends('testA()')]
    public function testOne(?Foo $a)
    {
    }

    /**
     * @depends testB
     */
    public function testTwo(&$b)
    {
    }

    #[Depends(TESTB)]
    /**
     * @depends TESTB()
     */
    public function testThree(...$b)
    {
    }

    /** @depends testC */
    public function testFour($c)
    {
    }

    /** @depends testMissing */
    public function testFive($c)
    {
    }

    /** @depends testA */
    public function testSix(int $c)
    {
    }

    /** @depends testA */
    public function testSeven($a, $b)
    {
    }

    /** @depends testA */
    private function testEight($a)
    {
    }

    #[Depends(method: 'testA')]
    public function testNine(?Foo $a)
    {
    }

    #[Depends]
    /** @depends testA */
    public function testTen($a)
    {
    }

    /** @depends */
    public function testEleven($a)
    {
    }
}
