<?php

final class M2Test extends \PHPUnit\Framework\TestCase
{
    public function testMe()
    {
        $mock->expects($this->AT(0x3))->method('a');
        $mock->expects(($this)->at(1, 0))->method('a');
        $mock->expects($this->at('2'))->method('a');
        $mock->expects($this->at(-1))->method('a');
        $mock->expects($this->at((5)))->method('a');
        $mock->expects($this->at(0));
        $mock->foo()->expects($this->at(0))->method('a');
        $mock->expects($this->at(1_000))->method('a');
    }
}
-----
<?php

final class M2Test extends \PHPUnit\Framework\TestCase
{
    public function testMe()
    {
        $mock->expects($this->exactly(3))->method('a');
        $mock->expects($this->never())->method('a');
        $mock->expects($this->at('2'))->method('a');
        $mock->expects($this->at(-1))->method('a');
        $mock->expects($this->exactly(5))->method('a');
        $mock->expects($this->at(0));
        $mock->foo()->expects($this->never())->method('a');
        $mock->expects($this->exactly(1000))->method('a');
    }
}
