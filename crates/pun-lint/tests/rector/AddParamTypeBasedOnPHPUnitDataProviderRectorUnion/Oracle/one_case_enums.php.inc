<?php

namespace Probe;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\TestCase;

enum Pure
{
    case One;
}

enum Backed: string
{
    case One = 'one';
}

final class EnumsTest extends TestCase
{
    #[DataProvider('pureFirst')]
    public function testPureFirst($value)
    {
    }

    #[DataProvider('backedFirst')]
    public function testBackedFirst($value)
    {
    }

    #[DataProvider('backedKeyed')]
    public function testBackedKeyed($value)
    {
    }

    #[DataProvider('backedTyped')]
    public function testBackedTyped($value)
    {
    }

    #[DataProvider('laterFirst')]
    public function testLaterFirst($value)
    {
    }

    public static function pureFirst()
    {
        return [[Pure::One], ['a'], [null]];
    }

    public static function backedFirst()
    {
        return [[Backed::One], ['a'], [null]];
    }

    public static function backedKeyed()
    {
        return ['x' => [Backed::One], 'y' => ['a']];
    }

    public static function backedTyped(): array
    {
        return [[Backed::One], [1]];
    }

    public static function laterFirst()
    {
        return [[Later::One], [1]];
    }
}

enum Later
{
    case One;
}
-----
<?php

namespace Probe;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\TestCase;

enum Pure
{
    case One;
}

enum Backed: string
{
    case One = 'one';
}

final class EnumsTest extends TestCase
{
    #[DataProvider('pureFirst')]
    public function testPureFirst(string|\Probe\Pure|null $value)
    {
    }

    #[DataProvider('backedFirst')]
    public function testBackedFirst(string|\Probe\Backed|null $value)
    {
    }

    #[DataProvider('backedKeyed')]
    public function testBackedKeyed(string|\Probe\Backed $value)
    {
    }

    #[DataProvider('backedTyped')]
    public function testBackedTyped(int|\Probe\Backed $value)
    {
    }

    #[DataProvider('laterFirst')]
    public function testLaterFirst(int|\Probe\Later $value)
    {
    }

    public static function pureFirst()
    {
        return [[Pure::One], ['a'], [null]];
    }

    public static function backedFirst()
    {
        return [[Backed::One], ['a'], [null]];
    }

    public static function backedKeyed()
    {
        return ['x' => [Backed::One], 'y' => ['a']];
    }

    public static function backedTyped(): array
    {
        return [[Backed::One], [1]];
    }

    public static function laterFirst()
    {
        return [[Later::One], [1]];
    }
}

enum Later
{
    case One;
}
