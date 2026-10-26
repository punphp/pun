<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    public function test()
    {
        $m = $this->createMock('X');
        $m->method('a')->with($this->callback(function (  $args , $b = [1,2]) use ($m): bool {
           // keep me
           $item = 100;
              return $args === 1 && count($args) == 2;
        }));
        $m->method('b')->with($this->callback(static fn ($a) =>
            $a instanceof Foo
                && isset($a[1], $a['k'])
        ), 5);
        $m->method('c')->with($this->callback(function ($a) {
            return $a instanceof static;
        }));
        $m->method('d')->with($this->callback(fn ($a) => $a === $this->x || $a));
        $m->method('e')->with($this->callback(fn ($a) => count($a) === $n));
        $m->method('f')->with($this->callback(fn ($a) => isset($a)));
        $m->method('g')->with($this->callback(fn ($a) => $a->ok() && $a));
        $m->method('h')->WITH($this->CALLBACK(fn ($a) => $a == $b && $c === $a));
    }
}
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    public function test()
    {
        $m = $this->createMock('X');
        $m->method('a')->with($this->callback(function (  $args , $b = [1,2]) use ($m): void {
           // keep me
           $item = 100;
           $this->assertSame(1, $args);
           $this->assertCount(2, $args);
           return;
        }));
        $m->method('b')->with($this->callback(function ($a): void {
            $this->assertInstanceOf(\App\Tests\Foo::class, $a);
            $this->assertArrayHasKey('k', $a);
            $this->assertArrayHasKey(1, $a);
            return;
        }
        ), 5);
        $m->method('c')->with($this->callback(function ($a): void {
            $this->assertInstanceOf(static::class, $a);
            return;
        }));
        $m->method('d')->with($this->callback(fn ($a) => $a === $this->x || $a));
        $m->method('e')->with($this->callback(fn ($a) => count($a) === $n));
        $m->method('f')->with($this->callback(fn ($a) => isset($a)));
        $m->method('g')->with($this->callback(fn ($a) => $a->ok() && $a));
        $m->method('h')->WITH($this->CALLBACK(function ($a) use ($b, $c): void {
            $this->assertEquals($b, $a);
            $this->assertSame($a, $c);
            return;
        }));
    }
}
