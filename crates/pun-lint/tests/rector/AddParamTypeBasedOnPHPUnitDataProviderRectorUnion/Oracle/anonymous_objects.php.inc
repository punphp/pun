<?php

namespace Probe;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\TestCase;

class Leaf
{
}

class MadeTest extends TestCase
{
    #[DataProvider('computed')]
    public function testComputed($value)
    {
    }

    #[DataProvider('objects')]
    public function testObjects($value)
    {
    }

    public static function computed()
    {
        return self::objects();
    }

    public static function objects()
    {
        return [[new Leaf()], [new class {}]];
    }
}
-----
<?php

namespace Probe;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\TestCase;

class Leaf
{
}

class MadeTest extends TestCase
{
    #[DataProvider('computed')]
    public function testComputed($value)
    {
    }

    #[DataProvider('objects')]
    public function testObjects(object $value)
    {
    }

    public static function computed()
    {
        return self::objects();
    }

    public static function objects()
    {
        return [[new Leaf()], [new class {}]];
    }
}
