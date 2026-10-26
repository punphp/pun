<?php
class SomeTest extends \PHPUnit\Framework\TestCase
{
    public function testB()
    {
        $this->personServiceMock->expects($this->exactly(3))
            ->method('prepare')
            ->withConsecutive([1], [1], [1]);
        $this->personServiceMock->expects($this->exactly(3))
            ->method('prepare')
            ->withConsecutive(
                [1],
                [1]
            )
            ->willReturnOnConsecutiveCalls(2, 2);
        $this->mock->withConsecutive([1]);
    }
}
-----
<?php
class SomeTest extends \PHPUnit\Framework\TestCase
{
    public function testB()
    {
        $this->personServiceMock->expects($this->exactly(3))
            ->method('prepare')->with([1]);
        $this->personServiceMock->expects($this->exactly(3))
            ->method('prepare')->with([1])->willReturn(2);
        $this->mock->with([1]);
    }
}
