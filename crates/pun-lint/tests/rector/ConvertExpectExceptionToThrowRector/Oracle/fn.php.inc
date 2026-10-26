<?php

function foo() {
    $a = 1;
    $this->expectException(RuntimeException::class);
    doSomething();
}

class T {
    public function testA(): void
    {
        $this->expectException(RuntimeException::class);
        $this->expectExceptionMessage('m');
        $this->run();
        $b = 2;
    }
}

if ($x) {
    $y = 1;
    $this->expectException(RuntimeException::class);
    doSomething();
}
-----
<?php

function foo() {
    $a = 1;
    expect(fn() => doSomething())->toThrow(RuntimeException::class);
}

class T {
    public function testA(): void
    {
        expect(fn() => $this->run())->toThrow(RuntimeException::class, 'm');
        $b = 2;
    }
}

if ($x) {
    $y = 1;
    expect(fn() => doSomething())->toThrow(RuntimeException::class);
}
