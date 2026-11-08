<?php

final class Z2Test extends \PHPUnit\Framework\TestCase
{
    /**
     * @test
     *
     * @group fast
     */
    public function a()
    {
    }

    /**
     * Text
     * @test
     */
    public function b()
    {
    }

    /**
     * Text
     *
     * @test
     *
     */
    public function c()
    {
    }

    /**
     * @group x
     * @test
     * @group y
     */
    public function d()
    {
    }
}
-----
<?php

final class Z2Test extends \PHPUnit\Framework\TestCase
{
    /**
     *
     * @group fast
     */
    public function testA()
    {
    }

    /**
     * Text
     */
    public function testB()
    {
    }

    /**
     * Text
     *
     *
     */
    public function testC()
    {
    }

    /**
     * @group x
     * @group y
     */
    public function testD()
    {
    }
}
