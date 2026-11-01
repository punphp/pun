<?php
namespace App;
use PHPUnit\Framework\TestCase;
class Obj { public $a; public $b; public $c; public function me(): Obj { return $this; } }
final class T extends TestCase
{
    public function test(Obj $o, ?Obj $n, object $g)
    {
        $this->assertSame(['a' => 1], (array) $o);
        $this->assertSame(['a' => 'it\'s', 'b' => true, 'c' => null], (array) $o, 'message');
        $this->assertSame(['a' => "x\ny", 'b' => -5, 'c' => false], (array) $o->me());
        if ($x) $this->assertSame(['a' => 1, 'b' => 2], (array) $o);
        $this->assertSame(['a' => 1.5], (array) $o);
        $this->assertSame(['a' => [1, 2]], (array) $o);
        $this->assertSame(['a' => 1, 'a' => 2], (array) $o);
        $this->assertSame([1, 2], (array) $o);
        $this->assertSame([], (array) $o);
        $this->assertSame(['a' => 1], (array) $n);
        $this->assertSame(['a' => 1], (array) $g);
        $this->assertSame(['a' => 1], (array) $this);
        $this->assertSame(['a' => 0x10, 'b' => "d\$"], (array) $o);
        self::assertSame(['a' => 1], (array) $o);
        $this->assertSame(['a' => 1, 'b' => 2],
            (array) $o);
        $this->assertSame(['5' => 1], (array) $o);
        $this->assertSame(['a' => 1]);
    }
}
-----
<?php
namespace App;
use PHPUnit\Framework\TestCase;
class Obj { public $a; public $b; public $c; public function me(): Obj { return $this; } }
final class T extends TestCase
{
    public function test(Obj $o, ?Obj $n, object $g)
    {
        $this->assertSame(1, $o->a);
        $this->assertSame('it\'s', $o->a);
        $this->assertSame(true, $o->b);
        $this->assertSame(null, $o->c);
        $this->assertSame('x
y', $o->me()->a);
        $this->assertSame(-5, $o->me()->b);
        $this->assertSame(false, $o->me()->c);
        if ($x) {
            $this->assertSame(1, $o->a);
            $this->assertSame(2, $o->b);
        }
        $this->assertSame(1.5, $o->a);
        $this->assertSame([1, 2], $o->a);
        $this->assertSame(2, $o->a);
        $this->assertSame([1, 2], (array) $o);
        $this->assertSame([], (array) $o);
        $this->assertSame(['a' => 1], (array) $n);
        $this->assertSame(['a' => 1], (array) $g);
        $this->assertSame(['a' => 1], (array) $this);
        $this->assertSame(16, $o->a);
        $this->assertSame('d$', $o->b);
        self::assertSame(['a' => 1], (array) $o);
        $this->assertSame(1, $o->a);
        $this->assertSame(2, $o->b);
        $this->assertSame(['5' => 1], (array) $o);
        $this->assertSame(['a' => 1]);
    }
}
