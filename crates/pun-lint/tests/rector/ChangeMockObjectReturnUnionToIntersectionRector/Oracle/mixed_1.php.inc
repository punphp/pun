<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    /**
     * @return  Foo[] | MockObject   the mock
     */
    private function a()
    {
    }

    /** @return \PHPUnit\Framework\MockObject\Stub|null */
    public function b()
    {
    }

    /** @return MockObject&Foo */
    public function c()
    {
    }

    /** @return MockObject[]|Foo */
    public function d()
    {
    }

    /**
     * @return Foo
     * @return Foo|Stub
     */
    public function e()
    {
    }

    /** @return mockobject|Foo $x */
    public function f()
    {
    }

    /** @return \Some\MockObject|Foo */
    public function g()
    {
    }
}

trait SomeTrait
{
    /** @return Foo|MockObject */
    public function testIt()
    {
    }
}
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    /**
     * @return Foo[]&MockObject the mock
     */
    private function a()
    {
    }

    /** @return \PHPUnit\Framework\MockObject\Stub&null */
    public function b()
    {
    }

    /** @return MockObject&Foo */
    public function c()
    {
    }

    /** @return MockObject[]|Foo */
    public function d()
    {
    }

    /**
     * @return Foo
     * @return Foo|Stub
     */
    public function e()
    {
    }

    /** @return mockobject|Foo $x */
    public function f()
    {
    }

    /** @return \Some\MockObject|Foo */
    public function g()
    {
    }
}

trait SomeTrait
{
    /** @return Foo&MockObject */
    public function testIt()
    {
    }
}
