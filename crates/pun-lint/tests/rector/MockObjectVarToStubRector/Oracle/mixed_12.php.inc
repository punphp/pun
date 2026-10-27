<?php

namespace App\Tests;

use PHPUnit\Framework\MockObject\Stub;
use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    /** @var  MockObject   $i   comment */
    private Stub $i;

    /** @var MockObject | Foo  $j  text */
    private Stub $j;

    /** @phpstan-var MockObject */
    private Stub $k;

    /** @var Foo & MockObject */
    private Stub $l;

    /**
     * @var MockObject
     */
    #[Attr]
    private Stub $m;

    /** @var mockobject|MockObject[] */
    private Stub $o;

    /** @var MockObject */
    private Stub $p, $q;

    /** @var MockObject */
    private $r;

    /** @var MockObject */
    private Stub|Foo $s;
}

final class NotATest
{
    /** @var MockObject */
    private Stub $a;
}
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\MockObject\Stub;
use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    /** @var \PHPUnit\Framework\MockObject\Stub $i comment */
    private Stub $i;

    /** @var \PHPUnit\Framework\MockObject\Stub|Foo $j text */
    private Stub $j;

    /** @phpstan-var MockObject */
    private Stub $k;

    /** @var Foo&\PHPUnit\Framework\MockObject\Stub */
    private Stub $l;

    /**
     * @var \PHPUnit\Framework\MockObject\Stub
     */
    #[Attr]
    private Stub $m;

    /** @var mockobject|MockObject[] */
    private Stub $o;

    /** @var \PHPUnit\Framework\MockObject\Stub */
    private Stub $p, $q;

    /** @var MockObject */
    private $r;

    /** @var MockObject */
    private Stub|Foo $s;
}

final class NotATest
{
    /** @var MockObject */
    private Stub $a;
}
