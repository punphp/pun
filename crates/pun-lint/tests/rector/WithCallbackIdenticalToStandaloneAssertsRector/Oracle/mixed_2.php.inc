<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    public function test()
    {
        $m = $this->createMock('X');
        $m->method('a')->with($this->callback(function (  $args ,$b) use ( $m ) {
              // lost?
              return $args === 1;
        }));
        $m->method('b')->with($this->callback(
            function ($a)  :  bool   {   return $a === 2; }
        ));
        $m->method('c')->with($this->callback(function ($a) {
            $x = 1;
            return $a === $x;
            $y = 2;
        }), $this->callback(fn (int $a, /* c */ $b) => $a === $b));
        $m->method('d')->with($this->callback(fn & ($a) => $a === $b), $this->callback(fn ($a) => $a === [
            1,
        ]));
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
        $m->method('a')->with($this->callback(function ($args, $b) use ($m): void {
            $this->assertSame(1, $args);
            return;
        }));
        $m->method('b')->with($this->callback(
            function ($a): void {
                $this->assertSame(2, $a);
                return;
            }
        ));
        $m->method('c')->with($this->callback(function ($a): void {
            $x = 1;
            $y = 2;
            $this->assertSame($x, $a);
            return;
        }), $this->callback(function (
            int $a,
            /* c */
            $b
        ): void {
            $this->assertSame($b, $a);
            return;
        }));
        $m->method('d')->with($this->callback(function ($a) use ($b): void {
            $this->assertSame($b, $a);
            return;
        }), $this->callback(function ($a): void {
            $this->assertSame([
                1,
            ], $a);
            return;
        }));
    }
}
