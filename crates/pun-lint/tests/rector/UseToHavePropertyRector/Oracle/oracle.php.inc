<?php

test('a', function (object $good, int $bad, $untyped) {
    expect(property_exists($good, 'x'))->toBeTrue();
    expect(property_exists($bad, 'x'))->toBeTrue();
    expect(property_exists($untyped, 'x'))->toBeFalse();
    expect(property_exists($undefined, 'x'))->not->toBeFalse();
    expect(property_exists($untyped, 'x'))
        ->toBeTrue();
    expect(property_exists($untyped, 'x')) // why
        ->not->toBeTrue();
    expect($a)->toBeInt()->and(property_exists($untyped, 'x'))->toBeTrue();
    expect($a)->toBeInt()->and(property_exists($untyped, 'x'))->toBeTrue()->and($b)->toBe(1);
    expect(property_exists($untyped, 'x'))->toBeTrue()->toBeBool();
    expect(property_exists($untyped, 'x'))->toBeTrue('message');
    expect(value: property_exists($untyped, 'x'))->toBeTrue();
    expect(property_exists($untyped, 'x'), 'extra')->toBeTrue();
    \expect(\property_exists($untyped, 'x'))->toBeTrue();
    EXPECT(property_exists($untyped, 'x'))->ToBeTrue();
    expect(PROPERTY_EXISTS($untyped, 'x'))->toBeTrue();
    expect((property_exists(($untyped), 'x')))->toBeTrue();
    expect(property_exists($untyped, 'x'))->toBeTruthy();
    expect(property_exists($untyped, 'x'))->each->toBeTrue();
    $x->expect(property_exists($untyped, 'x'))->toBeTrue();
    expect(property_exists($untyped, 'x'))?->toBeTrue();
    expect(property_exists('abc', 'x'))->toBeTrue();
    expect(property_exists([1], 'x'))->toBeTrue();
    expect(property_exists($this->value, 'x'))->toBeTrue();
    expect(property_exists(strtolower($untyped), 'x'))->toBeTrue();
    expect(property_exists())->toBeTrue();
    expect(property_exists($a, $b, $c))->toBeTrue();
    expect(property_exists($untyped, needle: $n))->toBeTrue();
    expect(property_exists($untyped, ...$n))->toBeTrue();
});
-----
<?php

test('a', function (object $good, int $bad, $untyped) {
    expect($good)->toHaveProperty('x');
    expect(property_exists($bad, 'x'))->toBeTrue();
    expect(property_exists($untyped, 'x'))->toBeFalse();
    expect(property_exists($undefined, 'x'))->not->toBeFalse();
    expect(property_exists($untyped, 'x'))
        ->toBeTrue();
    expect(property_exists($untyped, 'x')) // why
        ->not->toBeTrue();
    expect($a)->toBeInt()->and(property_exists($untyped, 'x'))->toBeTrue();
    expect($a)->toBeInt()->and(property_exists($untyped, 'x'))->toBeTrue()->and($b)->toBe(1);
    expect(property_exists($untyped, 'x'))->toBeTrue()->toBeBool();
    expect(property_exists($untyped, 'x'))->toBeTrue('message');
    expect(value: property_exists($untyped, 'x'))->toBeTrue();
    expect(property_exists($untyped, 'x'), 'extra')->toBeTrue();
    \expect(\property_exists($untyped, 'x'))->toBeTrue();
    EXPECT(property_exists($untyped, 'x'))->ToBeTrue();
    expect(PROPERTY_EXISTS($untyped, 'x'))->toBeTrue();
    expect((property_exists(($untyped), 'x')))->toBeTrue();
    expect(property_exists($untyped, 'x'))->toBeTruthy();
    expect(property_exists($untyped, 'x'))->each->toBeTrue();
    $x->expect(property_exists($untyped, 'x'))->toBeTrue();
    expect(property_exists($untyped, 'x'))?->toBeTrue();
    expect(property_exists('abc', 'x'))->toBeTrue();
    expect(property_exists([1], 'x'))->toBeTrue();
    expect(property_exists($this->value, 'x'))->toBeTrue();
    expect(property_exists(strtolower($untyped), 'x'))->toBeTrue();
    expect(property_exists())->toBeTrue();
    expect(property_exists($a, $b, $c))->toBeTrue();
    expect(property_exists($untyped, needle: $n))->toBeTrue();
    expect(property_exists($untyped, ...$n))->toBeTrue();
});
