<?php
namespace App;
use PHPUnit\Framework\TestCase;
class A {} class B extends A {}
final class T extends TestCase
{
    public function test(A $a, ?A $n, int|string $u)
    {
        $c3 = function ($o) { $this->assertInstanceOf(B::class, $o); };
        $c5 = function ($o): object { $this->assertInstanceOf(B::class, $o); return $o; };
        $c6 = function ($o) { $this->assertInstanceOf(B::class, $o); return 1; };
        $c7 = function ($o) use ($a) { $this->assertInstanceOf(B::class, $o); return [$o]; };
        $c9 = function ($o) { $this->assertInstanceOf(B::class, $o); throw new \Exception(); };
        $c11 = function ($o) { $f = function () use ($o) { $this->assertInstanceOf(B::class, $o); }; return 'x'; };
    }
}
-----
<?php
namespace App;
use PHPUnit\Framework\TestCase;
class A {} class B extends A {}
final class T extends TestCase
{
    public function test(A $a, ?A $n, int|string $u)
    {
        $c3 = function ($o) { $this->assertInstanceOf(B::class, $o); };
        $c5 = function ($o): object { $this->assertInstanceOf(B::class, $o); return $o; };
        $c6 = function ($o): int { $this->assertInstanceOf(B::class, $o); return 1; };
        $c7 = function ($o) use ($a): array { $this->assertInstanceOf(B::class, $o); return [$o]; };
        $c9 = function ($o) { $this->assertInstanceOf(B::class, $o); throw new \Exception(); };
        $c11 = function ($o): string { $f = function () use ($o) { $this->assertInstanceOf(B::class, $o); }; return 'x'; };
    }
}
