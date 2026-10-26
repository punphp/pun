<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;
use Symfony\Component\HttpFoundation as Http;

final class SomeTest extends TestCase
{
    public function test()
    {
        $this->run('Symfony\Component\HttpFoundation\RequestStack');
        new Service(($this->createMock(Http\RequestStack::class)), $this->createMock(Other::class));
        $x = new class($this->createMock(\Symfony\Component\HttpFoundation\Request::class)) {
            public function __construct($r) {}
        };
        $this->run(request: $this->createMock(Http\Request::class));
        foo($this->createMock(Http\Request::class));
        $this->run(self::createMock(Http\Request::class), $this->CREATEMOCK('Symfony\Component\HttpFoundation\Request'));
        $this->run($this->createMock());
    }
}

final class NotATest
{
    public function test()
    {
        $this->run($this->createMock(\Symfony\Component\HttpFoundation\Request::class));
    }
}
-----
<?php

namespace App\Tests;

use PHPUnit\Framework\TestCase;
use Symfony\Component\HttpFoundation as Http;

final class SomeTest extends TestCase
{
    public function test()
    {
        $this->run('Symfony\Component\HttpFoundation\RequestStack');
        new Service((new \Symfony\Component\HttpFoundation\RequestStack()), $this->createMock(Other::class));
        $x = new class(new \Symfony\Component\HttpFoundation\Request()) {
            public function __construct($r) {}
        };
        $this->run(request: new \Symfony\Component\HttpFoundation\Request());
        foo($this->createMock(Http\Request::class));
        $this->run(self::createMock(Http\Request::class), new \Symfony\Component\HttpFoundation\Request());
        $this->run($this->createMock());
    }
}

final class NotATest
{
    public function test()
    {
        $this->run($this->createMock(\Symfony\Component\HttpFoundation\Request::class));
    }
}
