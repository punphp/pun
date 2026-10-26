<?php

final class R1Test extends \PHPUnit\Framework\TestCase
{
    public function test()
    {
        $mock->with(
            $this->callback(function ($a) {
                $this->assertSame($a, ['x' => 1]);
                return TRUE;
            }),
            $this->callback(function ($a) use (&$b) {
                $this->assertSame(1, $a);
            }),
            $this->callback(function ($a) {
                $this->assertSame(1, $a);
                return false;
            }),
            $this->callback(function ($a, $b) {
                $this->assertSame(1, $a);
            }),
            $this->callback(function ($a) {
                $this->assertSame(1, $a['k']);
            }),
            $this->callback(function ($a) use ($c) {
                $this->ASSERTSAME((2), $a, 'msg');
            }),
            $this->callback(function ($a) use ($c) {
                $this->ASSERTSAME((2), ($a));
            }),
        );
    }
}
-----
<?php

final class R1Test extends \PHPUnit\Framework\TestCase
{
    public function test()
    {
        $mock->with(
            $this->equalTo(['x' => 1]),
            $this->callback(function ($a) use (&$b) {
                $this->assertSame(1, $a);
            }),
            $this->callback(function ($a) {
                $this->assertSame(1, $a);
                return false;
            }),
            $this->callback(function ($a, $b) {
                $this->assertSame(1, $a);
            }),
            $this->callback(function ($a) {
                $this->assertSame(1, $a['k']);
            }),
            $this->callback(function ($a) use ($c) {
                $this->ASSERTSAME((2), $a, 'msg');
            }),
            $this->equalTo(2),
        );
    }
}
