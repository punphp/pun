<?php

it('b', function () {
    expect(123)->not->toBeString()->toBeInt();
    expect('12')->toBeNumeric()->toBe('12');
    expect(' 1e3 ')->toBeNumeric()->toBe(' 1e3 ');
    expect('0x1A')->not->toBeNumeric()->toBe('0x1A');
    expect('abc')->toBeNumeric()->toBe('abc');
    expect(1.5)->toBeNumeric()->toBe(1.5);
    expect(fn () => 1)->toBeCallable()->toBeObject();
    expect(function () {})->toBeObject()->toBeCallable();
    expect(new Foo)->toBeObject()->toBeInstanceOf(Foo::class);
    expect(new Foo)->toBeCallable()->toBeInstanceOf(Foo::class);
    expect(TRUE)->toBeBool()->toBeTrue();
    expect(\false)->toBeScalar()->toBeFalse();
    expect(null)->toBeNull()->toBe(null);
    expect(null)->not->toBeNull()->toBe(null);
    expect(NULL)->not->toBeString()->toBe(null);
    expect([1])->toBeIterable()->toHaveCount(1);
    expect(PHP_EOL)->toBeString()->toBe("\n");
    expect(Foo::class)->toBeScalar()->toBe('Foo');
    expect(Foo::CLASS)->toBeString()->toBe('Foo');
    expect('a')->toBeString()->AND('b')->toBe('b');
    expect('a')->toBeString()->each()->toBe('b');
    expect('a')->toBeString()->Json()->toBe('b');
    expect('a')->Not->toBeString()->toBe('b');
    expect('a')->TOBESTRING()->toBe('b');
    EXPECT('a')->toBeString()->toBe('b');
    \expect('a')->toBeString()->toBe('b');
    expect('a')->toBeString()->not->toBe('b');
    expect('a')->toBeString()->each->toBe('b');
    expect('a', 'b')->toBeString()->toBe('a');
    expect(...)->toBeString()->toBe('a');
    expect(value: 'a')->toBeString()->toBe('a');
    expect("1\x32")->toBeNumeric()->toBe('12');
    expect("a$b")->toBeString()->toBe('ab');
    expect(<<<TXT
    12
    TXT)->toBeNumeric()->toBe('12');
    expect(-1)->toBeInt()->toBe(-1);
    expect((1))->toBeInt()->toBe(1);
    (expect(1)->toBeInt())->toBe(1);
    expect(1)->toBeInt()?->toBe(1);
    expect(1)->toBeInt()->toBe(1)->toBe(1);
    expect(1)->toBeInt()->{'toBe'}(1);
});
-----
<?php

it('b', function () {
    expect(123)->toBeInt();
    expect('12')->toBe('12');
    expect(' 1e3 ')->toBe(' 1e3 ');
    expect('0x1A')->toBe('0x1A');
    expect('abc')->toBeNumeric()->toBe('abc');
    expect(1.5)->toBe(1.5);
    expect(fn () => 1)->toBeObject();
    expect(function () {})->toBeCallable();
    expect(new Foo)->toBeInstanceOf(Foo::class);
    expect(new Foo)->toBeCallable()->toBeInstanceOf(Foo::class);
    expect(TRUE)->toBeTrue();
    expect(\false)->toBeFalse();
    expect(null)->toBe(null);
    expect(null)->not->toBeNull()->toBe(null);
    expect(NULL)->toBe(null);
    expect([1])->toHaveCount(1);
    expect(PHP_EOL)->toBeString()->toBe("\n");
    expect(Foo::class)->toBe('Foo');
    expect(Foo::CLASS)->toBeString()->toBe('Foo');
    expect('a')->AND('b')->toBe('b');
    expect('a')->toBeString()->each()->toBe('b');
    expect('a')->Json()->toBe('b');
    expect('a')->Not->toBeString()->toBe('b');
    expect('a')->TOBESTRING()->toBe('b');
    EXPECT('a')->toBeString()->toBe('b');
    \expect('a')->toBe('b');
    expect('a')->toBeString()->not->toBe('b');
    expect('a')->toBeString()->each->toBe('b');
    expect('a', 'b')->toBe('a');
    expect(...)->toBeString()->toBe('a');
    expect(value: 'a')->toBe('a');
    expect("1\x32")->toBe('12');
    expect("a$b")->toBeString()->toBe('ab');
    expect(<<<TXT
    12
    TXT)->toBe('12');
    expect(-1)->toBeInt()->toBe(-1);
    expect((1))->toBe(1);
    (expect(1))->toBe(1);
    expect(1)->toBeInt()?->toBe(1);
    expect(1)->toBe(1)->toBe(1);
    expect(1)->toBeInt()->{'toBe'}(1);
});
