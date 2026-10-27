<?php

namespace App\Tests;

use PHPUnit\Framework\MockObject\Stub;
use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    private Stub $a;

    /** @var Foo */
    private Stub $b;

    /** @var \Some\Stub */
    private Stub $c;

    /** @var Foo|Bar */
    private Stub $d;

    /** @var Foo&Bar */
    private Stub $e;

    /** @var Foo[] */
    private Stub $f;

    private Stub $g, $h;

    /**
     * Text.
     * @var Foo $i
     */
    private Stub $i;

    private ?Stub $j;

    protected function setUp(): void
    {
        $this->a = $this->createStub(\stdClass::class);
        $this->e = $this->createStub(\stdClass::class);
        $this->g = self::createStub(static::class);
        $this->h = $this->createStub(\stdClass::class);
    }
}

final class NoSetUpTest extends TestCase
{
    /** @var Foo */
    private Stub $a;

    private Stub $b;
}
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\MockObject\Stub;
use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    /**
     * @var \PHPUnit\Framework\MockObject\Stub&\stdClass
     */
    private Stub $a;

    /** @var \PHPUnit\Framework\MockObject\Stub&Foo */
    private Stub $b;

    /** @var \Some\Stub */
    private Stub $c;

    /** @var Foo|Bar */
    private Stub $d;

    /** @var Foo&Bar */
    private Stub $e;

    /** @var Foo[] */
    private Stub $f;

    /**
     * @var \PHPUnit\Framework\MockObject\Stub&\static
     */
    private Stub $g, $h;

    /**
     * Text.
     * @var \PHPUnit\Framework\MockObject\Stub&Foo $i
     */
    private Stub $i;

    private ?Stub $j;

    protected function setUp(): void
    {
        $this->a = $this->createStub(\stdClass::class);
        $this->e = $this->createStub(\stdClass::class);
        $this->g = self::createStub(static::class);
        $this->h = $this->createStub(\stdClass::class);
    }
}

final class NoSetUpTest extends TestCase
{
    /** @var \PHPUnit\Framework\MockObject\Stub&Foo */
    private Stub $a;

    private Stub $b;
}
