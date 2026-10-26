<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

class Service
{
    public function __construct(...$args) {}
}

abstract class AbstractService {}

final class SomeTest extends TestCase
{
    private $other;

    public function testUnused()
    {
        $unused = $this->createMock(Service::class);
        $this->run();
    }

    public function testAbstract()
    {
        $abstract = $this->createMock(AbstractService::class);
        new Service($abstract);
    }

    public function testCase()
    {
        $Mock = $this->createMock('App\Tests\Service');
        // comment
        Service::make(1, $mock, fn () => 1);
    }

    public function testAssert()
    {
        $asserted = $this->createMock(Service::class);
        $this->assertNotNull($asserted);
    }

    public function testClosure()
    {
        $closed = $this->createMock(Service::class);
        $f = function () use ($closed) {};
        new Service($closed);
    }

    public function testProperty()
    {
        $other = $this->createMock(Service::class);
        $this->other->run();
        new Service($other);
    }

    public function testArrow()
    {
        $arrow = $this->createMock(Service::class);
        $f = fn () => new Service($arrow);
    }

    public function testLoop()
    {
        foreach ([1, 2] as $i) {
            $looped = $this->createMock(
                Service::class
            );
            $this->run($looped);
        }
        foreach ([1] as $j):
            $colon = $this->createMock(Service::class);
            new class($colon) {
                public function __construct($x) {}
            };
        endforeach;
    }

    public function testTwice()
    {
        $twice = $this->createMock(Service::class);
        $this->run($twice, $twice);
    }
}
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;

class Service
{
    public function __construct(...$args) {}
}

abstract class AbstractService {}

final class SomeTest extends TestCase
{
    private $other;

    public function testUnused()
    {
        $this->run();
    }

    public function testAbstract()
    {
        $abstract = $this->createMock(AbstractService::class);
        new Service($abstract);
    }

    public function testCase()
    {
        // comment
        Service::make(1, $this->createMock('App\Tests\Service'), fn () => 1);
    }

    public function testAssert()
    {
        $asserted = $this->createMock(Service::class);
        $this->assertNotNull($asserted);
    }

    public function testClosure()
    {
        $closed = $this->createMock(Service::class);
        $f = function () use ($closed) {};
        new Service($closed);
    }

    public function testProperty()
    {
        $other = $this->createMock(Service::class);
        $this->other->run();
        new Service($other);
    }

    public function testArrow()
    {
        $f = fn () => new Service($this->createMock(Service::class));
    }

    public function testLoop()
    {
        foreach ([1, 2] as $i) {
            $this->run($this->createMock(
                Service::class
            ));
        }
        foreach ([1] as $j):
            new class($this->createMock(Service::class)) {
                public function __construct($x) {}
            };
        endforeach;
    }

    public function testTwice()
    {
        $twice = $this->createMock(Service::class);
        $this->run($twice, $twice);
    }
}
