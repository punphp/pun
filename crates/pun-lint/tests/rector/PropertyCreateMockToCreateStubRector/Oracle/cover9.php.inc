<?php

namespace App\Tests\Cover9;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\MockObject\Stub;
use PHPUnit\Framework\TestCase;

class BaseTest extends TestCase
{
}

final class Cover9Test extends BaseTest
{
    private MockObject $parentMock;

    private MockObject $dynamicMock;

    /**
     * @varx MockObject
     */
    private Stub $p1;

    /**
     * @var Foo|Bar&MockObject
     */
    private Stub $p2;

    /**
     * @var MockObject[]
     */
    private Stub $p3;

    /**
     * @var 1Foo
     */
    private Stub $p4;

    protected function setUp(): void
    {
        $x = new \stdClass();
        $this->parentMock = $this->createMock(parent::class);
        $this->dynamicMock = $this->createMock($x::class);
    }
}
-----
<?php

namespace App\Tests\Cover9;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\MockObject\Stub;
use PHPUnit\Framework\TestCase;

class BaseTest extends TestCase
{
}

final class Cover9Test extends BaseTest
{
    private \PHPUnit\Framework\MockObject\Stub $parentMock;

    private \PHPUnit\Framework\MockObject\Stub $dynamicMock;

    /**
     * @varx MockObject
     */
    private Stub $p1;

    /**
     * @var Foo|Bar&MockObject
     */
    private Stub $p2;

    /**
     * @var MockObject[]
     */
    private Stub $p3;

    /**
     * @var 1Foo
     */
    private Stub $p4;

    protected function setUp(): void
    {
        $x = new \stdClass();
        $this->parentMock = $this->createStub(parent::class);
        $this->dynamicMock = $this->createStub($x::class);
    }
}
