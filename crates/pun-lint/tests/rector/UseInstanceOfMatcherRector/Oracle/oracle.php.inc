<?php

namespace App;

use Foo\Bar;

test('a', function ($x) {
    expect($x instanceof Bar)->toBeTrue();
    expect($x instanceof \DateTime)->toBeTrue();
    expect($x instanceof Baz\Qux)->toBeTrue();
    expect($x instanceof self)->toBeTrue();
    expect($x instanceof static)->toBeTrue();
    expect($x instanceof parent)->toBeTrue();
    expect($x instanceof $class)->toBeTrue();
    expect($x instanceof Bar)->not->toBeTrue();
    expect($x instanceof Bar)->toBeTrue('message');
    expect($x instanceof Bar, 'extra')->toBeTrue();
    expect($x instanceof Bar)->toBeObject()->toBeTrue();
    expect(new Foo() instanceof Bar)->toBeTrue();
    expect(!$x instanceof Bar)->toBeTrue();
    expect($x instanceof Bar)->TOBETRUE();
    expect($x instanceof Bar)
        ->toBeTrue();
});
-----
<?php

namespace App;

use Foo\Bar;

test('a', function ($x) {
    expect($x)->toBeInstanceOf(Bar::class);
    expect($x)->toBeInstanceOf(\DateTime::class);
    expect($x)->toBeInstanceOf(Baz\Qux::class);
    expect($x)->toBeInstanceOf(self::class);
    expect($x)->toBeInstanceOf(static::class);
    expect($x)->toBeInstanceOf(parent::class);
    expect($x instanceof $class)->toBeTrue();
    expect($x)->not->toBeInstanceOf(Bar::class);
    expect($x)->toBeInstanceOf(Bar::class);
    expect($x)->toBeInstanceOf(Bar::class);
    expect($x)->toBeObject()->toBeInstanceOf(Bar::class);
    expect(new Foo())->toBeInstanceOf(Bar::class);
    expect(!$x instanceof Bar)->toBeTrue();
    expect($x)->toBeInstanceOf(Bar::class);
    expect($x)
        ->toBeInstanceOf(Bar::class);
});
