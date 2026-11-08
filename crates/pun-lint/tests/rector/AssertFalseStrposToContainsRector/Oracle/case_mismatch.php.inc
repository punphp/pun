<?php

namespace App\Tests\Pun;

use PHPUnit\Framework\TestCase;

final class CaseMismatchTest extends TestCase
{
    public function testIt($a, $b): void
    {
        $this->AssertTrue($a > $b);
        $this->AssertTrue($a instanceof \stdClass);
        $this->AssertTrue(!$a);
        $this->ASSERTFALSE(strpos($a, 'x'));
        $this->AssertFalse(isset($a['x']));
        $this->ASSERTSAME(null, $a);
        $this->ASSERTEQUALS('a', 'b');
    }
}
-----
<?php

namespace App\Tests\Pun;

use PHPUnit\Framework\TestCase;

final class CaseMismatchTest extends TestCase
{
    public function testIt($a, $b): void
    {
        $this->AssertTrue($a > $b);
        $this->AssertTrue($a instanceof \stdClass);
        $this->AssertTrue(!$a);
        $this->assertStringNotContainsString('x', $a);
        $this->AssertFalse(isset($a['x']));
        $this->ASSERTSAME(null, $a);
        $this->ASSERTEQUALS('a', 'b');
    }
}
