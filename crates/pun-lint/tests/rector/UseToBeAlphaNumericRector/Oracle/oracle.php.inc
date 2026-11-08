<?php

test('a', function (string $good, array $bad, $untyped) {
    expect(ctype_alnum($good))->toBeTrue();
    expect(ctype_alnum($bad))->toBeTrue();
    expect(ctype_alnum($untyped))->toBeFalse();
    expect(ctype_alnum($undefined))->not->toBeFalse();
    expect(ctype_alnum($untyped))
        ->toBeTrue();
    expect(ctype_alnum($untyped)) // why
        ->not->toBeTrue();
    expect($a)->toBeInt()->and(ctype_alnum($untyped))->toBeTrue();
    expect($a)->toBeInt()->and(ctype_alnum($untyped))->toBeTrue()->and($b)->toBe(1);
    expect(ctype_alnum($untyped))->toBeTrue()->toBeBool();
    expect(ctype_alnum($untyped))->toBeTrue('message');
    expect(value: ctype_alnum($untyped))->toBeTrue();
    expect(ctype_alnum($untyped), 'extra')->toBeTrue();
    \expect(\ctype_alnum($untyped))->toBeTrue();
    EXPECT(ctype_alnum($untyped))->ToBeTrue();
    expect(CTYPE_ALNUM($untyped))->toBeTrue();
    expect((ctype_alnum(($untyped))))->toBeTrue();
    expect(ctype_alnum($untyped))->toBeTruthy();
    expect(ctype_alnum($untyped))->each->toBeTrue();
    $x->expect(ctype_alnum($untyped))->toBeTrue();
    expect(ctype_alnum($untyped))?->toBeTrue();
    expect(ctype_alnum('abc'))->toBeTrue();
    expect(ctype_alnum([1]))->toBeTrue();
    expect(ctype_alnum($this->value))->toBeTrue();
    expect(ctype_alnum(strtolower($untyped)))->toBeTrue();
    expect(ctype_alnum(...$args))->toBeTrue();
    expect(ctype_alnum())->toBeTrue();
    expect(ctype_alnum($a, $b, $c))->toBeTrue();
});
-----
<?php

test('a', function (string $good, array $bad, $untyped) {
    expect($good)->toBeAlphaNumeric();
    expect(ctype_alnum($bad))->toBeTrue();
    expect(ctype_alnum($untyped))->toBeFalse();
    expect(ctype_alnum($undefined))->not->toBeFalse();
    expect(ctype_alnum($untyped))
        ->toBeTrue();
    expect(ctype_alnum($untyped)) // why
        ->not->toBeTrue();
    expect($a)->toBeInt()->and(ctype_alnum($untyped))->toBeTrue();
    expect($a)->toBeInt()->and(ctype_alnum($untyped))->toBeTrue()->and($b)->toBe(1);
    expect(ctype_alnum($untyped))->toBeTrue()->toBeBool();
    expect(ctype_alnum($untyped))->toBeTrue('message');
    expect(value: ctype_alnum($untyped))->toBeTrue();
    expect(ctype_alnum($untyped), 'extra')->toBeTrue();
    \expect(\ctype_alnum($untyped))->toBeTrue();
    EXPECT(ctype_alnum($untyped))->ToBeTrue();
    expect(CTYPE_ALNUM($untyped))->toBeTrue();
    expect((ctype_alnum(($untyped))))->toBeTrue();
    expect(ctype_alnum($untyped))->toBeTruthy();
    expect(ctype_alnum($untyped))->each->toBeTrue();
    $x->expect(ctype_alnum($untyped))->toBeTrue();
    expect(ctype_alnum($untyped))?->toBeTrue();
    expect('abc')->toBeAlphaNumeric();
    expect(ctype_alnum([1]))->toBeTrue();
    expect(ctype_alnum($this->value))->toBeTrue();
    expect(strtolower($untyped))->toBeAlphaNumeric();
    expect(ctype_alnum(...$args))->toBeTrue();
    expect(ctype_alnum())->toBeTrue();
    expect(ctype_alnum($a, $b, $c))->toBeTrue();
});
