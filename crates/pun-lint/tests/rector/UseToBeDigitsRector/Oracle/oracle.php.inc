<?php

test('a', function (string $good, float $bad, $untyped) {
    expect(ctype_digit($good))->toBeTrue();
    expect(ctype_digit($bad))->toBeTrue();
    expect(ctype_digit($untyped))->toBeFalse();
    expect(ctype_digit($undefined))->not->toBeFalse();
    expect(ctype_digit($untyped))
        ->toBeTrue();
    expect(ctype_digit($untyped)) // why
        ->not->toBeTrue();
    expect($a)->toBeInt()->and(ctype_digit($untyped))->toBeTrue();
    expect($a)->toBeInt()->and(ctype_digit($untyped))->toBeTrue()->and($b)->toBe(1);
    expect(ctype_digit($untyped))->toBeTrue()->toBeBool();
    expect(ctype_digit($untyped))->toBeTrue('message');
    expect(value: ctype_digit($untyped))->toBeTrue();
    expect(ctype_digit($untyped), 'extra')->toBeTrue();
    \expect(\ctype_digit($untyped))->toBeTrue();
    EXPECT(ctype_digit($untyped))->ToBeTrue();
    expect(CTYPE_DIGIT($untyped))->toBeTrue();
    expect((ctype_digit(($untyped))))->toBeTrue();
    expect(ctype_digit($untyped))->toBeTruthy();
    expect(ctype_digit($untyped))->each->toBeTrue();
    $x->expect(ctype_digit($untyped))->toBeTrue();
    expect(ctype_digit($untyped))?->toBeTrue();
    expect(ctype_digit('abc'))->toBeTrue();
    expect(ctype_digit([1]))->toBeTrue();
    expect(ctype_digit($this->value))->toBeTrue();
    expect(ctype_digit(strtolower($untyped)))->toBeTrue();
    expect(ctype_digit(...$args))->toBeTrue();
    expect(ctype_digit())->toBeTrue();
    expect(ctype_digit($a, $b, $c))->toBeTrue();
});
-----
<?php

test('a', function (string $good, float $bad, $untyped) {
    expect($good)->toBeDigits();
    expect(ctype_digit($bad))->toBeTrue();
    expect(ctype_digit($untyped))->toBeFalse();
    expect(ctype_digit($undefined))->not->toBeFalse();
    expect(ctype_digit($untyped))
        ->toBeTrue();
    expect(ctype_digit($untyped)) // why
        ->not->toBeTrue();
    expect($a)->toBeInt()->and(ctype_digit($untyped))->toBeTrue();
    expect($a)->toBeInt()->and(ctype_digit($untyped))->toBeTrue()->and($b)->toBe(1);
    expect(ctype_digit($untyped))->toBeTrue()->toBeBool();
    expect(ctype_digit($untyped))->toBeTrue('message');
    expect(value: ctype_digit($untyped))->toBeTrue();
    expect(ctype_digit($untyped), 'extra')->toBeTrue();
    \expect(\ctype_digit($untyped))->toBeTrue();
    EXPECT(ctype_digit($untyped))->ToBeTrue();
    expect(CTYPE_DIGIT($untyped))->toBeTrue();
    expect((ctype_digit(($untyped))))->toBeTrue();
    expect(ctype_digit($untyped))->toBeTruthy();
    expect(ctype_digit($untyped))->each->toBeTrue();
    $x->expect(ctype_digit($untyped))->toBeTrue();
    expect(ctype_digit($untyped))?->toBeTrue();
    expect('abc')->toBeDigits();
    expect(ctype_digit([1]))->toBeTrue();
    expect(ctype_digit($this->value))->toBeTrue();
    expect(strtolower($untyped))->toBeDigits();
    expect(ctype_digit(...$args))->toBeTrue();
    expect(ctype_digit())->toBeTrue();
    expect(ctype_digit($a, $b, $c))->toBeTrue();
});
