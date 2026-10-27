<?php

namespace App\Tests\Noargs2;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\TestCase;

final class NoArgsTest extends TestCase
{
    #[DataProvider]
    public function testIt($a): void
    {
        $mock = $this->createMock(\stdClass::class);
        $matcher = $this->exactly(1);
        $mock->method('x')->willReturnCallback(function ($p) use ($matcher) {
            match ($matcher->numberOfInvocations()) {
                1 => $this->assertSame([1], $p),
            };
            return match ($matcher->numberOfInvocations()) {};
        });
    }

    public static function provide(): array
    {
        return [[1]];
    }
}
-----
<?php

declare(strict_types=1);

namespace App\Tests\Noargs2;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\TestCase;

final class NoArgsTest extends TestCase
{
    #[DataProvider]
    public function testIt($a): void
    {
        $mock = $this->createMock(\stdClass::class);
        $matcher = $this->exactly(1);
        $mock->method('x')->willReturnCallback(function ($p) use ($matcher) {
            match ($matcher->numberOfInvocations()) {
                1 => $this->assertSame([1], $p),
            };
            return match ($matcher->numberOfInvocations()) {};
        });
    }

    public static function provide(): array
    {
        return [[1]];
    }
}
