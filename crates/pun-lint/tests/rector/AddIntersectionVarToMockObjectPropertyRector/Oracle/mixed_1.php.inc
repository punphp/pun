<?php

namespace App\Tests;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    private MockObject $a;

    /**
     * Some text.
     */
    private \PHPUnit\Framework\MockObject\MockObject $b;

    /** @var Foo $c the c */
    private \PHPUnit\Framework\MockObject\MockObject $c;

    #[Attr]
    private \PHPUnit\Framework\MockObject\MockObject $d;

    private \PHPUnit\Framework\MockObject\MockObject $e;

    private \PHPUnit\Framework\MockObject\MockObject $f;

    /** @var array<Foo> */
    private \PHPUnit\Framework\MockObject\MockObject $g;

    protected function setUp(): void
    {
        $this->a = $this->createMock(\stdClass::class);
        $this->b = $this->createMock(Foo::class);
        $this->c = self::createMock(\stdClass::class);
        $this->d = $this->createMock(self::class);
        $this->e = $this->createMock('stdClass');
        $this->f = $this->createMock(\stdClass::FOO);
        $this->g = $this->createMock(\stdClass::class);
    }
}
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    /**
     * @var \PHPUnit\Framework\MockObject\MockObject&\stdClass
     */
    private MockObject $a;

    /**
     * Some text.
     * @var \PHPUnit\Framework\MockObject\MockObject&\App\Tests\Foo
     */
    private \PHPUnit\Framework\MockObject\MockObject $b;

    /** @var \PHPUnit\Framework\MockObject\MockObject&\stdClass $c the c */
    private \PHPUnit\Framework\MockObject\MockObject $c;

    /**
     * @var \PHPUnit\Framework\MockObject\MockObject&\self
     */
    #[Attr]
    private \PHPUnit\Framework\MockObject\MockObject $d;

    private \PHPUnit\Framework\MockObject\MockObject $e;

    /**
     * @var \PHPUnit\Framework\MockObject\MockObject&\stdClass
     */
    private \PHPUnit\Framework\MockObject\MockObject $f;

    /** @var \PHPUnit\Framework\MockObject\MockObject&\stdClass */
    private \PHPUnit\Framework\MockObject\MockObject $g;

    protected function setUp(): void
    {
        $this->a = $this->createMock(\stdClass::class);
        $this->b = $this->createMock(Foo::class);
        $this->c = self::createMock(\stdClass::class);
        $this->d = $this->createMock(self::class);
        $this->e = $this->createMock('stdClass');
        $this->f = $this->createMock(\stdClass::FOO);
        $this->g = $this->createMock(\stdClass::class);
    }
}
