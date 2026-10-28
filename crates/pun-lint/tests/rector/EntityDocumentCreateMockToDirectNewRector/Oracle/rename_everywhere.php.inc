<?php

namespace Pun\Tests;

use PHPUnit\Framework\TestCase;
use Pun\Entity;

final class RenameTest extends TestCase
{
    public static $tableMock;

    public function testRename($tableMock = null)
    {
        $tableMock = $this->createMock(Entity\Child::class);
        $f = function () use ($tableMock) { return "a $tableMock b {$tableMock} c ${tableMock}"; };
        $g = fn() => $tableMock;
        self::$tableMock = $tableMock;
        $this->tableMock = compact('tableMock');
        $x = <<<EOT
            $tableMock
            EOT;
        $$tableMock = 1;
        $o = new class($tableMock) {
            public $tableMock;
            public function __construct(public $tableMock) {}
        };
    }

    public function testChildNoDoc()
    {
        $aMock = $this->createMock(Entity\ChildNoDoc::class);
    }

    public function testListeners()
    {
        $aMock = $this->createMock('Pun\Entity\Listeners');
    }

    public function testFromParent()
    {
        $aMock = $this->createMock(Entity\FromParent::class);
        $aMock->method('isDone')->willReturn(true);
        $aMock->method('isFresh')->willReturn(true);
        $aMock->method('isfresh')->willReturn(true);
        $aMock->method('getMockName')->willReturn(1);
    }

    public function testNoMock()
    {
        $table = $this->createMock(Entity\FromParent::class);
        $table = $this->createMock(Entity\Child::class);
        $table->method('setA');
        $table->method('setB')->willReturn($table);
    }
}
-----
<?php

namespace Pun\Tests;

use PHPUnit\Framework\TestCase;
use Pun\Entity;

final class RenameTest extends TestCase
{
    public static $tableMock;

    public function testRename($table = null)
    {
        $table = new \Pun\Entity\Child();
        $f = function () use ($table) { return "a {$table} b {$table} c {$table}"; };
        $g = fn() => $table;
        self::$tableMock = $table;
        $this->tableMock = compact('tableMock');
        $x = <<<EOT
            {$table}
            EOT;
        $$table = 1;
        $o = new class($table) {
            public $tableMock;
            public function __construct(public $table) {}
        };
    }

    public function testChildNoDoc()
    {
        $aMock = $this->createMock(Entity\ChildNoDoc::class);
    }

    public function testListeners()
    {
        $a = new \Pun\Entity\Listeners();
    }

    public function testFromParent()
    {
        $a = new \Pun\Entity\FromParent();
        $a->setIsDone(true);
        $a->setIsFresh(true);
        $a->setIsfresh(true);
        $a->setMockName(1);
    }

    public function testNoMock()
    {
        $table = new \Pun\Entity\Child();
        $table = new \Pun\Entity\Child();
    }
}
