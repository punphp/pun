<?php

namespace Probe;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\TestCase;

class MadeTest extends TestCase
{
    public static function make(): static
    {
        return new static('a');
    }

    #[DataProvider('made')]
    public function testMade($value)
    {
    }

    #[DataProvider('computed')]
    public function testComputed($value)
    {
    }

    public static function made()
    {
        return [[static::make()]];
    }

    public static function computed()
    {
        return self::made();
    }
}
-----
<?php

namespace Probe;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\TestCase;

class MadeTest extends TestCase
{
    public static function make(): static
    {
        return new static('a');
    }

    #[DataProvider('made')]
    public function testMade(self $value)
    {
    }

    #[DataProvider('computed')]
    public function testComputed($value)
    {
    }

    public static function made()
    {
        return [[static::make()]];
    }

    public static function computed()
    {
        return self::made();
    }
}
