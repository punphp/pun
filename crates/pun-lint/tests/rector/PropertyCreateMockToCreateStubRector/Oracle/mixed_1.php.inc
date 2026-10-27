<?php

namespace App\Tests;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    private MockObject&\Countable $a;
    private $b;
    private ?MockObject $c;
    private MockObject|\Countable $d;
    private MockObject $e, $f;
    private \Countable&MockObject&\ArrayAccess $g;
    private MockObject $h;
    private MockObject $i;
    private MockObject $j;
    private MockObject $k;
    public MockObject $l {
        get => $this->l;
    }

    protected function setUp(): void
    {
        $this->a = $this->createMock(\Countable::class);
        $this->b = $this->createMock(\Countable::class);
        $this->c = self::createMock(\Countable::class);
        $this->d = $this->createMock(\Countable::class);
        $this->e = $this->createMock(\Countable::class);
        $this->g = $this->createMock(\Countable::class);
        $this->h = $this->createMock(\Countable::class);
        $this->i = $this->createMock(\Countable::class);
        $this->j = $this->createMock(\Countable::class);
        $this->k = $this->createMock(\Countable::class);
        $this->l = $this->createMock(\Countable::class);
        $other = new \stdClass();
        $other->h->method('x');
        $fn = function () {
            $this->i->method('x');
        };
        $this?->j->method('x');
        $this->k?->method('x');
    }
}

abstract class AbstractBase extends TestCase
{
    private MockObject $a;

    protected function setUp(): void
    {
        $this->a = $this->createMock(\Countable::class);
    }
}

final class AbstractNamedTest extends TestCase
{
    private MockObject $a;

    protected function setUp(): void
    {
        $this->a = $this->createMock(\Countable::class);
    }
}

final class NoSetUpTest extends TestCase
{
    private MockObject $a;

    protected function before(): void
    {
        $this->a = $this->createMock(\Countable::class);
    }
}

final class PromotedTest extends TestCase
{
    public function __construct(private MockObject $a)
    {
    }

    protected function setUp(): void
    {
        $this->a = $this->createMock(\Countable::class);
        $this->missing = $this->createMock(\Countable::class);
    }
}

final class NotATest
{
    private MockObject $a;

    protected function setUp(): void
    {
        $this->a = $this->createMock(\Countable::class);
    }
}
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    private \PHPUnit\Framework\MockObject\Stub&\Countable $a;
    private \PHPUnit\Framework\MockObject\Stub $b;
    private \PHPUnit\Framework\MockObject\Stub $c;
    private \PHPUnit\Framework\MockObject\Stub $d;
    private \PHPUnit\Framework\MockObject\Stub $e, $f;
    private \Countable&\PHPUnit\Framework\MockObject\Stub&\ArrayAccess $g;
    private MockObject $h;
    private MockObject $i;
    private \PHPUnit\Framework\MockObject\Stub $j;
    private \PHPUnit\Framework\MockObject\Stub $k;
    public \PHPUnit\Framework\MockObject\Stub $l {
        get => $this->l;
    }

    protected function setUp(): void
    {
        $this->a = $this->createStub(\Countable::class);
        $this->b = $this->createStub(\Countable::class);
        $this->c = self::createStub(\Countable::class);
        $this->d = $this->createStub(\Countable::class);
        $this->e = $this->createStub(\Countable::class);
        $this->g = $this->createStub(\Countable::class);
        $this->h = $this->createMock(\Countable::class);
        $this->i = $this->createMock(\Countable::class);
        $this->j = $this->createStub(\Countable::class);
        $this->k = $this->createStub(\Countable::class);
        $this->l = $this->createStub(\Countable::class);
        $other = new \stdClass();
        $other->h->method('x');
        $fn = function () {
            $this->i->method('x');
        };
        $this?->j->method('x');
        $this->k?->method('x');
    }
}

abstract class AbstractBase extends TestCase
{
    private MockObject $a;

    protected function setUp(): void
    {
        $this->a = $this->createMock(\Countable::class);
    }
}

final class AbstractNamedTest extends TestCase
{
    private MockObject $a;

    protected function setUp(): void
    {
        $this->a = $this->createMock(\Countable::class);
    }
}

final class NoSetUpTest extends TestCase
{
    private MockObject $a;

    protected function before(): void
    {
        $this->a = $this->createMock(\Countable::class);
    }
}

final class PromotedTest extends TestCase
{
    public function __construct(private MockObject $a)
    {
    }

    protected function setUp(): void
    {
        $this->a = $this->createMock(\Countable::class);
        $this->missing = $this->createMock(\Countable::class);
    }
}

final class NotATest
{
    private MockObject $a;

    protected function setUp(): void
    {
        $this->a = $this->createMock(\Countable::class);
    }
}
