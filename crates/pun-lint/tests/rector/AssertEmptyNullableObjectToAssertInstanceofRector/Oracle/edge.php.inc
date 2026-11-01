<?php
namespace App;
use PHPUnit\Framework\TestCase;
class Foo {}
class Bar {}
final class T extends TestCase
{
    public function test(?Foo $a, Foo|Bar|null $b, ?int $c, Foo $d)
    {
        $this->assertNotEmpty($a);
        $this->assertEmpty($a, 'msg');
        $this->AssertNull($a);
        $this->assertNotNull($b);
        $this->assertNull($c);
        $this->assertNotNull($d);
        self::assertNull($a);
        $this->assertNull(...);
        $x = $a;
        $this->assertNotEmpty($this->foo ?? $a);
    }
}
-----
<?php
namespace App;
use PHPUnit\Framework\TestCase;
class Foo {}
class Bar {}
final class T extends TestCase
{
    public function test(?Foo $a, Foo|Bar|null $b, ?int $c, Foo $d)
    {
        $this->assertInstanceOf(\App\Foo::class, $a);
        $this->assertNotInstanceOf(\App\Foo::class, $a, 'msg');
        $this->assertNotInstanceOf(\App\Foo::class, $a);
        $this->assertNotNull($b);
        $this->assertNull($c);
        $this->assertNotNull($d);
        self::assertNull($a);
        $this->assertNull(...);
        $x = $a;
        $this->assertNotEmpty($this->foo ?? $a);
    }
}
