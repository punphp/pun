<?php

namespace App;

use PHPUnit\Framework\MockObject\MockObject;
use PHPUnit\Framework\TestCase;

interface First
{
    public function run(int $a): int;
}

interface Second
{
    public function run(string $a): string;
}

final class IntersectionOrderTest extends TestCase
{
    public function testIt(): void
    {
        $this->both()->method('run')->willReturnCallback(fn ($a) => $a);
    }

    private function both(): MockObject&First&Second
    {
        return $this->createMock(First::class);
    }
}
