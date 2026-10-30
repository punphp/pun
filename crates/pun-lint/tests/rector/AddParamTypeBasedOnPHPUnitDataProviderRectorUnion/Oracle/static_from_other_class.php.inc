<?php

namespace Probe;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\TestCase;

class Leaf
{
    /**
     * @return static
     */
    public static function now()
    {
        return new static();
    }
}

final class NowTest extends TestCase
{
    #[DataProvider('nows')]
    public function testNow($value)
    {
    }

    public static function nows()
    {
        return [[Leaf::now()]];
    }
}
-----
<?php

namespace Probe;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\TestCase;

class Leaf
{
    /**
     * @return static
     */
    public static function now()
    {
        return new static();
    }
}

final class NowTest extends TestCase
{
    #[DataProvider('nows')]
    public function testNow(\Probe\Leaf $value)
    {
    }

    public static function nows()
    {
        return [[Leaf::now()]];
    }
}
