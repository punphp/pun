<?php

final class U1Test extends \PHPUnit\Framework\TestCase
{
    public function test($a, $b)
    {
        $m->with($this->callback(function ($x) use ($a) {
            // check
            return TRUE;
        }), 'extra')->willReturn([$a, $b, $x, $this, fn () => $c, $b]);
        $m->with($this->callback(function ($x): bool { return true; }))->willReturn(true);
        $m->with($this->callback(function ($x): bool { return true; }))->willReturn(null);
        $m->with($this->callback(function ($x): bool { return true; }))->willReturn(new \DateTime());
        $m->with($this->callback(function ($x): bool { return true; }))->willReturn($a ? 1 : 'x');
        $m->with($this->callback(function ($x): bool { return false; }))->willReturn(1);
        $m->with($this->callback(function ($x): bool { return true; // c
        }))->willReturn(1);
        $m->with($this->callback(fn ($x) => true))->willReturn(1);
    }
}
-----
<?php

final class U1Test extends \PHPUnit\Framework\TestCase
{
    public function test($a, $b)
    {
        $m->willReturnCallback(function ($x) use ($a, $b, $c): array {
            // check
            return [$a, $b, $x, $this, fn () => $c, $b];
        });
        $m->willReturnCallback(function ($x): true { return true; });
        $m->willReturnCallback(function ($x): null { return null; });
        $m->willReturnCallback(function ($x): \DateTime { return new \DateTime(); });
        $m->willReturnCallback(function ($x) use ($a): int|string { return $a ? 1 : 'x'; });
        $m->with($this->callback(function ($x): bool { return false; }))->willReturn(1);
        $m->with($this->callback(function ($x): bool { return true; // c
        }))->willReturn(1);
        $m->with($this->callback(fn ($x) => true))->willReturn(1);
    }
}
