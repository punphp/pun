<?php

namespace Pun\Tests;

use Pun\Entity;

final class NotATest
{
    public function multi()
    {
        $aMock = $this->createMock(Entity\Child::class);
        $bMock = $this->createMock(Entity\FromParent::class);
        $aMock->method('getB')->willReturn($bMock);
        $bMock->method('getA')->willReturn($aMock);
        $aMock->method('getC')->willReturn($bMock->foo);
        ($aMock->method('getD')->willReturn(...$list));
        $aMock->method(name: 'getE')->willReturn(value: [1, 2]);
        $aMock->method('getF')->willReturn(FOO)->with('x');
        $aMock->method('is')->willReturn(true);
        $aMock->method('get')->willReturn(true);
        $x = "$aMock[0] $aMock->foo {$bMock}";
        $c = $this->createMock();
        $aMock->method('isVisible')->willReturn(($aMock));
    }

    public function overwritten()
    {
        $aMock = $this->createMock(Entity\Plain::class);
        $aMock = $this->createMock(Entity\Child::class);
        if (true) {
            $aMock->method('getX')->willReturn(1);
        }
        $aMock = $this->createMock(Entity\Plain::class);
    }

    public function fromString()
    {
        $aMock = $this->createMock("Pun\\Entity\\Child");
        $aMock = $other->createMock(Entity\Listeners::class, 1);
    }

    abstract public function none();
}
-----
<?php

namespace Pun\Tests;

use Pun\Entity;

final class NotATest
{
    public function multi()
    {
        $a = new \Pun\Entity\Child();
        $b = new \Pun\Entity\FromParent();
        $a->setB($b);
        $b->setA($a);
        $a->setC($b->foo);
        ($a->setD($list));
        $a->setE([1, 2]);
        $a->setF(FOO);
        $a->is(true);
        $a->set(true);
        $x = "$a[0] $a->foo {$b}";
        $c = $this->createMock();
        $a->isVisible($a);
    }

    public function overwritten()
    {
        $aMock = $this->createMock(Entity\Plain::class);
        $aMock = $this->createMock(Entity\Child::class);
        if (true) {
            $aMock->method('getX')->willReturn(1);
        }
        $aMock = $this->createMock(Entity\Plain::class);
    }

    public function fromString()
    {
        $a = new \Pun\Entity\Listeners();
        $a = new \Pun\Entity\Listeners();
    }

    abstract public function none();
}
