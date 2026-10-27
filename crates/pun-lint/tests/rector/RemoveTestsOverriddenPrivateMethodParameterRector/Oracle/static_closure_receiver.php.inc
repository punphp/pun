<?php

namespace Tests\Untyped;

use PHPUnit\Framework\TestCase;

class UntypedTest extends TestCase
{
    public function testRun()
    {
        $fn = static function () {
            return $this->fromStatic(1, 2);
        };
        $fn();
    }

    private function fromStatic($a, $b)
    {
        $b = 1;
        return $a + $b;
    }
}
-----
<?php

namespace Tests\Untyped;

use PHPUnit\Framework\TestCase;

class UntypedTest extends TestCase
{
    public function testRun()
    {
        $fn = static function () {
            return $this->fromStatic(1, 2);
        };
        $fn();
    }

    private function fromStatic($a)
    {
        $b = 1;
        return $a + $b;
    }
}
