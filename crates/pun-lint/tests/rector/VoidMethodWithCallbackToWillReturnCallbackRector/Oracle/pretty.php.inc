<?php

namespace Pun\Vm;

use PHPUnit\Framework\TestCase;

final class PrettyTest extends TestCase
{
    public function testPretty(): void
    {
        $m = $this->createMock(Manager::class);
        $m->method('persist')->with($this->callback(function ($e = 1) {
            return true;
        }));
        $m->method('persist')->with($this->callback(#[Pure] #[Other(1)] function ($e) {
            return true;
        }));
        $m->method('persist')->with($this->callback(static fn ($e) => true));
        $m->method('persist')->with($this->callback(function & (? object $e, int ...$rest) use ($m, &$e) {
            return true;
        }));
	$m->method('persist')->with($this->callback(function ($e) {
		echo 1;
		return true;
	}));
    }
}
-----
<?php

namespace Pun\Vm;

use PHPUnit\Framework\TestCase;

final class PrettyTest extends TestCase
{
    public function testPretty(): void
    {
        $m = $this->createMock(Manager::class);
        $m->method('persist')->willReturnCallback(function ($e = 1): void {
        });
        $m->method('persist')->willReturnCallback(#[Pure] #[Other(1)] function ($e): void {
        });
        $m->method('persist')->with($this->callback(static fn ($e) => true));
        $m->method('persist')->willReturnCallback(function &(? object $e, int ...$rest) use ($m, &$e): void {
        });
	$m->method('persist')->willReturnCallback(function ($e): void {
		echo 1;
	});
    }
}
