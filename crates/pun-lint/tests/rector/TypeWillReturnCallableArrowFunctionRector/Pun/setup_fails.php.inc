<?php

namespace App;

use PHPUnit\Framework\TestCase;

class Mocked
{
    public function run(int $a): int { return $a; }
}

final class SetUpFailsTest extends TestCase
{
    private $mock;

    protected function setUp(): void
    {
        $this->mock = $this->createMock(('A' . 'B')::class);
    }

    public function testIt(): void
    {
        $this->createMock(Mocked::class)->method('run')->willReturnCallback(fn ($a) => $a);
    }
}
