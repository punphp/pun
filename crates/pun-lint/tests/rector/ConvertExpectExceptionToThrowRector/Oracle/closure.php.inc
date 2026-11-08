<?php

test('a', function () {
    $this->expectException(RuntimeException::class);
    doSomething();
});

test('b', function () {
    $service = make();

    // why
    $this->expectException(RuntimeException::class);
    /* msg */
    $this->expectExceptionMessage('boom');
    $service->run(
        1,
    );

    expect(true)->toBeTrue();
});

test('c', static   function()use($a):void{
    $this->expectExceptionObject(new RuntimeException('x'));
    $a->run();
    // trailing
});

describe('d', function () {
    test('e', function () {
        $this->expectException(RuntimeException::class);
        doSomething();

        $this->expectException(LogicException::class);
        $x = 1;
    });
});

test('f', function () {
    $this->expectException(RuntimeException::class);
    $this->expectExceptionCode(5);
    doSomething();
});

test('h', function () {
    $this->expectException(RuntimeException::class);
    return doSomething();
});

test('i', function () {
    $this->expectException(...);
    doSomething();
});

test('j', function () {
    $this->expectException(class: RuntimeException::class);
    $this->expectExceptionMessage();
    doSomething();
});
-----
<?php

test('a', function () {
    expect(fn() => doSomething())->toThrow(RuntimeException::class);
});

test('b', function () {
    $service = make();
    // why
    /* msg */
    expect(fn() => $service->run(
        1,
    ))->toThrow(RuntimeException::class, 'boom');

    expect(true)->toBeTrue();
});

test('c', static   function()use($a):void{
    expect(fn() => $a->run())->toThrow(new RuntimeException('x'));
    // trailing
});

describe('d', function () {
    test('e', function () {
        expect(fn() => doSomething())->toThrow(RuntimeException::class);
        $this->expectException(LogicException::class);
        $x = 1;
    });
});

test('f', function () {
    $this->expectException(RuntimeException::class);
    $this->expectExceptionCode(5);
    doSomething();
});

test('h', function () {
    $this->expectException(RuntimeException::class);
    return doSomething();
});

test('i', function () {
    $this->expectException(...);
    doSomething();
});

test('j', function () {
    $this->expectException(class: RuntimeException::class);
    $this->expectExceptionMessage();
    doSomething();
});
