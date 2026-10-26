<?php

test('a', function (string $good, int $bad, $untyped) {
    expect(ctype_alpha($good))->toBeTrue();
    expect(ctype_alpha($bad))->toBeTrue();
    expect(ctype_alpha($untyped))->toBeFalse();
    expect(ctype_alpha($undefined))->not->toBeFalse();
    expect(ctype_alpha($untyped))
        ->toBeTrue();
    expect(ctype_alpha($untyped)) // why
        ->not->toBeTrue();
    expect($a)->toBeInt()->and(ctype_alpha($untyped))->toBeTrue();
    expect($a)->toBeInt()->and(ctype_alpha($untyped))->toBeTrue()->and($b)->toBe(1);
    expect(ctype_alpha($untyped))->toBeTrue()->toBeBool();
    expect(ctype_alpha($untyped))->toBeTrue('message');
    expect(value: ctype_alpha($untyped))->toBeTrue();
    expect(ctype_alpha($untyped), 'extra')->toBeTrue();
    \expect(\ctype_alpha($untyped))->toBeTrue();
    EXPECT(ctype_alpha($untyped))->ToBeTrue();
    expect(CTYPE_ALPHA($untyped))->toBeTrue();
    expect((ctype_alpha(($untyped))))->toBeTrue();
    expect(ctype_alpha($untyped))->toBeTruthy();
    expect(ctype_alpha($untyped))->each->toBeTrue();
    $x->expect(ctype_alpha($untyped))->toBeTrue();
    expect(ctype_alpha($untyped))?->toBeTrue();
    expect(ctype_alpha('abc'))->toBeTrue();
    expect(ctype_alpha([1]))->toBeTrue();
    expect(ctype_alpha($this->value))->toBeTrue();
    expect(ctype_alpha(strtolower($untyped)))->toBeTrue();
    expect(ctype_alpha(...$args))->toBeTrue();
    expect(ctype_alpha())->toBeTrue();
    expect(ctype_alpha($a, $b, $c))->toBeTrue();
});
-----
<?php

test('a', function (string $good, int $bad, $untyped) {
    expect($good)->toBeAlpha();
    expect(ctype_alpha($bad))->toBeTrue();
    expect($untyped)->not->toBeAlpha();
    expect($undefined)->toBeAlpha();
    expect($untyped)->toBeAlpha();
    expect($untyped)->not->toBeAlpha();
    expect($a)->toBeInt()->and($untyped)->toBeAlpha();
    expect($a)->toBeInt()->and($untyped)->toBeAlpha()->and($b)->toBe(1);
    expect($untyped)->toBeAlpha()->toBeBool();
    expect($untyped)->toBeAlpha();
    expect($untyped)->toBeAlpha();
    expect($untyped, 'extra')->toBeAlpha();
    \expect($untyped)->toBeAlpha();
    EXPECT(ctype_alpha($untyped))->ToBeTrue();
    expect(CTYPE_ALPHA($untyped))->toBeTrue();
    expect($untyped)->toBeAlpha();
    expect(ctype_alpha($untyped))->toBeTruthy();
    expect(ctype_alpha($untyped))->each->toBeTrue();
    $x->expect(ctype_alpha($untyped))->toBeTrue();
    expect(ctype_alpha($untyped))?->toBeTrue();
    expect('abc')->toBeAlpha();
    expect(ctype_alpha([1]))->toBeTrue();
    expect($this->value)->toBeAlpha();
    expect(strtolower($untyped))->toBeAlpha();
    expect($args)->toBeAlpha();
    expect(ctype_alpha())->toBeTrue();
    expect(ctype_alpha($a, $b, $c))->toBeTrue();
});
