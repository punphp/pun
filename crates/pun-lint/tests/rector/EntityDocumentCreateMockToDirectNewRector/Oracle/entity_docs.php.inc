<?php

namespace Pun\Tests;

use PHPUnit\Framework\TestCase;
use Pun\Entity;

final class DocsTest extends TestCase
{
    public function testLineAfterDoc()
    {
        $aMock = $this->createMock(Entity\LineAfterDoc::class);
    }

    public function testBlockAfterDoc()
    {
        $aMock = $this->createMock(Entity\BlockAfterDoc::class);
    }

    public function testNotADoc()
    {
        $aMock = $this->createMock(Entity\NotADoc::class);
    }

    public function testAttributeBetween()
    {
        $aMock = $this->createMock(Entity\AttributeBetween::class);
        $aMock->method('isActive')->willReturn(true);
    }

    public function testAbstract()
    {
        $aMock = $this->createMock(Entity\AbstractEntity::class);
    }

    public function testInterface()
    {
        $aMock = $this->createMock(Entity\EntityInterface::class);
    }

    public function testTrait()
    {
        $aMock = $this->createMock(Entity\EntityTrait::class);
    }

    public function testEnum()
    {
        $aMock = $this->createMock(Entity\EntityEnum::class);
    }

    public function testLower()
    {
        $aMock = $this->createMock(Entity\LowerCase::class);
    }

    public function testPlain()
    {
        $aMock = $this->createMock(Entity\Plain::class);
    }

    public function testMagic()
    {
        $aMock = $this->createMock(Entity\WithMagic::class);
        $aMock->method('isOpen')->willReturn(true);
    }

    public function testMethodTag()
    {
        $aMock = $this->createMock(Entity\WithMethodTag::class);
        $aMock->method('isReady')->willReturn(true);
    }

    public function testChild()
    {
        $aMock = $this->createMock(Entity\Child::class);
        $aMock->method('isVisible')->willReturn(false);
        $aMock->method('getName')->willReturn('x');
        $aMock->method('getName')->willReturn(X);
        $aMock->method('get')->willReturn($aMock);
    }
}
-----
<?php

namespace Pun\Tests;

use PHPUnit\Framework\TestCase;
use Pun\Entity;

final class DocsTest extends TestCase
{
    public function testLineAfterDoc()
    {
        $a = new \Pun\Entity\LineAfterDoc();
    }

    public function testBlockAfterDoc()
    {
        $a = new \Pun\Entity\BlockAfterDoc();
    }

    public function testNotADoc()
    {
        $aMock = $this->createMock(Entity\NotADoc::class);
    }

    public function testAttributeBetween()
    {
        $a = new \Pun\Entity\AttributeBetween();
        $a->setIsActive(true);
    }

    public function testAbstract()
    {
        $aMock = $this->createMock(Entity\AbstractEntity::class);
    }

    public function testInterface()
    {
        $a = new \Pun\Entity\EntityInterface();
    }

    public function testTrait()
    {
        $a = new \Pun\Entity\EntityTrait();
    }

    public function testEnum()
    {
        $a = new \Pun\Entity\EntityEnum();
    }

    public function testLower()
    {
        $aMock = $this->createMock(Entity\LowerCase::class);
    }

    public function testPlain()
    {
        $aMock = $this->createMock(Entity\Plain::class);
    }

    public function testMagic()
    {
        $a = new \Pun\Entity\WithMagic();
        $a->isOpen(true);
    }

    public function testMethodTag()
    {
        $a = new \Pun\Entity\WithMethodTag();
        $a->setIsReady(true);
    }

    public function testChild()
    {
        $a = new \Pun\Entity\Child();
        $a->isVisible(false);
        $a->method('getName')->willReturn('x');
        $a->setName(X);
        $a->set($a);
    }
}
