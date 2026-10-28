<?php

namespace Probe;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\TestCase;

final class CastTest extends TestCase
{
    #[DataProvider('provider')]
    public function testCast($data)
    {
    }

    public static function provider()
    {
        return [
            'array' => [['foo' => 'bar']],
            'object' => [(object) ['foo' => 'bar']],
        ];
    }

    #[DataProvider('onlyObjects')]
    public function testObject($data)
    {
    }

    public static function onlyObjects()
    {
        return [
            [(object) ['foo' => 'bar']],
            [(object) []],
        ];
    }
}
-----
<?php

namespace Probe;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\TestCase;

final class CastTest extends TestCase
{
    #[DataProvider('provider')]
    public function testCast(\stdClass|array $data)
    {
    }

    public static function provider()
    {
        return [
            'array' => [['foo' => 'bar']],
            'object' => [(object) ['foo' => 'bar']],
        ];
    }

    #[DataProvider('onlyObjects')]
    public function testObject(\stdClass $data)
    {
    }

    public static function onlyObjects()
    {
        return [
            [(object) ['foo' => 'bar']],
            [(object) []],
        ];
    }
}
