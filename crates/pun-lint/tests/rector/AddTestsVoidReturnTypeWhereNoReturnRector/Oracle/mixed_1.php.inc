<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

abstract class SomeTest extends TestCase
{
    public function testA()
    {
        $f = function () { return 1; };
    }

    public function testB()
    {
        return 1;
    }

    public function testC()
    {
        yield 1;
    }

    /**
     * @test
     */
    public function itWorks()
    {
    }

    #[\PHPUnit\Framework\Attributes\Test]
    public function viaAttribute() {}

    public function helper() {}

    public function testD(): int { return 1; }

    abstract public function testE();

    public static function testF() {}

    public function Testg() {}

    public function testH()
    {
        return;
    }
}

$a = new class extends TestCase {
    public function testI() {}
};
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

abstract class SomeTest extends TestCase
{
    public function testA(): void
    {
        $f = function () { return 1; };
    }

    public function testB()
    {
        return 1;
    }

    public function testC()
    {
        yield 1;
    }

    /**
     * @test
     */
    public function itWorks(): void
    {
    }

    #[\PHPUnit\Framework\Attributes\Test]
    public function viaAttribute(): void {}

    public function helper() {}

    public function testD(): int { return 1; }

    abstract public function testE();

    public static function testF(): void {}

    public function Testg() {}

    public function testH(): void
    {
        return;
    }
}

$a = new class extends TestCase {
    public function testI(): void {}
};
