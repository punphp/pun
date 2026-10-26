<?php

final class M3Test extends \PHPUnit\Framework\TestCase
{
    public function testMe()
    {
        $a = $this->getMockBuilder('X')->disableOriginalClone()->DISABLEARGUMENTCLONING()->getMock();
        $b = $this->getMockBuilder(
            // why
            'X',
        )->getMock();
        $c = $this->getMockBuilder('X')->setMethods(['a'])->getMock();
        $d = $this->getMockBuilder(
            'X'
        )
            ->getMock();
        $e = $this->GetMockBuilder('X')->getMOCK();
    }
}

final class NotTest
{
    public function testMe()
    {
        $a = $this->getMockBuilder('X')->getMock();
    }
}
-----
<?php

final class M3Test extends \PHPUnit\Framework\TestCase
{
    public function testMe()
    {
        $a = $this->createMock('X');
        $b = $this->createMock(
            // why
            'X'
        );
        $c = $this->getMockBuilder('X')->setMethods(['a'])->getMock();
        $d = $this->createMock('X');
        $e = $this->createMock('X');
    }
}

final class NotTest
{
    public function testMe()
    {
        $a = $this->getMockBuilder('X')->getMock();
    }
}
