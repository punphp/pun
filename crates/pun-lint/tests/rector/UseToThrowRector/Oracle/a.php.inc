<?php

it('a', function () {
    $a = 1;
    // before try
    try {
        // inside
        foo($a);
    } catch (E $e) {
        expect($e->getMessage())->toBe('m');
    }
});

it('b', function () {
    if ($x) {
        try {
            $b = $a + $c;
            $b->run($_GET, $this, $$v, fn ($p) => $p + $q);
            foreach ($list as $k => $v2) { $v2($k); }
            [$l, $m] = $pair;
            $l($m, $n);
            $n = 2;
            function () use ($z) { return $y; };
        } catch (E) {
        }
    }
});

describe('c', function () {
    beforeEach(function () {
        try {
            foo(); // trailing
        } catch (E $e) {
        }
    });

    try {
        foo();
    } catch (E $e) {
    }
});

TEST('d', function () {
    try {
        foo();
    } catch (A|B $e) {
    }
    try {
    } catch (E $e) {
    }
    try {
        foo();
    } catch (E $e) {
        expect($e->getMessage())->toBe('m', 'x');
    }
    try {
        foo();
    } catch (E $e) {
        expect($other->getMessage())->toBe('m');
    }
    try {
        foo();
    } catch (E $e) {
        expect($e->GETMESSAGE())->TOBE(...$m);
    }
    try {
        foo();
    } catch (E $e) {
        // only a comment
    }
    try {
        try {
            bar();
        } catch (F $f) {
        }
    } catch (E $e) {
    }
    try {
        foo();
    } catch (E $e) {
        try {
            bar();
        } catch (F $f) {
        }
    }
});

it('e', fn () => 1);

it('f', function () {
    try {
        $x->foo(
            1,
        );
    } catch (\Foo\E $e) {
        expect($e->getMessage())->toBe(
            'long'
        );
    }
});
-----
<?php

it('a', function () {
    $a = 1;
    expect(fn() => foo($a))->toThrow(E::class, 'm');
});

it('b', function () {
    if ($x) {
        expect(function () use ($a, $c, $v, $q, $list, $pair, $n, $z, $y) {
            $b = $a + $c;
            $b->run($_GET, $this, $$v, fn ($p) => $p + $q);
            foreach ($list as $k => $v2) { $v2($k); }
            [$l, $m] = $pair;
            $l($m, $n);
            $n = 2;
            function () use ($z) { return $y; };
        })->toThrow(E::class);
    }
});

describe('c', function () {
    beforeEach(function () {
        expect(function () {
            foo();
            // trailing
        })->toThrow(E::class);
    });

    expect(fn() => foo())->toThrow(E::class);
});

TEST('d', function () {
    try {
        foo();
    } catch (A|B $e) {
    }
    try {
    } catch (E $e) {
    }
    try {
        foo();
    } catch (E $e) {
        expect($e->getMessage())->toBe('m', 'x');
    }
    try {
        foo();
    } catch (E $e) {
        expect($other->getMessage())->toBe('m');
    }
    expect(fn() => foo())->toThrow(E::class, $m);
    try {
        foo();
    } catch (E $e) {
        // only a comment
    }
    expect(function () use ($f) {
        try {
            bar();
        } catch (F $f) {
        }
    })->toThrow(E::class);
    try {
        foo();
    } catch (E $e) {
        expect(fn() => bar())->toThrow(F::class);
    }
});

it('e', fn () => 1);

it('f', function () {
    expect(fn() => $x->foo(
        1,
    ))->toThrow(\Foo\E::class, 'long');
});
