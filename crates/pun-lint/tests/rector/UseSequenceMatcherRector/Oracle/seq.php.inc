<?php

test('a', function () {
    $items = [1, 2];

    expect($items[0])->toBe(1);
    // two
    expect($items[1])->not->toBe(3)->toBeInt();
    expect($items[2])->each->toBeInt();
    expect($items[3])->toBeInt()->not->toBe(5);
    expect($items[4], 'm')->toBe(
        5,
    );
    expect($items[6])->toBe(6);
});

test('b', static   function()use($a):void{
    expect($a[0])->toBe(1);
    expect($a[1])->toBe(2);
});

function foo() {
    expect($a[0])->toBe(1);
    expect($a[1])->toBe(2);
}

class T {
    public function x() {
        expect($a[0])->toBe(1);
        expect($a[1])->toBe(2);
    }
}

test('c', function () {
    expect($a[1])->toBe(1);
    expect($a[0x0])->toBe(1);
    expect($a[01])->toBe(2);
    expect($b[0])->toBe(1);
    expect($c[0])->toBe(1);
});
-----
<?php

test('a', function () {
    $items = [1, 2];
    // two
    expect($items)->sequence(fn($e) => $e->toBe(1), fn($e) => $e->not->toBe(3)->toBeInt(), fn($e) => $e->not->toBeInt(), fn($e) => $e->not->toBe(5), fn($e) => $e->toBe(5));
    expect($items[6])->toBe(6);
});

test('b', static function () use ($a): void {
    expect($a)->sequence(fn($e) => $e->toBe(1), fn($e) => $e->toBe(2));
});

function foo()
{
    expect($a)->sequence(fn($e) => $e->toBe(1), fn($e) => $e->toBe(2));
}

class T {
    public function x()
    {
        expect($a)->sequence(fn($e) => $e->toBe(1), fn($e) => $e->toBe(2));
    }
}

test('c', function () {
    expect($a[1])->toBe(1);
    expect($a)->sequence(fn($e) => $e->toBe(1), fn($e) => $e->toBe(2));
    expect($b[0])->toBe(1);
    expect($c[0])->toBe(1);
});
