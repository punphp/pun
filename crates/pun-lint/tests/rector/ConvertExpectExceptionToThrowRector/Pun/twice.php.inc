<?php

test('g', function () {
    $this->expectException(RuntimeException::class);
    $this->expectException(LogicException::class);
    doSomething();
});
-----
<?php

test('g', function () {
    expect(fn() => expect(fn() => doSomething())->toThrow(LogicException::class))->toThrow(RuntimeException::class);
});
