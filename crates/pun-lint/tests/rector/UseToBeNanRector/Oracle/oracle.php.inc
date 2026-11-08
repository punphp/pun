<?php

test('a', function (int $good, string $bad, $untyped) {
    expect(is_nan($good))->toBeTrue();
    expect(is_nan($bad))->toBeTrue();
    expect(is_nan($untyped))->toBeFalse();
    expect(is_nan($undefined))->not->toBeFalse();
    expect(is_nan($untyped))
        ->toBeTrue();
    expect(is_nan($untyped)) // why
        ->not->toBeTrue();
    expect($a)->toBeInt()->and(is_nan($untyped))->toBeTrue();
    expect($a)->toBeInt()->and(is_nan($untyped))->toBeTrue()->and($b)->toBe(1);
    expect(is_nan($untyped))->toBeTrue()->toBeBool();
    expect(is_nan($untyped))->toBeTrue('message');
    expect(value: is_nan($untyped))->toBeTrue();
    expect(is_nan($untyped), 'extra')->toBeTrue();
    \expect(\is_nan($untyped))->toBeTrue();
    EXPECT(is_nan($untyped))->ToBeTrue();
    expect(IS_NAN($untyped))->toBeTrue();
    expect((is_nan(($untyped))))->toBeTrue();
    expect(is_nan($untyped))->toBeTruthy();
    expect(is_nan($untyped))->each->toBeTrue();
    $x->expect(is_nan($untyped))->toBeTrue();
    expect(is_nan($untyped))?->toBeTrue();
    expect(is_nan('abc'))->toBeTrue();
    expect(is_nan([1]))->toBeTrue();
    expect(is_nan($this->value))->toBeTrue();
    expect(is_nan(strtolower($untyped)))->toBeTrue();
    expect(is_nan(...$args))->toBeTrue();
    expect(is_nan())->toBeTrue();
    expect(is_nan($a, $b, $c))->toBeTrue();
});
-----
<?php

test('a', function (int $good, string $bad, $untyped) {
    expect($good)->toBeNan();
    expect(is_nan($bad))->toBeTrue();
    expect(is_nan($untyped))->toBeFalse();
    expect(is_nan($undefined))->not->toBeFalse();
    expect(is_nan($untyped))
        ->toBeTrue();
    expect(is_nan($untyped)) // why
        ->not->toBeTrue();
    expect($a)->toBeInt()->and(is_nan($untyped))->toBeTrue();
    expect($a)->toBeInt()->and(is_nan($untyped))->toBeTrue()->and($b)->toBe(1);
    expect(is_nan($untyped))->toBeTrue()->toBeBool();
    expect(is_nan($untyped))->toBeTrue('message');
    expect(value: is_nan($untyped))->toBeTrue();
    expect(is_nan($untyped), 'extra')->toBeTrue();
    \expect(\is_nan($untyped))->toBeTrue();
    EXPECT(is_nan($untyped))->ToBeTrue();
    expect(IS_NAN($untyped))->toBeTrue();
    expect((is_nan(($untyped))))->toBeTrue();
    expect(is_nan($untyped))->toBeTruthy();
    expect(is_nan($untyped))->each->toBeTrue();
    $x->expect(is_nan($untyped))->toBeTrue();
    expect(is_nan($untyped))?->toBeTrue();
    expect(is_nan('abc'))->toBeTrue();
    expect(is_nan([1]))->toBeTrue();
    expect(is_nan($this->value))->toBeTrue();
    expect(is_nan(strtolower($untyped)))->toBeTrue();
    expect(is_nan(...$args))->toBeTrue();
    expect(is_nan())->toBeTrue();
    expect(is_nan($a, $b, $c))->toBeTrue();
});
