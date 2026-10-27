<?php

namespace App\Tests\Consecutive;

use PHPUnit\Framework\TestCase;

final class ConsecutiveTest extends TestCase
{
    public function testIt($a): void
    {
        $mock = $this->createMock(\stdClass::class);
        $mock->method('x')->withConsecutive(['k' => $this->equalTo(1), 2]);
        $mock->method('x')->withConsecutive([...$a, $this->equalTo(1)]);
        $mock->method('x')->withConsecutive([$this->equalTo()]);
        $mock->method('x')->withConsecutive($this->equalTo());
    }
}
-----
<?php

namespace App\Tests\Consecutive;

use PHPUnit\Framework\TestCase;

final class ConsecutiveTest extends TestCase
{
    public function testIt($a): void
    {
        $mock = $this->createMock(\stdClass::class);
        $mock->method('x')->with(1, 2);
        $mock->method('x')->with($a, 1);
        $mock->method('x')->with($this->equalTo());
        $mock->method('x')->with($this->equalTo());
    }
}
