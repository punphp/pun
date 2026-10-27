<?php

namespace App\Tests;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    private MockObject $a;
    private MockObject $b;
    private static MockObject $c;
    private MockObject $d;
    public MockObject $e {
        get => $this->e;
    }
    private MockObject $h;

    protected function setUp(): void
    {
        $this->a = $this->createMock(\stdClass::class);
        $this->b = $this->createMock(\stdClass::class);
        $this->c = $this->createMock(\stdClass::class);
        $this->d = $this->createMock(\stdClass::class);
        $this->e = $this->createMock(\stdClass::class);
        $this->h = $this->createMock(\stdClass::class);
        $this?->a;
        $self = new self('x');
        $self->b;
        self::$c;
        SomeTest::$d;
        $this->h->foo()->bar();
    }

    public function testAnonymous(): void
    {
        $x = new class extends TestCase {
            private MockObject $a;

            protected function setUp(): void
            {
                $this->a = $this->createMock(\stdClass::class);
            }
        };
    }
}
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    private MockObject $a;
    private MockObject $b;
    private static MockObject $c;
    private MockObject $d;
    public MockObject $e {
        get => $this->e;
    }

    protected function setUp(): void
    {
        $this->a = $this->createMock(\stdClass::class);
        $this->b = $this->createMock(\stdClass::class);
        $this->c = $this->createMock(\stdClass::class);
        $this->d = $this->createMock(\stdClass::class);
        $this->e = $this->createMock(\stdClass::class);
        $this?->a;
        $self = new self('x');
        $self->b;
        self::$c;
        SomeTest::$d;
    }

    public function testAnonymous(): void
    {
        $x = new class extends TestCase {
            protected function setUp(): void
            {
            }
        };
    }
}
