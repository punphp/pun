<?php

namespace App\Tests;

use PHPUnit\Framework\MockObject\Stub;
use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    private Stub $a;
    private ?Stub $bStub;
    private Stub&\Countable $c;
    private Stub|\Countable $d;
    private Stub $e, $f;
    private $g;
    private Stub $h;
    private Stub $i;

    protected function setUp(): void
    {
        $this->a = $this->createStub(\Countable::class);
        $this->bStub = $this->createStub('Some\Thing');
        $this->c = $this->createStub(\Countable::class);
        $this->d = $this->createStub(\Countable::class);
        $this->e = $this->createStub(\Countable::class);
        $this->g = $this->createStub(\Countable::class);
        $this->h = $this->createStub(\Countable::class);
        $this->i = $this->createStub(\Countable::class);
        $other = 1;
    }

    public function testOne(): void
    {
        $x = 1;
        // comment before
        foo($this->a);
        bar([$this->a, 'k' => $this->bStub]);
        $y = new \ArrayObject($this->bStub);
        $fn = function () {
            return baz($this->c);
        };
        $z = $this->h?->foo();
        $w = qux(...$this->i);
    }

    public function testTwo(): void
    {
        foo($this->a);
        foo($this->c, $this->c);
    }
}
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\MockObject\Stub;
use PHPUnit\Framework\TestCase;

final class SomeTest extends TestCase
{
    private Stub&\Countable $c;
    private Stub|\Countable $d;
    private Stub $e, $f;
    private $g;
    private Stub $h;

    protected function setUp(): void
    {
        $this->c = $this->createStub(\Countable::class);
        $this->d = $this->createStub(\Countable::class);
        $this->e = $this->createStub(\Countable::class);
        $this->g = $this->createStub(\Countable::class);
        $this->h = $this->createStub(\Countable::class);
        $other = 1;
    }

    public function testOne(): void
    {
        $x = 1;
        $aStub = $this->createStub(\Countable::class);
        // comment before
        foo($aStub);
        $bStub = $this->createStub(\Some\Thing::class);
        bar([$aStub, 'k' => $bStub]);
        $y = new \ArrayObject($bStub);
        $fn = function () {
            return baz($this->c);
        };
        $z = $this->h?->foo();
        $w = qux(...$this->createStub(\Countable::class));
    }

    public function testTwo(): void
    {
        foo($this->createStub(\Countable::class));
        foo($this->c, $this->c);
    }
}
