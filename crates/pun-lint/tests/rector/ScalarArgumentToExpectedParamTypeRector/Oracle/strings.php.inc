<?php

namespace App\Tests\ExactArg;

use PHPUnit\Framework\TestCase;

final class Value
{
    public function __construct(int $value)
    {
    }
}

final class ExactArgTest extends TestCase
{
    public function testIt($method, $class): void
    {
        $this->takesInt('1.5');
        $this->takesInt('.5');
        $this->takesInt('.');
        $this->takesInt('1.');
        $this->takesInt('1e3');
        $this->takesInt('1e+3');
        $this->takesInt('1e-3');
        $this->takesInt('1e');
        $this->takesInt('1e999');
        $this->takesInt('-1e999');
        $this->takesInt('-9.3e18');
        $this->takesInt('9.3e18');
        $this->takesInt('99999999999999999999');
        $this->takesInt(' +12abc');
        $this->takesInt('abc');
        new Value;
        new Value('2');
        new $class('2');
        self::$method('1');
        self::takesStatic('3');
    }

    private function takesInt(int $value): void
    {
    }

    private static function takesStatic(int $value): void
    {
    }
}
-----
<?php

namespace App\Tests\ExactArg;

use PHPUnit\Framework\TestCase;

final class Value
{
    public function __construct(int $value)
    {
    }
}

final class ExactArgTest extends TestCase
{
    public function testIt($method, $class): void
    {
        $this->takesInt(1);
        $this->takesInt(0);
        $this->takesInt(0);
        $this->takesInt(1);
        $this->takesInt(1000);
        $this->takesInt(1000);
        $this->takesInt(0);
        $this->takesInt(1);
        $this->takesInt(0);
        $this->takesInt(0);
        $this->takesInt((-9223372036854775807-1));
        $this->takesInt(9223372036854775807);
        $this->takesInt(9223372036854775807);
        $this->takesInt(12);
        $this->takesInt(0);
        new Value;
        new Value(2);
        new $class('2');
        self::$method('1');
        self::takesStatic(3);
    }

    private function takesInt(int $value): void
    {
    }

    private static function takesStatic(int $value): void
    {
    }
}
