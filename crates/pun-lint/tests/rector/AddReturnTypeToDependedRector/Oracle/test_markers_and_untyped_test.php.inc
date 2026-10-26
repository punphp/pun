<?php

use PHPUnit\Framework\Attributes\Test;

final class V1Test extends \PHPUnit\Framework\TestCase
{
    /**
     * @test
     */
    public function first()
    {
        return true;
    }

    #[Test]
    public function second()
    {
        return rand(0, 1) ? 'a' : null;
    }

    /** @testdox Foo */
    public function third()
    {
        return 1;
    }

    public function testFour()
    {
        if (rand(0, 1)) {
            return 1;
        }
        return 2.5;
    }
}

final class V2Test extends \PHPUnit\Framework\TestCase
{
    public function testOne()
    {
        return 1;
    }

    public function testTwo()
    {
        $this->assertTrue(true);
    }
}
-----
<?php

use PHPUnit\Framework\Attributes\Test;

final class V1Test extends \PHPUnit\Framework\TestCase
{
    /**
     * @test
     */
    public function first(): true
    {
        return true;
    }

    #[Test]
    public function second(): ?string
    {
        return rand(0, 1) ? 'a' : null;
    }

    /** @testdox Foo */
    public function third()
    {
        return 1;
    }

    public function testFour()
    {
        if (rand(0, 1)) {
            return 1;
        }
        return 2.5;
    }
}

final class V2Test extends \PHPUnit\Framework\TestCase
{
    public function testOne(): int
    {
        return 1;
    }

    public function testTwo()
    {
        $this->assertTrue(true);
    }
}
