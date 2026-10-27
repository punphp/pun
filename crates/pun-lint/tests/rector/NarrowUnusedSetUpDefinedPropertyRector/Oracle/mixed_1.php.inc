<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

trait LocalTrait
{
    private $fromTrait;

    public function helper(): void
    {
        $this->fetchedInTrait = 1;
        self::$staticInTrait = 2;
        $this?->nullsafeInTrait;
    }
}

final class SomeTest extends TestCase
{
    use LocalTrait;

    private $a;
    protected $b;
    private static $c;
    private $d = 1;
    private $e, $f;
    private $fromTrait2;
    private $fetchedInTrait;
    private $staticInTrait;
    private $nullsafeInTrait;
    private $g;
    private $h;
    private $i;
    private $j;
    private $k;
    public $l {
        get => 1;
    }

    protected function setUp(): void
    {
        $this->a = new \stdClass();
        $this->a->foo = $this->a;
        $this->b = 1;
        $this->fetchedInTrait = 1;
        $this->staticInTrait = 1;
        $this->nullsafeInTrait = 1;
        $fn = function () {
            return $this->g;
        };
        $arrow = fn () => $this->h;
        $this->h = 1;
        $this->i = 1;
        $arrow = fn () => $this->i;
        $this->j = fn () => $this->j;
        $this->k = 1;
    }

    public function testIt(): void
    {
        $this->k->foo();
        $other->a;
    }
}

abstract class AbstractTest extends TestCase
{
    private $a;
    protected $b;

    protected function setUp(): void
    {
        $this->a = 1;
        $this->b = 1;
    }
}
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

trait LocalTrait
{
    private $fromTrait;

    public function helper(): void
    {
        $this->fetchedInTrait = 1;
        self::$staticInTrait = 2;
        $this?->nullsafeInTrait;
    }
}

final class SomeTest extends TestCase
{
    use LocalTrait;
    private static $c;
    private $d = 1;
    private $e, $f;
    private $fetchedInTrait;
    private $staticInTrait;
    private $g;
    private $h;
    private $j;
    private $k;

    protected function setUp(): void
    {
        $a = new \stdClass();
        $a->foo = $a;
        $b = 1;
        $this->fetchedInTrait = 1;
        $this->staticInTrait = 1;
        $nullsafeInTrait = 1;
        $fn = function () {
            return $this->g;
        };
        $arrow = fn () => $this->h;
        $this->h = 1;
        $i = 1;
        $arrow = fn () => $i;
        $this->j = fn () => $this->j;
        $this->k = 1;
    }

    public function testIt(): void
    {
        $this->k->foo();
        $other->a;
    }
}

abstract class AbstractTest extends TestCase
{
    protected $b;

    protected function setUp(): void
    {
        $a = 1;
        $this->b = 1;
    }
}
