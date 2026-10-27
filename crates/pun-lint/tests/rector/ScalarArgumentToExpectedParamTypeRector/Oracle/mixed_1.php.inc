<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

class Target
{
    public function __construct(string $s = '', int $i = 0)
    {
    }

    public function strings(string $a, ?string $b = null, string ...$rest)
    {
    }

    public function ints(int $a, ?int $b = null, int|string $c = 0, $d = 0, float $e = 0.0)
    {
    }

    public static function make(string $a)
    {
    }
}

class Child extends Target
{
}

final class SomeTest extends TestCase
{
    public function testIt(): void
    {
        $t = new Target(123, '45abc');
        $t->strings(1.0, 0.1, 1e15, 1e14);
        $t->strings(123.456, 1e20, 1.5e-5, 0.0001);
        $t->strings(123456789012345678.0, 0x1A, 0b11, 0o17);
        $t->strings(017, 1_000, 2.5, 9223372036854775807);
        $t->ints(' 12', '12abc', '1.9', 'abc');
        $t->ints('1e3', '-5', '+7');
        $t->ints('9999999999999999999', '1e100', '-1e100', ".5");
        $t->ints("\t\n 3 ", '0x1A', '', '1e', '1.', '.', '-');
        $t->ints(b: '5', a: '6');
        $t->ints(c: '5', zzz: '1');
        Target::make(5);
        Child::make(6);
        $c = new Child(7);
        $c->strings(8);
        $this->helper(9);
        self::staticHelper(10);
        static::staticHelper(11);
        $t->{'strings'}(12);
        $t->missing(13);
        $u = rand() ? $t : $c;
        $u->strings(14);
        new class(15) extends Target {};
        $x = new $cls(16);
    }

    private function helper(string $a)
    {
    }

    private static function staticHelper(string $a)
    {
    }
}

final class NotATest
{
    public function run(): void
    {
        $t = new Target(1);
    }
}
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

class Target
{
    public function __construct(string $s = '', int $i = 0)
    {
    }

    public function strings(string $a, ?string $b = null, string ...$rest)
    {
    }

    public function ints(int $a, ?int $b = null, int|string $c = 0, $d = 0, float $e = 0.0)
    {
    }

    public static function make(string $a)
    {
    }
}

class Child extends Target
{
}

final class SomeTest extends TestCase
{
    public function testIt(): void
    {
        $t = new Target('123', 45);
        $t->strings('1', '0.1', '1.0E+15', 1e14);
        $t->strings('123.456', '1.0E+20', '1.5E-5', 0.0001);
        $t->strings('1.2345678901235E+17', '26', '3', 0o17);
        $t->strings('15', '1000', '2.5', 9223372036854775807);
        $t->ints(12, 12, '1.9', 'abc');
        $t->ints(1000, -5, '+7');
        $t->ints(9223372036854775807, 9223372036854775807, '-1e100', ".5");
        $t->ints(3, 0, '', '1e', '1.', '.', '-');
        $t->ints(b: 5, a: 6);
        $t->ints(c: '5', zzz: 1);
        Target::make('5');
        Child::make('6');
        $c = new Child('7');
        $c->strings('8');
        $this->helper('9');
        self::staticHelper('10');
        static::staticHelper('11');
        $t->{'strings'}(12);
        $t->missing(13);
        $u = rand() ? $t : $c;
        $u->strings(14);
        new class(15) extends Target {};
        $x = new $cls(16);
    }

    private function helper(string $a)
    {
    }

    private static function staticHelper(string $a)
    {
    }
}

final class NotATest
{
    public function run(): void
    {
        $t = new Target(1);
    }
}
