<?php

test('a', function (float $good, string $bad, $untyped) {
    expect(is_infinite($good))->toBeTrue();
    expect(is_infinite($bad))->toBeTrue();
    expect(is_infinite($untyped))->toBeFalse();
    expect(is_infinite($undefined))->not->toBeFalse();
    expect(is_infinite($untyped))
        ->toBeTrue();
    expect(is_infinite($untyped)) // why
        ->not->toBeTrue();
    expect($a)->toBeInt()->and(is_infinite($untyped))->toBeTrue();
    expect($a)->toBeInt()->and(is_infinite($untyped))->toBeTrue()->and($b)->toBe(1);
    expect(is_infinite($untyped))->toBeTrue()->toBeBool();
    expect(is_infinite($untyped))->toBeTrue('message');
    expect(value: is_infinite($untyped))->toBeTrue();
    expect(is_infinite($untyped), 'extra')->toBeTrue();
    \expect(\is_infinite($untyped))->toBeTrue();
    EXPECT(is_infinite($untyped))->ToBeTrue();
    expect(IS_INFINITE($untyped))->toBeTrue();
    expect((is_infinite(($untyped))))->toBeTrue();
    expect(is_infinite($untyped))->toBeTruthy();
    expect(is_infinite($untyped))->each->toBeTrue();
    $x->expect(is_infinite($untyped))->toBeTrue();
    expect(is_infinite($untyped))?->toBeTrue();
    expect(is_infinite('abc'))->toBeTrue();
    expect(is_infinite([1]))->toBeTrue();
    expect(is_infinite($this->value))->toBeTrue();
    expect(is_infinite(strtolower($untyped)))->toBeTrue();
    expect(is_infinite(...$args))->toBeTrue();
    expect(is_infinite())->toBeTrue();
    expect(is_infinite($a, $b, $c))->toBeTrue();
});
-----
<?php

test('a', function (float $good, string $bad, $untyped) {
    expect($good)->toBeInfinite();
    expect(is_infinite($bad))->toBeTrue();
    expect($untyped)->not->toBeInfinite();
    expect($undefined)->toBeInfinite();
    expect($untyped)->toBeInfinite();
    expect($untyped)->not->toBeInfinite();
    expect($a)->toBeInt()->and($untyped)->toBeInfinite();
    expect($a)->toBeInt()->and($untyped)->toBeInfinite()->and($b)->toBe(1);
    expect($untyped)->toBeInfinite()->toBeBool();
    expect($untyped)->toBeInfinite();
    expect($untyped)->toBeInfinite();
    expect($untyped, 'extra')->toBeInfinite();
    \expect($untyped)->toBeInfinite();
    EXPECT(is_infinite($untyped))->ToBeTrue();
    expect(IS_INFINITE($untyped))->toBeTrue();
    expect($untyped)->toBeInfinite();
    expect(is_infinite($untyped))->toBeTruthy();
    expect(is_infinite($untyped))->each->toBeTrue();
    $x->expect(is_infinite($untyped))->toBeTrue();
    expect(is_infinite($untyped))?->toBeTrue();
    expect(is_infinite('abc'))->toBeTrue();
    expect(is_infinite([1]))->toBeTrue();
    expect($this->value)->toBeInfinite();
    expect(is_infinite(strtolower($untyped)))->toBeTrue();
    expect($args)->toBeInfinite();
    expect(is_infinite())->toBeTrue();
    expect(is_infinite($a, $b, $c))->toBeTrue();
});
