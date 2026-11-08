<?php

test('a', function (?string $good, null $bad, $untyped) {
    expect(is_file($good))->toBeTrue();
    expect(is_file($bad))->toBeTrue();
    expect(is_file($untyped))->toBeFalse();
    expect(is_file($undefined))->not->toBeFalse();
    expect(is_file($untyped))
        ->toBeTrue();
    expect(is_file($untyped)) // why
        ->not->toBeTrue();
    expect($a)->toBeInt()->and(is_file($untyped))->toBeTrue();
    expect($a)->toBeInt()->and(is_file($untyped))->toBeTrue()->and($b)->toBe(1);
    expect(is_file($untyped))->toBeTrue()->toBeBool();
    expect(is_file($untyped))->toBeTrue('message');
    expect(value: is_file($untyped))->toBeTrue();
    expect(is_file($untyped), 'extra')->toBeTrue();
    \expect(\is_file($untyped))->toBeTrue();
    EXPECT(is_file($untyped))->ToBeTrue();
    expect(IS_FILE($untyped))->toBeTrue();
    expect((is_file(($untyped))))->toBeTrue();
    expect(is_file($untyped))->toBeTruthy();
    expect(is_file($untyped))->each->toBeTrue();
    $x->expect(is_file($untyped))->toBeTrue();
    expect(is_file($untyped))?->toBeTrue();
    expect(is_file('abc'))->toBeTrue();
    expect(is_file([1]))->toBeTrue();
    expect(is_file($this->value))->toBeTrue();
    expect(is_file(strtolower($untyped)))->toBeTrue();
    expect(is_file(...$args))->toBeTrue();
    expect(is_file())->toBeTrue();
    expect(is_file($a, $b, $c))->toBeTrue();
});
-----
<?php

test('a', function (?string $good, null $bad, $untyped) {
    expect(is_file($good))->toBeTrue();
    expect(is_file($bad))->toBeTrue();
    expect(is_file($untyped))->toBeFalse();
    expect(is_file($undefined))->not->toBeFalse();
    expect(is_file($untyped))
        ->toBeTrue();
    expect(is_file($untyped)) // why
        ->not->toBeTrue();
    expect($a)->toBeInt()->and(is_file($untyped))->toBeTrue();
    expect($a)->toBeInt()->and(is_file($untyped))->toBeTrue()->and($b)->toBe(1);
    expect(is_file($untyped))->toBeTrue()->toBeBool();
    expect(is_file($untyped))->toBeTrue('message');
    expect(value: is_file($untyped))->toBeTrue();
    expect(is_file($untyped), 'extra')->toBeTrue();
    \expect(\is_file($untyped))->toBeTrue();
    EXPECT(is_file($untyped))->ToBeTrue();
    expect(IS_FILE($untyped))->toBeTrue();
    expect((is_file(($untyped))))->toBeTrue();
    expect(is_file($untyped))->toBeTruthy();
    expect(is_file($untyped))->each->toBeTrue();
    $x->expect(is_file($untyped))->toBeTrue();
    expect(is_file($untyped))?->toBeTrue();
    expect('abc')->toBeFile();
    expect(is_file([1]))->toBeTrue();
    expect(is_file($this->value))->toBeTrue();
    expect(strtolower($untyped))->toBeFile();
    expect(is_file(...$args))->toBeTrue();
    expect(is_file())->toBeTrue();
    expect(is_file($a, $b, $c))->toBeTrue();
});
