<?php

it('a', function () {
    try {
        foo();
    } catch (E $e) {
        expect($e->getCode())->toBe('m');
    }
});

it('b', function () {
    try {
        foo();
    } catch (E $e) {
        expect($this->e->getMessage())->toBe('m');
    }
});

it('c', function () {
    try {
        [, $a] = f();
        list($b) = g();
        ['k' => $c] = h();
        [&$r] = $refs;
        foreach ($d as &$v) {
            bar($v, $q, $a, $b, $c, $r);
        }
        foreach ($d as $w) {
            baz($w, $z);
        }
    } catch (E $e) {
        expect($e->getMessage())->toBe('m');
    }
});
-----
<?php

it('a', function () {
    try {
        foo();
    } catch (E $e) {
        expect($e->getCode())->toBe('m');
    }
});

it('b', function () {
    try {
        foo();
    } catch (E $e) {
        expect($this->e->getMessage())->toBe('m');
    }
});

it('c', function () {
    expect(function () use ($refs, $d, $q, $z) {
        [, $a] = f();
        list($b) = g();
        ['k' => $c] = h();
        [&$r] = $refs;
        foreach ($d as &$v) {
            bar($v, $q, $a, $b, $c, $r);
        }
        foreach ($d as $w) {
            baz($w, $z);
        }
    })->toThrow(E::class, 'm');
});
