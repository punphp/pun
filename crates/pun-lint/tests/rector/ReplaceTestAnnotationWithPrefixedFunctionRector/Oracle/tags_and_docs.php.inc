<?php

final class Z1Test extends \PHPUnit\Framework\TestCase
{
    /**
     * Checks it.
     *
     * @test
     * @group fast
     */
    public function itWorks()
    {
    }

    /** @test */
    public function oneLine()
    {
    }

    /**
     * @test
     */
    #[Foo]
    public function _under()
    {
    }

    // @test
    public function notDoc()
    {
    }
}
-----
<?php

final class Z1Test extends \PHPUnit\Framework\TestCase
{
    /**
     * Checks it.
     *
     * @group fast
     */
    public function testItWorks()
    {
    }

    /** @test */
    public function oneLine()
    {
    }

    #[Foo]
    public function test_under()
    {
    }

    // @test
    public function notDoc()
    {
    }
}
