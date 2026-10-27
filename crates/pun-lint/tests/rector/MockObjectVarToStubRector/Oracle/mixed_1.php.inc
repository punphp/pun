<?php

namespace App\Tests;

use PHPUnit\Framework\MockObject\Stub;
use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    /**
     * @var MockObject
     */
    private Stub $a;

        /**
   * Some text
        * @var \PHPUnit\Framework\MockObject\MockObject|Foo
         */
    private Stub&\Countable $b;

    /** @var Foo&MockObject&\Some\MockObject */
    protected \PHPUnit\Framework\MockObject\Stub $c;

    /** @var MockObject<Foo> */
    private Stub $d;

    /** @var ?MockObject */
    private Stub $e;

    /**
     * @var Foo
     * @var MockObject
     */
    private Stub $f;

    /** @var MockObject */
    private MockObject $g;

    /** @var (MockObject|Foo)&Bar */
    private Stub $h;

    /** @var MockObject $i comment */
    private Stub $i;

    /** @var   MockObject|Foo   */
    private Stub $j;
}
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\MockObject\Stub;
use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    /**
     * @var \PHPUnit\Framework\MockObject\Stub
     */
    private Stub $a;

        /**
     * Some text
     * @var \PHPUnit\Framework\MockObject\Stub|Foo
     */
    private Stub&\Countable $b;

    /** @var Foo&\PHPUnit\Framework\MockObject\Stub&\PHPUnit\Framework\MockObject\Stub */
    protected \PHPUnit\Framework\MockObject\Stub $c;

    /** @var MockObject<Foo> */
    private Stub $d;

    /** @var ?MockObject */
    private Stub $e;

    /**
     * @var Foo
     * @var MockObject
     */
    private Stub $f;

    /** @var MockObject */
    private MockObject $g;

    /** @var (MockObject|Foo)&Bar */
    private Stub $h;

    /** @var \PHPUnit\Framework\MockObject\Stub $i comment */
    private Stub $i;

    /** @var \PHPUnit\Framework\MockObject\Stub|Foo   */
    private Stub $j;
}
