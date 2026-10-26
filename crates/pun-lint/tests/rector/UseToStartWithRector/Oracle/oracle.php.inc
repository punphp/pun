<?php

test('a', function (string $good, object $bad, $untyped) {
    expect(str_starts_with($good, 'x'))->toBeTrue();
    expect(str_starts_with($bad, 'x'))->toBeTrue();
    expect(str_starts_with($untyped, 'x'))->toBeFalse();
    expect(str_starts_with($undefined, 'x'))->not->toBeFalse();
    expect(str_starts_with($untyped, 'x'))
        ->toBeTrue();
    expect(str_starts_with($untyped, 'x')) // why
        ->not->toBeTrue();
    expect($a)->toBeInt()->and(str_starts_with($untyped, 'x'))->toBeTrue();
    expect($a)->toBeInt()->and(str_starts_with($untyped, 'x'))->toBeTrue()->and($b)->toBe(1);
    expect(str_starts_with($untyped, 'x'))->toBeTrue()->toBeBool();
    expect(str_starts_with($untyped, 'x'))->toBeTrue('message');
    expect(value: str_starts_with($untyped, 'x'))->toBeTrue();
    expect(str_starts_with($untyped, 'x'), 'extra')->toBeTrue();
    \expect(\str_starts_with($untyped, 'x'))->toBeTrue();
    EXPECT(str_starts_with($untyped, 'x'))->ToBeTrue();
    expect(STR_STARTS_WITH($untyped, 'x'))->toBeTrue();
    expect((str_starts_with(($untyped), 'x')))->toBeTrue();
    expect(str_starts_with($untyped, 'x'))->toBeTruthy();
    expect(str_starts_with($untyped, 'x'))->each->toBeTrue();
    $x->expect(str_starts_with($untyped, 'x'))->toBeTrue();
    expect(str_starts_with($untyped, 'x'))?->toBeTrue();
    expect(str_starts_with('abc', 'x'))->toBeTrue();
    expect(str_starts_with([1], 'x'))->toBeTrue();
    expect(str_starts_with($this->value, 'x'))->toBeTrue();
    expect(str_starts_with(strtolower($untyped), 'x'))->toBeTrue();
    expect(str_starts_with())->toBeTrue();
    expect(str_starts_with($a, $b, $c))->toBeTrue();
    expect(str_starts_with($untyped, needle: $n))->toBeTrue();
    expect(str_starts_with($untyped, ...$n))->toBeTrue();
});
-----
<?php

test('a', function (string $good, object $bad, $untyped) {
    expect($good)->toStartWith('x');
    expect(str_starts_with($bad, 'x'))->toBeTrue();
    expect($untyped)->not->toStartWith('x');
    expect($undefined)->toStartWith('x');
    expect($untyped)->toStartWith('x');
    expect($untyped)->not->toStartWith('x');
    expect($a)->toBeInt()->and($untyped)->toStartWith('x');
    expect($a)->toBeInt()->and($untyped)->toStartWith('x')->and($b)->toBe(1);
    expect($untyped)->toStartWith('x')->toBeBool();
    expect($untyped)->toStartWith('x');
    expect($untyped)->toStartWith('x');
    expect($untyped, 'extra')->toStartWith('x');
    \expect($untyped)->toStartWith('x');
    EXPECT(str_starts_with($untyped, 'x'))->ToBeTrue();
    expect(STR_STARTS_WITH($untyped, 'x'))->toBeTrue();
    expect($untyped)->toStartWith('x');
    expect(str_starts_with($untyped, 'x'))->toBeTruthy();
    expect(str_starts_with($untyped, 'x'))->each->toBeTrue();
    $x->expect(str_starts_with($untyped, 'x'))->toBeTrue();
    expect(str_starts_with($untyped, 'x'))?->toBeTrue();
    expect('abc')->toStartWith('x');
    expect(str_starts_with([1], 'x'))->toBeTrue();
    expect($this->value)->toStartWith('x');
    expect(strtolower($untyped))->toStartWith('x');
    expect(str_starts_with())->toBeTrue();
    expect(str_starts_with($a, $b, $c))->toBeTrue();
    expect($untyped)->toStartWith($n);
    expect($untyped)->toStartWith($n);
});
