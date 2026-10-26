<?php

final class M1Test extends \PHPUnit\Framework\TestCase
{
    public function testMe()
    {
        // keep
        $this->CREATEMOCK('X');
        ($this->createMock('X'))->method('a');
        $this->createMock('X')?->method('a');
        $other->createMock('X');
        if (true) {
            $this->createMock('X');
        }
        $a = 1;
    }
}

final class NotTest
{
    public function testMe()
    {
        $this->createMock('X');
    }
}
-----
<?php

final class M1Test extends \PHPUnit\Framework\TestCase
{
    public function testMe()
    {
        $this->createMock('X')?->method('a');
        if (true) {
            $this->createMock('X');
        }
        $a = 1;
    }
}

final class NotTest
{
    public function testMe()
    {
        $this->createMock('X');
    }
}
