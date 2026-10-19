<?php

namespace App;

use function strlen as len;

function a($f, $x, $cls)
{
    $a = \strlen('x');
    $a = 1;
    $b = len('x');
    $b = 1;
    $c = Sub\strlen('x');
    $c = 1;
    $d = $f('x');
    $d = 1;
    $e = $x?->m();
    $e = 1;
    $g = new class {
    };
    $g = 1;
    $h = new $cls();
    $h = 1;
    $i = ($x)[0];
    $i = 1;
    $j = \Exception::create();
    $j = 1;
    $k = new \PhpParser\Node\Name('x');
    $k = 1;
    $l = new \PhpParser();
    $l = 1;
    $m = foo()[0];
    $m = 1;
}

class SomeTest extends \PHPUnit\Framework\TestCase
{
    public function test($method)
    {
        $a = $this->createMock(Foo::class);
        $a = 1;
        $b = self::createStub(Foo::class);
        $b = 1;
        $c = $this->$method();
        $c = 1;
        $d = $this->other();
        $d = 1;
        $e = $other->createMock(Foo::class);
        $e = 1;
        $f = parent::createMock(Foo::class);
        $f = 1;
    }
}

class Plain
{
    public function test()
    {
        $a = $this->createMock(Foo::class);
        $a = 1;
    }
}

class Implementing implements \Countable
{
    public function test()
    {
        $a = $this->createMock(Foo::class);
        $a = 1;
    }

    public function count(): int
    {
        return 0;
    }
}

class ShortParent extends Test
{
    public function test()
    {
        $a = $this->createMock(Foo::class);
        $a = 1;
    }
}
-----
<?php

namespace App;

use function strlen as len;

function a($f, $x, $cls)
{
    $a = 1;
    $b = 1;
    $c = Sub\strlen('x');
    $c = 1;
    $d = $f('x');
    $d = 1;
    $e = $x?->m();
    $e = 1;
    $g = new class {
    };
    $g = 1;
    $h = new $cls();
    $h = 1;
    $i = 1;
    $j = 1;
    $k = 1;
    $l = new \PhpParser();
    $l = 1;
    $m = foo()[0];
    $m = 1;
}

class SomeTest extends \PHPUnit\Framework\TestCase
{
    public function test($method)
    {
        $a = 1;
        $b = 1;
        $c = $this->$method();
        $c = 1;
        $d = $this->other();
        $d = 1;
        $e = $other->createMock(Foo::class);
        $e = 1;
        $f = 1;
    }
}

class Plain
{
    public function test()
    {
        $a = $this->createMock(Foo::class);
        $a = 1;
    }
}

class Implementing implements \Countable
{
    public function test()
    {
        $a = $this->createMock(Foo::class);
        $a = 1;
    }

    public function count(): int
    {
        return 0;
    }
}

class ShortParent extends Test
{
    public function test()
    {
        $a = $this->createMock(Foo::class);
        $a = 1;
    }
}
