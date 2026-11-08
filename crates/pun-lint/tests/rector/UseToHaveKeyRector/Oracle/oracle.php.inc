<?php

test('a', function (array $good, string $bad, $untyped) {
    expect(array_key_exists('x', $good))->toBeTrue();
    expect(array_key_exists('x', $bad))->toBeTrue();
    expect(array_key_exists('x', $untyped))->toBeFalse();
    expect(array_key_exists('x', $undefined))->not->toBeFalse();
    expect(array_key_exists('x', $untyped))
        ->toBeTrue();
    expect(array_key_exists('x', $untyped)) // why
        ->not->toBeTrue();
    expect($a)->toBeInt()->and(array_key_exists('x', $untyped))->toBeTrue();
    expect($a)->toBeInt()->and(array_key_exists('x', $untyped))->toBeTrue()->and($b)->toBe(1);
    expect(array_key_exists('x', $untyped))->toBeTrue()->toBeBool();
    expect(array_key_exists('x', $untyped))->toBeTrue('message');
    expect(value: array_key_exists('x', $untyped))->toBeTrue();
    expect(array_key_exists('x', $untyped), 'extra')->toBeTrue();
    \expect(\array_key_exists('x', $untyped))->toBeTrue();
    EXPECT(array_key_exists('x', $untyped))->ToBeTrue();
    expect(ARRAY_KEY_EXISTS('x', $untyped))->toBeTrue();
    expect((array_key_exists('x', ($untyped))))->toBeTrue();
    expect(array_key_exists('x', $untyped))->toBeTruthy();
    expect(array_key_exists('x', $untyped))->each->toBeTrue();
    $x->expect(array_key_exists('x', $untyped))->toBeTrue();
    expect(array_key_exists('x', $untyped))?->toBeTrue();
    expect(array_key_exists('x', 'abc'))->toBeTrue();
    expect(array_key_exists('x', [1]))->toBeTrue();
    expect(array_key_exists('x', $this->value))->toBeTrue();
    expect(array_key_exists('x', strtolower($untyped)))->toBeTrue();
    expect(array_key_exists('x', ...$args))->toBeTrue();
    expect(array_key_exists())->toBeTrue();
    expect(array_key_exists($a, $b, $c))->toBeTrue();
});
-----
<?php

test('a', function (array $good, string $bad, $untyped) {
    expect($good)->toHaveKey('x');
    expect(array_key_exists('x', $bad))->toBeTrue();
    expect(array_key_exists('x', $untyped))->toBeFalse();
    expect(array_key_exists('x', $undefined))->not->toBeFalse();
    expect(array_key_exists('x', $untyped))
        ->toBeTrue();
    expect(array_key_exists('x', $untyped)) // why
        ->not->toBeTrue();
    expect($a)->toBeInt()->and(array_key_exists('x', $untyped))->toBeTrue();
    expect($a)->toBeInt()->and(array_key_exists('x', $untyped))->toBeTrue()->and($b)->toBe(1);
    expect(array_key_exists('x', $untyped))->toBeTrue()->toBeBool();
    expect(array_key_exists('x', $untyped))->toBeTrue('message');
    expect(value: array_key_exists('x', $untyped))->toBeTrue();
    expect(array_key_exists('x', $untyped), 'extra')->toBeTrue();
    \expect(\array_key_exists('x', $untyped))->toBeTrue();
    EXPECT(array_key_exists('x', $untyped))->ToBeTrue();
    expect(ARRAY_KEY_EXISTS('x', $untyped))->toBeTrue();
    expect((array_key_exists('x', ($untyped))))->toBeTrue();
    expect(array_key_exists('x', $untyped))->toBeTruthy();
    expect(array_key_exists('x', $untyped))->each->toBeTrue();
    $x->expect(array_key_exists('x', $untyped))->toBeTrue();
    expect(array_key_exists('x', $untyped))?->toBeTrue();
    expect(array_key_exists('x', 'abc'))->toBeTrue();
    expect([1])->toHaveKey('x');
    expect(array_key_exists('x', $this->value))->toBeTrue();
    expect(array_key_exists('x', strtolower($untyped)))->toBeTrue();
    expect(array_key_exists('x', ...$args))->toBeTrue();
    expect(array_key_exists())->toBeTrue();
    expect(array_key_exists($a, $b, $c))->toBeTrue();
});
