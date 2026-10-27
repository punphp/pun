<?php

namespace App\Tests;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\MockObject\MockObject as Mocked;
use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    private MockObject&\stdClass $a;
    protected \PHPUnit\Framework\MockObject\MockObject|\stdClass $b;
    private MockObject|\stdClass|null $c;
    private ?MockObject $d;
    private (MockObject&\stdClass)|null $e;
    public Mocked|\Countable $f, $g;
    public \stdClass|MOCKOBJECT $h;
    private \stdClass|\ArrayAccess $i;
    public MockObject|\stdClass $j {
        get => $this->j;
    }
    private $k;

    public function test(): void
    {
        $x = new class {
            private MockObject|\stdClass $a;
        };
    }
}

final class NotATest
{
    private MockObject&\stdClass $a;
}
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\MockObject\MockObject as Mocked;
use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    private MockObject $a;
    protected \PHPUnit\Framework\MockObject\MockObject $b;
    private MockObject|\stdClass|null $c;
    private ?MockObject $d;
    private (MockObject&\stdClass)|null $e;
    public Mocked $f, $g;
    public MOCKOBJECT $h;
    private \stdClass|\ArrayAccess $i;
    public MockObject $j {
        get => $this->j;
    }
    private $k;

    public function test(): void
    {
        $x = new class {
            private MockObject|\stdClass $a;
        };
    }
}

final class NotATest
{
    private MockObject&\stdClass $a;
}
