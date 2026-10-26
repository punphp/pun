<?php

test('a', function (array $good, string $bad, $untyped) {
    expect(array_is_list($good))->toBeTrue();
    expect(array_is_list($bad))->toBeTrue();
    expect(array_is_list($untyped))->toBeFalse();
    expect(array_is_list($undefined))->not->toBeFalse();
    expect(array_is_list($untyped))
        ->toBeTrue();
    expect(array_is_list($untyped)) // why
        ->not->toBeTrue();
    expect($a)->toBeInt()->and(array_is_list($untyped))->toBeTrue();
    expect($a)->toBeInt()->and(array_is_list($untyped))->toBeTrue()->and($b)->toBe(1);
    expect(array_is_list($untyped))->toBeTrue()->toBeBool();
    expect(array_is_list($untyped))->toBeTrue('message');
    expect(value: array_is_list($untyped))->toBeTrue();
    expect(array_is_list($untyped), 'extra')->toBeTrue();
    \expect(\array_is_list($untyped))->toBeTrue();
    EXPECT(array_is_list($untyped))->ToBeTrue();
    expect(ARRAY_IS_LIST($untyped))->toBeTrue();
    expect((array_is_list(($untyped))))->toBeTrue();
    expect(array_is_list($untyped))->toBeTruthy();
    expect(array_is_list($untyped))->each->toBeTrue();
    $x->expect(array_is_list($untyped))->toBeTrue();
    expect(array_is_list($untyped))?->toBeTrue();
    expect(array_is_list('abc'))->toBeTrue();
    expect(array_is_list([1]))->toBeTrue();
    expect(array_is_list($this->value))->toBeTrue();
    expect(array_is_list(strtolower($untyped)))->toBeTrue();
    expect(array_is_list(...$args))->toBeTrue();
    expect(array_is_list())->toBeTrue();
    expect(array_is_list($a, $b, $c))->toBeTrue();
});
-----
<?php

test('a', function (array $good, string $bad, $untyped) {
    expect($good)->toBeList();
    expect(array_is_list($bad))->toBeTrue();
    expect($untyped)->not->toBeList();
    expect($undefined)->toBeList();
    expect($untyped)->toBeList();
    expect($untyped)->not->toBeList();
    expect($a)->toBeInt()->and($untyped)->toBeList();
    expect($a)->toBeInt()->and($untyped)->toBeList()->and($b)->toBe(1);
    expect($untyped)->toBeList()->toBeBool();
    expect($untyped)->toBeList();
    expect($untyped)->toBeList();
    expect($untyped, 'extra')->toBeList();
    \expect($untyped)->toBeList();
    EXPECT(array_is_list($untyped))->ToBeTrue();
    expect(ARRAY_IS_LIST($untyped))->toBeTrue();
    expect($untyped)->toBeList();
    expect(array_is_list($untyped))->toBeTruthy();
    expect(array_is_list($untyped))->each->toBeTrue();
    $x->expect(array_is_list($untyped))->toBeTrue();
    expect(array_is_list($untyped))?->toBeTrue();
    expect(array_is_list('abc'))->toBeTrue();
    expect([1])->toBeList();
    expect($this->value)->toBeList();
    expect(array_is_list(strtolower($untyped)))->toBeTrue();
    expect($args)->toBeList();
    expect(array_is_list())->toBeTrue();
    expect(array_is_list($a, $b, $c))->toBeTrue();
});
