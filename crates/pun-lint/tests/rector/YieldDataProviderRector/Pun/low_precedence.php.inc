<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

final class PrecTest extends TestCase
{
    /**
     * @dataProvider p1
     */
    public function test(int $a): void
    {
    }

    public static function p1()
    {
        return [$a and $b, print 1, fn() => 1, ($a), (($a)), include 'x', throw $e, ($a or $b), ($a) => ($b), $a = 1, yield from $x, yield $a => $b];
    }
}
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

final class PrecTest extends TestCase
{
    /**
     * @dataProvider p1
     */
    public function test(int $a): void
    {
    }

    public static function p1(): \Iterator
    {
        yield ($a and $b);
        yield print 1;
        yield fn() => 1;
        yield $a;
        yield $a;
        yield include 'x';
        yield throw $e;
        yield ($a or $b);
        yield $a => $b;
        yield $a = 1;
        yield yield from $x;
        yield yield $a => $b;
    }
}
