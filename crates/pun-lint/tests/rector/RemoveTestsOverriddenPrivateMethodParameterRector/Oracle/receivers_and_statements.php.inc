<?php

namespace Tests\Receivers;

use PHPUnit\Framework\TestCase;

class BaseTest extends TestCase
{
    protected function inherited($a)
    {
        $a = 1;
        return $a;
    }
}

class ChildTest extends BaseTest
{
    private int $count = 0;

    public function testCalls(?self $maybe, self $object, int $number)
    {
        parent::inherited(1);
        ChildTest::fromName(1, 2);
        GrandChildTest::fromSubclass(1, 2);
        $object::fromValue(1, 2);
        $maybe->fromNullable(1, 2);
        $number->fromNumber(1, 2);
        $this->fewer(1);
        $this->loop(1);
        $f = 'strlen';
        $f('x');
    }

    private function fromName($a, $b)
    {
        $b = 1;
        return $a + $b;
    }

    private function fromSubclass($a, $b)
    {
        $b = 1;
        return $a + $b;
    }

    private function fromValue($a, $b)
    {
        $b = 1;
        return $a + $b;
    }

    private function fromNullable($a, $b)
    {
        $b = 1;
        return $a + $b;
    }

    private function fromNumber($a, $b)
    {
        $b = 1;
        return $a + $b;
    }

    private function fewer($a, $b = null)
    {
        $b = 1;
        return $a + $b;
    }

    private function loop($a)
    {
        $a = 1;
        if ($a > 5) {
            return $this->loop($a - 1);
        }
        return $a;
    }

    private function statements($a, $b, $c, $d, $e)
    {
        strlen($a);
        $b .= 'x';
        $c = &$this->count;
        $f = 'trim';
        $f($d);
        return [$a, $b, $c];
    }

    private function callable($a)
    {
        $a = 1;
        return self::callable(...);
    }
}

class GrandChildTest extends ChildTest
{
}

$anonymous = new class ('name') extends TestCase {
    public function testRun()
    {
        $this->helper(1);
    }

    private function helper($value)
    {
        $value = 2;
        return $value;
    }
};
-----
<?php

namespace Tests\Receivers;

use PHPUnit\Framework\TestCase;

class BaseTest extends TestCase
{
    protected function inherited($a)
    {
        $a = 1;
        return $a;
    }
}

class ChildTest extends BaseTest
{
    private int $count = 0;

    public function testCalls(?self $maybe, self $object, int $number)
    {
        parent::inherited(1);
        ChildTest::fromName(1);
        GrandChildTest::fromSubclass(1);
        $object::fromValue(1);
        $maybe->fromNullable(1);
        $number->fromNumber(1, 2);
        $this->fewer(1);
        $this->loop();
        $f = 'strlen';
        $f('x');
    }

    private function fromName($a)
    {
        $b = 1;
        return $a + $b;
    }

    private function fromSubclass($a)
    {
        $b = 1;
        return $a + $b;
    }

    private function fromValue($a)
    {
        $b = 1;
        return $a + $b;
    }

    private function fromNullable($a)
    {
        $b = 1;
        return $a + $b;
    }

    private function fromNumber($a)
    {
        $b = 1;
        return $a + $b;
    }

    private function fewer($a)
    {
        $b = 1;
        return $a + $b;
    }

    private function loop()
    {
        $a = 1;
        if ($a > 5) {
            return $this->loop();
        }
        return $a;
    }

    private function statements($a, $b, $c, $d, $e)
    {
        strlen($a);
        $b .= 'x';
        $c = &$this->count;
        $f = 'trim';
        $f($d);
        return [$a, $b, $c];
    }

    private function callable($a)
    {
        $a = 1;
        return self::callable(...);
    }
}

class GrandChildTest extends ChildTest
{
}

$anonymous = new class ('name') extends TestCase {
    public function testRun()
    {
        $this->helper();
    }

    private function helper()
    {
        $value = 2;
        return $value;
    }
};
