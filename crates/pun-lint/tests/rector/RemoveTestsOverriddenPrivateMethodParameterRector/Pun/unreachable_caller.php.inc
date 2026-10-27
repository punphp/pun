<?php

namespace Tests\Unreachable;

use PHPUnit\Framework\TestCase;

class UntypedTest extends TestCase
{
    public function testDead()
    {
        return;
        $this->fromDead(1, 2);
    }

    private function fromDead($a, $b)
    {
        $b = 1;
        return $a + $b;
    }
}
-----
<?php

namespace Tests\Unreachable;

use PHPUnit\Framework\TestCase;

class UntypedTest extends TestCase
{
    public function testDead()
    {
        return;
        $this->fromDead(1, 2);
    }

    private function fromDead($a)
    {
        $b = 1;
        return $a + $b;
    }
}
