<?php

test('a', function (string $good, int $bad, $untyped) {
    expect(str_ends_with($good, 'x'))->toBeTrue();
    expect(str_ends_with($bad, 'x'))->toBeTrue();
    expect(str_ends_with($untyped, 'x'))->toBeFalse();
    expect(str_ends_with($undefined, 'x'))->not->toBeFalse();
    expect(str_ends_with($untyped, 'x'))
        ->toBeTrue();
    expect(str_ends_with($untyped, 'x')) // why
        ->not->toBeTrue();
    expect($a)->toBeInt()->and(str_ends_with($untyped, 'x'))->toBeTrue();
    expect($a)->toBeInt()->and(str_ends_with($untyped, 'x'))->toBeTrue()->and($b)->toBe(1);
    expect(str_ends_with($untyped, 'x'))->toBeTrue()->toBeBool();
    expect(str_ends_with($untyped, 'x'))->toBeTrue('message');
    expect(value: str_ends_with($untyped, 'x'))->toBeTrue();
    expect(str_ends_with($untyped, 'x'), 'extra')->toBeTrue();
    \expect(\str_ends_with($untyped, 'x'))->toBeTrue();
    EXPECT(str_ends_with($untyped, 'x'))->ToBeTrue();
    expect(STR_ENDS_WITH($untyped, 'x'))->toBeTrue();
    expect((str_ends_with(($untyped), 'x')))->toBeTrue();
    expect(str_ends_with($untyped, 'x'))->toBeTruthy();
    expect(str_ends_with($untyped, 'x'))->each->toBeTrue();
    $x->expect(str_ends_with($untyped, 'x'))->toBeTrue();
    expect(str_ends_with($untyped, 'x'))?->toBeTrue();
    expect(str_ends_with('abc', 'x'))->toBeTrue();
    expect(str_ends_with([1], 'x'))->toBeTrue();
    expect(str_ends_with($this->value, 'x'))->toBeTrue();
    expect(str_ends_with(strtolower($untyped), 'x'))->toBeTrue();
    expect(str_ends_with())->toBeTrue();
    expect(str_ends_with($a, $b, $c))->toBeTrue();
    expect(str_ends_with($untyped, needle: $n))->toBeTrue();
    expect(str_ends_with($untyped, ...$n))->toBeTrue();
});
-----
<?php

test('a', function (string $good, int $bad, $untyped) {
    expect($good)->toEndWith('x');
    expect(str_ends_with($bad, 'x'))->toBeTrue();
    expect(str_ends_with($untyped, 'x'))->toBeFalse();
    expect(str_ends_with($undefined, 'x'))->not->toBeFalse();
    expect(str_ends_with($untyped, 'x'))
        ->toBeTrue();
    expect(str_ends_with($untyped, 'x')) // why
        ->not->toBeTrue();
    expect($a)->toBeInt()->and(str_ends_with($untyped, 'x'))->toBeTrue();
    expect($a)->toBeInt()->and(str_ends_with($untyped, 'x'))->toBeTrue()->and($b)->toBe(1);
    expect(str_ends_with($untyped, 'x'))->toBeTrue()->toBeBool();
    expect(str_ends_with($untyped, 'x'))->toBeTrue('message');
    expect(value: str_ends_with($untyped, 'x'))->toBeTrue();
    expect(str_ends_with($untyped, 'x'), 'extra')->toBeTrue();
    \expect(\str_ends_with($untyped, 'x'))->toBeTrue();
    EXPECT(str_ends_with($untyped, 'x'))->ToBeTrue();
    expect(STR_ENDS_WITH($untyped, 'x'))->toBeTrue();
    expect((str_ends_with(($untyped), 'x')))->toBeTrue();
    expect(str_ends_with($untyped, 'x'))->toBeTruthy();
    expect(str_ends_with($untyped, 'x'))->each->toBeTrue();
    $x->expect(str_ends_with($untyped, 'x'))->toBeTrue();
    expect(str_ends_with($untyped, 'x'))?->toBeTrue();
    expect('abc')->toEndWith('x');
    expect(str_ends_with([1], 'x'))->toBeTrue();
    expect(str_ends_with($this->value, 'x'))->toBeTrue();
    expect(strtolower($untyped))->toEndWith('x');
    expect(str_ends_with())->toBeTrue();
    expect(str_ends_with($a, $b, $c))->toBeTrue();
    expect(str_ends_with($untyped, needle: $n))->toBeTrue();
    expect(str_ends_with($untyped, ...$n))->toBeTrue();
});
