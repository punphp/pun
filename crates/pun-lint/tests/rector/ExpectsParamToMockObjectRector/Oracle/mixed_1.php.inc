<?php

namespace App\Tests;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    /**
     * @param Foo $b
     */
    private function a($a, $b, \Countable $c, ?MockObject $d, #[Attr] &$e, ...$f)
    {
        $a->expects($this->once());
        $b->expects($this->once());
        $c->expects($this->once());
        $d->expects($this->once());
        $e->expects($this->once());
        $f->expects($this->once());
    }

    private function closure($a, $b)
    {
        $fn = function () use ($a) {
            $a->expects($this->once());
        };
        $arrow = fn () => $b->expects($this->once());
    }

    public function notPrivate($a)
    {
        $a->expects($this->once());
    }

    private function noParams()
    {
    }

    private function other($a, Foo|MockObject $b)
    {
        $a->method('x');
        $A->expects($this->once());
        $b->expects($this->once());
    }
}

final class NotATest
{
    private function a($a)
    {
        $a->expects($this->once());
    }
}
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    /**
     * @param Foo $b
     */
    private function a(\PHPUnit\Framework\MockObject\MockObject $a, $b, \PHPUnit\Framework\MockObject\MockObject $c, ?MockObject $d, #[Attr] \PHPUnit\Framework\MockObject\MockObject &$e, \PHPUnit\Framework\MockObject\MockObject ...$f)
    {
        $a->expects($this->once());
        $b->expects($this->once());
        $c->expects($this->once());
        $d->expects($this->once());
        $e->expects($this->once());
        $f->expects($this->once());
    }

    private function closure($a, \PHPUnit\Framework\MockObject\MockObject $b)
    {
        $fn = function () use ($a) {
            $a->expects($this->once());
        };
        $arrow = fn () => $b->expects($this->once());
    }

    public function notPrivate($a)
    {
        $a->expects($this->once());
    }

    private function noParams()
    {
    }

    private function other(\PHPUnit\Framework\MockObject\MockObject $a, Foo|MockObject $b)
    {
        $a->method('x');
        $A->expects($this->once());
        $b->expects($this->once());
    }
}

final class NotATest
{
    private function a($a)
    {
        $a->expects($this->once());
    }
}
