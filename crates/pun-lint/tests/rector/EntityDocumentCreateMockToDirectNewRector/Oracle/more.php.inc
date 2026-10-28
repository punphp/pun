<?php

namespace Pun\Tests;

use Pun\Entity;

final class MoreTest
{
    public function twoMocks()
    {
        $aMock = $this->createMock(Entity\Child::class);
        $bMock = $this->createMock(Entity\Child::class);
        $aMock->method('setA');
        $this->$name();
        $o->$name()->with($bMock->method('setB'));
        return;
    }

    public function others()
    {
        $aMock = $this->createMock(Entity\Child::class);
        $this->aMock = $this->createMock(Entity\Child::class);
        $b = $this->foo(1);
        $c = $this->createMock($class);
        $d = $this->createMock(Entity\Missing::class);
        $e = $this->createMock(\ArrayObject::class);
        $s = "${'aMock'} ${aMock}";
        $o = new class {
            public $aMock = 1;
        };
        $aMock->method('getName')->willReturn("\xff");
    }

    public function partial()
    {
        $aMock = $this->createMock(Entity\Child::class);
        $aMock = $this->createMock(...);
    }
}
-----
<?php

namespace Pun\Tests;

use Pun\Entity;

final class MoreTest
{
    public function twoMocks()
    {
        $a = new \Pun\Entity\Child();
        $b = new \Pun\Entity\Child();
        $this->$name();
        return;
    }

    public function others()
    {
        $a = new \Pun\Entity\Child();
        $this->aMock = $this->createMock(Entity\Child::class);
        $b = $this->foo(1);
        $c = $this->createMock($class);
        $d = $this->createMock(Entity\Missing::class);
        $e = $this->createMock(\ArrayObject::class);
        $s = "${'aMock'} {$a}";
        $o = new class {
            public $aMock = 1;
        };
        $a->method('getName')->willReturn("\xff");
    }

    public function partial()
    {
        $a = new \Pun\Entity\Child();
        $a = new \Pun\Entity\Child();
    }
}
