<?php

namespace App\Tests;

use PHPUnit\Framework\MockObject\MockObject as Mocked;
use PHPUnit\Framework\TestCase;
use Some\Thing;

final class SomeTest extends TestCase
{
    private Mocked  &  Thing $a;
    private \phpunit\framework\mockobject\mockobject&Thing $b;
    private (Mocked&Thing)|null $c;
    var $d;

    protected function setUp(): void
    {
        $this->a = $this->createMock(Thing::class);
        $this->b = $this->createMock(Thing::class);
        $this->c = $this->createMock(Thing::class);
        $this->d = $this->createMock(Thing::class);
    }
}
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\MockObject\MockObject as Mocked;
use PHPUnit\Framework\TestCase;
use Some\Thing;

final class SomeTest extends TestCase
{
    private \PHPUnit\Framework\MockObject\Stub&Thing $a;
    private \phpunit\framework\mockobject\mockobject&Thing $b;
    private \PHPUnit\Framework\MockObject\Stub $c;
    var \PHPUnit\Framework\MockObject\Stub $d;

    protected function setUp(): void
    {
        $this->a = $this->createStub(Thing::class);
        $this->b = $this->createStub(Thing::class);
        $this->c = $this->createStub(Thing::class);
        $this->d = $this->createStub(Thing::class);
    }
}
