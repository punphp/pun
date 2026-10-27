<?php

final class ExpressionOrderTest extends \PHPUnit\Framework\TestCase
{
    /** @return float|string */
    private function documented()
    {
        return rand(0, 1) ? 1.5 : 'x';
    }

    public function testOrder(bool $a, float|string $forward, string|float $reverse)
    {
        $m->with($this->callback(function ($x): bool { return true; }))->willReturn($a ? 1 : 'x');
        $m->with($this->callback(function ($x): bool { return true; }))->willReturn($a ? 'x' : 1);
        $m->with($this->callback(function ($x): bool { return true; }))->willReturn($forward);
        $m->with($this->callback(function ($x): bool { return true; }))->willReturn($reverse);
        $m->with($this->callback(function ($x): bool { return true; }))->willReturn($this->documented());
    }
}
-----
<?php

final class ExpressionOrderTest extends \PHPUnit\Framework\TestCase
{
    /** @return float|string */
    private function documented()
    {
        return rand(0, 1) ? 1.5 : 'x';
    }

    public function testOrder(bool $a, float|string $forward, string|float $reverse)
    {
        $m->willReturnCallback(function ($x) use ($a): int|string { return $a ? 1 : 'x'; });
        $m->willReturnCallback(function ($x) use ($a): string|int { return $a ? 'x' : 1; });
        $m->willReturnCallback(function ($x) use ($forward): float|string { return $forward; });
        $m->willReturnCallback(function ($x) use ($reverse): string|float { return $reverse; });
        $m->willReturnCallback(function ($x): float|string { return $this->documented(); });
    }
}
