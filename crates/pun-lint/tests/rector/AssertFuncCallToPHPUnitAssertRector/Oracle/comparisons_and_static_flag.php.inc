<?php

namespace App\Tests;

use App\Thing;

final class S1Test extends \PHPUnit\Framework\TestCase
{
    public function testOne($a, $b)
    {
        assert($a == $b);
        \ASSERT(($a === 1), 'same');
        assert($a !== NULL);
        assert($a !== 1);
        assert($a != $b);
        assert((boolean) $a);
        assert(\method_exists($a, 'x'));
        assert(is_int($a));
        assert($a instanceof Thing);
        assert($a instanceof self);
        $f = static fn () => assert($a instanceof Thing);
        assert($a == 2);
        $g = static function () {};
        assert($a == 3);
    }

    public function testTwo($a)
    {
        $f = function () use ($a) {
            assert($a == 1);
        };
        Other\assert($a == 1);
    }
}
-----
<?php

namespace App\Tests;

use App\Thing;

final class S1Test extends \PHPUnit\Framework\TestCase
{
    public function testOne($a, $b)
    {
        $this->assertEquals($b, $a);
        $this->assertSame(1, $a, 'same');
        $this->assertNotNull($a);
        assert($a !== 1);
        assert($a != $b);
        $this->assertTrue((boolean) $a);
        $this->assertTrue(\method_exists($a, 'x'));
        assert(is_int($a));
        $this->assertInstanceOf(Thing::class, $a);
        assert($a instanceof self);
        $f = static fn () => $this->assertInstanceOf(Thing::class, $a);
        $this->assertEquals(2, $a);
        $g = static function () {};
        \PHPUnit\Framework\Assert::assertEquals(3, $a);
    }

    public function testTwo($a)
    {
        $f = function () use ($a) {
            $this->assertEquals(1, $a);
        };
        Other\assert($a == 1);
    }
}
