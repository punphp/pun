<?php

expect(new Foo())->toHaveMethod('a');
expect($a ?? $b)->toHaveMethod('a');
expect(clone $x)->toHaveMethod('a');
expect(Foo::bar())->toHaveMethod('a');
expect(Foo::$bar)->toHaveMethod('a');
expect($a[0])->toHaveMethod('a');
expect(foo(...))->toHaveMethod('a');
expect(FOO)->toHaveMethod('a');
expect(Foo::BAR)->toHaveMethod('a');
expect(Foo::class)->toHaveMethod('a');
expect('Foo')->toHaveMethod('a');
expect("Foo")->toHaveMethod('a');
expect("Foo$x")->toHaveMethod('a');
expect([1])->toHaveMethod('a');
expect(match ($a) { default => 1 })->toHaveMethod('a');
expect($a?->b)->toHaveMethod('a');
expect($a?->b())->toHaveMethod('a');
expect(fn () => 1)->toHaveMethod('a');
expect(1)->toHaveMethod('a');
expect(__CLASS__)->toHaveMethod('a');
expect(value: $obj)->toHaveMethods(['a']);
expect(...$objs)->toHaveMethods(['a']);
expect($a)->not->toHaveMethod('a');
expect($a)->and($b)->toHaveMethod('a');
expect(($a))->toHaveMethod('a');
expect(@$a)->toHaveMethod('a');
expect(!$a)->toHaveMethod('a');
expect(Foo::{$a})->toHaveMethod('a');
expect($a::$b)->toHaveMethod('a');
expect(static::class)->toHaveMethod('a');
expect(new class {})->toHaveMethod('a');
expect(PHP_EOL)->toHaveMethod('a');
expect($a)->TOHAVEMETHOD('a');
expect($a)
    ->toHaveMethods(['a']);
expect($a)->toBeObject()->toHaveMethod('a');
test('t', function (string $s, object $o, $untyped) {
    expect($s)->toHaveMethod('a');
    expect($o)->toHaveMethod('a');
    expect($untyped)->toHaveMethod('a');
    expect(__CLASS__)->toHaveMethod('a');
    expect(<<<EOT
    Foo
    EOT)->toHaveMethod('a');
    expect($s . 'x')->toHaveMethod('a');
    expect(get_class($o))->toHaveMethod('a');
});
-----
<?php

expect((new Foo())::class)->toHaveMethod('a');
expect(($a ?? $b)::class)->toHaveMethod('a');
expect((clone $x)::class)->toHaveMethod('a');
expect(Foo::bar()::class)->toHaveMethod('a');
expect(Foo::$bar::class)->toHaveMethod('a');
expect($a[0]::class)->toHaveMethod('a');
expect(foo(...)::class)->toHaveMethod('a');
expect((FOO)::class)->toHaveMethod('a');
expect(Foo::BAR)->toHaveMethod('a');
expect(Foo::class)->toHaveMethod('a');
expect('Foo')->toHaveMethod('a');
expect("Foo")->toHaveMethod('a');
expect("Foo$x")->toHaveMethod('a');
expect([1]::class)->toHaveMethod('a');
expect((match ($a) { default => 1 })::class)->toHaveMethod('a');
expect($a?->b::class)->toHaveMethod('a');
expect($a?->b()::class)->toHaveMethod('a');
expect((fn () => 1)::class)->toHaveMethod('a');
expect((1)::class)->toHaveMethod('a');
expect(__CLASS__)->toHaveMethod('a');
expect($obj::class)->toHaveMethods(['a']);
expect($objs::class)->toHaveMethods(['a']);
expect($a::class)->not->toHaveMethod('a');
expect($a::class)->and($b)->toHaveMethod('a');
expect($a::class)->toHaveMethod('a');
expect((@$a)::class)->toHaveMethod('a');
expect((!$a)::class)->toHaveMethod('a');
expect(Foo::{$a})->toHaveMethod('a');
expect($a::$b::class)->toHaveMethod('a');
expect(static::class)->toHaveMethod('a');
expect((new class {})::class)->toHaveMethod('a');
expect(PHP_EOL)->toHaveMethod('a');
expect($a::class)->TOHAVEMETHOD('a');
expect($a::class)
    ->toHaveMethods(['a']);
expect($a::class)->toBeObject()->toHaveMethod('a');
test('t', function (string $s, object $o, $untyped) {
    expect($s)->toHaveMethod('a');
    expect($o::class)->toHaveMethod('a');
    expect($untyped::class)->toHaveMethod('a');
    expect(__CLASS__)->toHaveMethod('a');
    expect(<<<EOT
    Foo
    EOT)->toHaveMethod('a');
    expect($s . 'x')->toHaveMethod('a');
    expect(get_class($o))->toHaveMethod('a');
});
