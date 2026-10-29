<?php

test('the value of and() is printed anew', function () {
    expect(new Tom($question)->messages())->toBe([1]);
    expect(new Tom($question)->instructions())->toContain('@bob')
        ->not->toContain('@alice');
    expect($a)->toBe(1);
    expect(foo( $a,$b ))->toBe(2);
    expect($a)->toBe(1);
    expect($x->m( 1,
        2 ))->toBe(2);
    expect($a)->toBe(1);
    expect($a  ->b( 1 )
        ->c( 2 ))->toBe(2);
    expect($a)->toBe(1);
    expect([1,  2])->toBe(2);
    expect($a)->toBe(1);
    expect([
        'a' => 1,
        'b' => 2,
    ])->toBe(2);
    expect($a)->toBe(1);
    expect([
        // first
        'a'=>1,
    ])->toBe(2);
    expect($a)->toBe(1);
    expect(array( 1 ))->toBe(2);
    expect($a)->toBe(1);
    expect(array(
        // first
        1
    ))->toBe(2);
    expect($a)->toBe(1);
    expect(new Foo)->toBe(2);
    expect($a)->toBe(1);
    expect(new $cls( 1 ))->toBe(2);
    expect($a)->toBe(1);
    expect(new ($cls))->toBe(2);
    expect($a)->toBe(1);
    expect(new $a->b['c'])->toBe(2);
    expect($a)->toBe(1);
    expect(new $a?->b)->toBe(2);
    expect($a)->toBe(1);
    expect(new (Foo::$cls))->toBe(2);
    expect($a)->toBe(1);
    expect(new (foo()))->toBe(2);
    expect($a)->toBe(1);
    expect($a [ 'x' ])->toBe(2);
    expect($a)->toBe(1);
    expect(FOO['x'])->toBe(2);
    expect($a)->toBe(1);
    expect(('x' . $y)[0])->toBe(2);
    expect($a)->toBe(1);
    expect($a ->  b)->toBe(2);
    expect($a)->toBe(1);
    expect($a?->b( 1,2 ))->toBe(2);
    expect($a)->toBe(1);
    expect($a ?-> b)->toBe(2);
    expect($a)->toBe(1);
    expect($a->$m( 1 ))->toBe(2);
    expect($a)->toBe(1);
    expect((new Foo)->bar)->toBe(2);
    expect($a)->toBe(1);
    expect(new Foo(1, 2)->bar( 3 ))->toBe(2);
    expect($a)->toBe(1);
    expect(Foo :: bar( 1 ))->toBe(2);
    expect($a)->toBe(1);
    expect(Foo::$m( 1 ))->toBe(2);
    expect($a)->toBe(1);
    expect(Foo::{'m'}( 1 ))->toBe(2);
    expect($a)->toBe(1);
    expect('Foo'::bar())->toBe(2);
    expect($a)->toBe(1);
    expect(FOO::bar())->toBe(2);
    expect($a)->toBe(1);
    expect(($a . $b)::bar())->toBe(2);
    expect($a)->toBe(1);
    expect(Foo :: $bar)->toBe(2);
    expect($a)->toBe(1);
    expect(Foo :: class)->toBe(2);
    expect($a)->toBe(1);
    expect(Foo::{'BAR'})->toBe(2);
    expect($a)->toBe(1);
    expect(static::BAR)->toBe(2);
    expect($a)->toBe(1);
    expect(FOO->bar)->toBe(2);
    expect($a)->toBe(1);
    expect($f(  1 ))->toBe(2);
    expect($a)->toBe(1);
    expect((function () {})( 1 ))->toBe(2);
    expect($a)->toBe(1);
    expect(foo(
        // c
        $a,
    ))->toBe(2);
    expect($a)->toBe(1);
    expect(foo(function () {
        return 1;
    }))->toBe(2);
    expect($a)->toBe(1);
    expect(($b))->toBe(2);
});
-----
<?php

test('the value of and() is printed anew', function () {
    expect(new Tom($question)->messages())->toBe([1])
        ->and((new Tom($question))->instructions())->toContain('@bob')->not->toContain('@alice')
        ->and($a)->toBe(1)
        ->and(foo($a, $b))->toBe(2)
        ->and($a)->toBe(1)
        ->and($x->m(1, 2))->toBe(2)
        ->and($a)->toBe(1)
        ->and($a  ->b( 1 )->c(2))->toBe(2)
        ->and($a)->toBe(1)
        ->and([1, 2])->toBe(2)
        ->and($a)->toBe(1)
        ->and(['a' => 1, 'b' => 2])->toBe(2)
        ->and($a)->toBe(1)
        ->and([
        // first
        'a'=>1,
    ])->toBe(2)
        ->and($a)->toBe(1)
        ->and(array(1))->toBe(2)
        ->and($a)->toBe(1)
        ->and(array(
        // first
        1,
    ))->toBe(2)
        ->and($a)->toBe(1)
        ->and(new Foo())->toBe(2)
        ->and($a)->toBe(1)
        ->and(new $cls(1))->toBe(2)
        ->and($a)->toBe(1)
        ->and(new $cls())->toBe(2)
        ->and($a)->toBe(1)
        ->and(new $a->b['c']())->toBe(2)
        ->and($a)->toBe(1)
        ->and(new $a?->b())->toBe(2)
        ->and($a)->toBe(1)
        ->and(new Foo::$cls())->toBe(2)
        ->and($a)->toBe(1)
        ->and(new (foo())())->toBe(2)
        ->and($a)->toBe(1)
        ->and($a['x'])->toBe(2)
        ->and($a)->toBe(1)
        ->and(FOO['x'])->toBe(2)
        ->and($a)->toBe(1)
        ->and(('x' . $y)[0])->toBe(2)
        ->and($a)->toBe(1)
        ->and($a->b)->toBe(2)
        ->and($a)->toBe(1)
        ->and($a?->b(1, 2))->toBe(2)
        ->and($a)->toBe(1)
        ->and($a?->b)->toBe(2)
        ->and($a)->toBe(1)
        ->and($a->{$m}(1))->toBe(2)
        ->and($a)->toBe(1)
        ->and((new Foo)->bar)->toBe(2)
        ->and($a)->toBe(1)
        ->and((new Foo(1, 2))->bar(3))->toBe(2)
        ->and($a)->toBe(1)
        ->and(Foo::bar(1))->toBe(2)
        ->and($a)->toBe(1)
        ->and(Foo::$m(1))->toBe(2)
        ->and($a)->toBe(1)
        ->and(Foo::{'m'}(1))->toBe(2)
        ->and($a)->toBe(1)
        ->and('Foo'::bar())->toBe(2)
        ->and($a)->toBe(1)
        ->and(FOO::bar())->toBe(2)
        ->and($a)->toBe(1)
        ->and(($a . $b)::bar())->toBe(2)
        ->and($a)->toBe(1)
        ->and(Foo::$bar)->toBe(2)
        ->and($a)->toBe(1)
        ->and(Foo::class)->toBe(2)
        ->and($a)->toBe(1)
        ->and(Foo::{'BAR'})->toBe(2)
        ->and($a)->toBe(1)
        ->and(static::BAR)->toBe(2)
        ->and($a)->toBe(1)
        ->and(FOO->bar)->toBe(2)
        ->and($a)->toBe(1)
        ->and($f(1))->toBe(2)
        ->and($a)->toBe(1)
        ->and((function () {})(1))->toBe(2)
        ->and($a)->toBe(1)
        ->and(foo(
        // c
        $a
    ))->toBe(2)
        ->and($a)->toBe(1)
        ->and(foo(function () {
        return 1;
    }))->toBe(2)
        ->and($a)->toBe(1)
        ->and($b)->toBe(2);
});
