<?php

final class M4Test extends \PHPUnit\Framework\TestCase
{
    public function testMe()
    {
        self::run($a ?? $this->CreateMock('X'));
        new Foo(x: ($a ?? ($this->createMock('X'))));
        new class ($a ?? $this->createMock('X')) {};
        run($a ?? $this->createMock('X'));
        $x = [...$a ?? $this->createMock('X'), 'k' => $b ?? $c ?? $this->createMock('X')];
        $this->run($a ?: $this->createMock('X'));
        $this->run(fn () => $a ?? $this->createMock('X'));
    }
}
-----
<?php

final class M4Test extends \PHPUnit\Framework\TestCase
{
    public function testMe()
    {
        self::run($a ?? $this->createStub('X'));
        new Foo(x: ($a ?? ($this->createStub('X'))));
        new class ($a ?? $this->createStub('X')) {};
        run($a ?? $this->createMock('X'));
        $x = [...$a ?? $this->createStub('X'), 'k' => $b ?? $c ?? $this->createMock('X')];
        $this->run($a ?: $this->createMock('X'));
        $this->run(fn () => $a ?? $this->createMock('X'));
    }
}
