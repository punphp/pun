<?php

namespace Pun\Tests;

use PHPUnit\Framework\TestCase;
use Pun\Entity;

final class RectorFailsTest extends TestCase
{
    public function testWillReturnCallable()
    {
        $aMock = $this->createMock(Entity\Child::class);
        $aMock->method('getA')->willReturn(...);
    }
}
-----
<?php

namespace Pun\Tests;

use PHPUnit\Framework\TestCase;
use Pun\Entity;

final class RectorFailsTest extends TestCase
{
    public function testWillReturnCallable()
    {
        $a = new \Pun\Entity\Child();
        $a->method('getA')->willReturn(...);
    }
}
