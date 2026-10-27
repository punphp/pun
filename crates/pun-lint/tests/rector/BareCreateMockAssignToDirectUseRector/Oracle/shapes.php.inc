<?php

namespace App\Tests\BareAssign;

use PHPUnit\Framework\TestCase;

final class BareAssignTest extends TestCase
{
    public function testDynamicName($m): void
    {
        $mock = $this->createMock(\stdClass::class);
        $this->{$m}($mock);
    }

    public function testStaticAssert(): void
    {
        $mock = $this->createMock(\stdClass::class);
        TestCase::assertNotNull($mock);
    }

    public function testUnknownClass(): void
    {
        $mock = $this->createMock(Unknown::class);
        $this->use($mock);
    }

    public function testStatementBefore($x, $f): void
    {
        $this->other();
        $mock = $this->createMock(\stdClass::class);
        $x?->y->foo();
        self::$z->foo();
        f()->foo();
        $f()->foo();
        $x->{$f}->foo();
        self::${'z'}->foo();
        $this->use($mock);
    }
}

function anonymous()
{
    return new class extends TestCase {
        public function testIt(): void
        {
            $mock = $this->createMock(\stdClass::class);
            $this->assertNotNull($mock);
        }
    };
}
-----
<?php

namespace App\Tests\BareAssign;

use PHPUnit\Framework\TestCase;

final class BareAssignTest extends TestCase
{
    public function testDynamicName($m): void
    {
        $this->{$m}($this->createMock(\stdClass::class));
    }

    public function testStaticAssert(): void
    {
        TestCase::assertNotNull($this->createMock(\stdClass::class));
    }

    public function testUnknownClass(): void
    {
        $mock = $this->createMock(Unknown::class);
        $this->use($mock);
    }

    public function testStatementBefore($x, $f): void
    {
        $this->other();
        $x?->y->foo();
        self::$z->foo();
        f()->foo();
        $f()->foo();
        $x->{$f}->foo();
        self::${'z'}->foo();
        $this->use($this->createMock(\stdClass::class));
    }
}

function anonymous()
{
    return new class extends TestCase {
        public function testIt(): void
        {
            $mock = $this->createMock(\stdClass::class);
            $this->assertNotNull($mock);
        }
    };
}
