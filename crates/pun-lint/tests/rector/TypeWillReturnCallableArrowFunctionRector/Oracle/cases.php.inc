<?php

namespace App;

use PHPUnit\Framework\TestCase;

class Mocked
{
    public function noTypes($a) {}
    public function obj(Mocked $m): Mocked { return $m; }
    public function nullable(?Mocked $m, ?int $i): ?Mocked { return $m; }
    public function union(int|string $a): int|false { return 1; }
    public function isVoid(int $a): void {}
    public function isStatic(): static { return $this; }
    public function isSelf(self $s): self { return $this; }
    public function closure(\Closure $c, callable $d, iterable $e, array ...$rest): array { return []; }
    public function test(MockTest $t): MockTest { return $t; }
    public function mixedRet(mixed $m): mixed { return 1; }
    public function nullRet(): ?int { return 1; }
}

final class MockTest extends TestCase
{
    private $fromSetUp;

    protected function setUp(): void
    {
        $this->fromSetUp = $this->createMock(Mocked::class);
    }

    public function testSetUp(): void
    {
        $this->fromSetUp->method('nullRet')->willReturnCallback(fn () => 1);
        $local = new Mocked();
        $local->method('nullRet')->willReturnCallback(fn () => 1);
        $m = $this->createMock(Mocked::class);
        $m->method('isVoid')->willReturnCallback(function (&$a) use ($local) {});
        $m->method('isVoid')->willReturnCallback(...);
    }

    public function testIt(): void
    {
        $m = $this->createMock(Mocked::class);
        $m->method('noTypes')->willReturnCallback(fn ($a) => 1);
        $m->method('obj')->willReturnCallback(fn ($m) => $m);
        $m->method('nullable')->willReturnCallback(fn ($m, $i) => $m);
        $m->method('union')->willReturnCallback(fn ($a) => 1);
        $m->method('isVoid')->willReturnCallback(function ($a) {});
        $m->method('isStatic')->willReturnCallback(fn () => 1);
        $m->method('isSelf')->willReturnCallback(fn ($s) => 1);
        $m->method('closure')->willReturnCallback(fn ($c, $d, $e, ...$rest) => []);
        $m->method('test')->willReturnCallback(fn ($t) => $t);
        $m->method('mixedRet')->willReturnCallback(fn ($m) => 1);
        $m->method('nullRet')->willReturnCallback(function () { return 1; });
        $m->method('nullRet')->willReturnCallback(function () { return 'a'; });
        $m->method('nullRet')->willReturnCallback(function () { if (rand()) { return 1; } return null; });
        $m->method('nullRet')->willReturnCallback(function () { return fn () => 1; });
        $m->method('union')->willReturnCallback(function (int $a) { return 'x'; });
        $m->method('UNION')->willReturnCallback(fn ($a) => 1);
        $m->method('missing')->willReturnCallback(fn ($a) => 1);
        $m->method('union')->willReturn(fn ($a) => 1);
        $m->method('union')->willReturnCallback('strlen');
        $m->expects($this->once())->method('union')->willReturnCallback(fn ($a) => 1);
        $m->method('union')->with($this->callback(fn ($a) => true));
        $m->method('union')->with($this->callback('is_int'));
        $m->method('union')->with(1)->willReturnCallback(fn ($a) => 1);
    }
}
-----
<?php

namespace App;

use PHPUnit\Framework\TestCase;

class Mocked
{
    public function noTypes($a) {}
    public function obj(Mocked $m): Mocked { return $m; }
    public function nullable(?Mocked $m, ?int $i): ?Mocked { return $m; }
    public function union(int|string $a): int|false { return 1; }
    public function isVoid(int $a): void {}
    public function isStatic(): static { return $this; }
    public function isSelf(self $s): self { return $this; }
    public function closure(\Closure $c, callable $d, iterable $e, array ...$rest): array { return []; }
    public function test(MockTest $t): MockTest { return $t; }
    public function mixedRet(mixed $m): mixed { return 1; }
    public function nullRet(): ?int { return 1; }
}

final class MockTest extends TestCase
{
    private $fromSetUp;

    protected function setUp(): void
    {
        $this->fromSetUp = $this->createMock(Mocked::class);
    }

    public function testSetUp(): void
    {
        $this->fromSetUp->method('nullRet')->willReturnCallback(fn () => 1);
        $local = new Mocked();
        $local->method('nullRet')->willReturnCallback(fn () => 1);
        $m = $this->createMock(Mocked::class);
        $m->method('isVoid')->willReturnCallback(function (int &$a) use ($local): void {});
        $m->method('isVoid')->willReturnCallback(...);
    }

    public function testIt(): void
    {
        $m = $this->createMock(Mocked::class);
        $m->method('noTypes')->willReturnCallback(fn ($a) => 1);
        $m->method('obj')->willReturnCallback(fn ($m) => $m);
        $m->method('nullable')->willReturnCallback(fn (?\App\Mocked $m, ?int $i): ?\App\Mocked => $m);
        $m->method('union')->willReturnCallback(fn (int|string $a): int|false => 1);
        $m->method('isVoid')->willReturnCallback(function (int $a): void {});
        $m->method('isStatic')->willReturnCallback(fn () => 1);
        $m->method('isSelf')->willReturnCallback(fn ($s) => 1);
        $m->method('closure')->willReturnCallback(fn ($c, callable $d, iterable $e, ...$rest): array => []);
        $m->method('test')->willReturnCallback(fn (\App\MockTest $t): \App\MockTest => $t);
        $m->method('mixedRet')->willReturnCallback(fn ($m): mixed => 1);
        $m->method('nullRet')->willReturnCallback(function () { return 1; });
        $m->method('nullRet')->willReturnCallback(function () { return 'a'; });
        $m->method('nullRet')->willReturnCallback(function (): ?int { if (rand()) { return 1; } return null; });
        $m->method('nullRet')->willReturnCallback(function () { return fn () => 1; });
        $m->method('union')->willReturnCallback(function (int $a) { return 'x'; });
        $m->method('UNION')->willReturnCallback(fn (int|string $a): int|false => 1);
        $m->method('missing')->willReturnCallback(fn ($a) => 1);
        $m->method('union')->willReturn(fn ($a) => 1);
        $m->method('union')->willReturnCallback('strlen');
        $m->expects($this->once())->method('union')->willReturnCallback(fn (int|string $a): int|false => 1);
        $m->method('union')->with($this->callback(fn (int|string $a): int|false => true));
        $m->method('union')->with($this->callback('is_int'));
        $m->method('union')->with(1)->willReturnCallback(fn ($a) => 1);
    }
}
