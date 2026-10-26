<?php

test('a', function (string $good, bool $bad, $untyped) {
    expect(is_dir($good))->toBeTrue();
    expect(is_dir($bad))->toBeTrue();
    expect(is_dir($untyped))->toBeFalse();
    expect(is_dir($undefined))->not->toBeFalse();
    expect(is_dir($untyped))
        ->toBeTrue();
    expect(is_dir($untyped)) // why
        ->not->toBeTrue();
    expect($a)->toBeInt()->and(is_dir($untyped))->toBeTrue();
    expect($a)->toBeInt()->and(is_dir($untyped))->toBeTrue()->and($b)->toBe(1);
    expect(is_dir($untyped))->toBeTrue()->toBeBool();
    expect(is_dir($untyped))->toBeTrue('message');
    expect(value: is_dir($untyped))->toBeTrue();
    expect(is_dir($untyped), 'extra')->toBeTrue();
    \expect(\is_dir($untyped))->toBeTrue();
    EXPECT(is_dir($untyped))->ToBeTrue();
    expect(IS_DIR($untyped))->toBeTrue();
    expect((is_dir(($untyped))))->toBeTrue();
    expect(is_dir($untyped))->toBeTruthy();
    expect(is_dir($untyped))->each->toBeTrue();
    $x->expect(is_dir($untyped))->toBeTrue();
    expect(is_dir($untyped))?->toBeTrue();
    expect(is_dir('abc'))->toBeTrue();
    expect(is_dir([1]))->toBeTrue();
    expect(is_dir($this->value))->toBeTrue();
    expect(is_dir(strtolower($untyped)))->toBeTrue();
    expect(is_dir(...$args))->toBeTrue();
    expect(is_dir())->toBeTrue();
    expect(is_dir($a, $b, $c))->toBeTrue();
});
-----
<?php

test('a', function (string $good, bool $bad, $untyped) {
    expect($good)->toBeDirectory();
    expect(is_dir($bad))->toBeTrue();
    expect($untyped)->not->toBeDirectory();
    expect($undefined)->toBeDirectory();
    expect($untyped)->toBeDirectory();
    expect($untyped)->not->toBeDirectory();
    expect($a)->toBeInt()->and($untyped)->toBeDirectory();
    expect($a)->toBeInt()->and($untyped)->toBeDirectory()->and($b)->toBe(1);
    expect($untyped)->toBeDirectory()->toBeBool();
    expect($untyped)->toBeDirectory();
    expect($untyped)->toBeDirectory();
    expect($untyped, 'extra')->toBeDirectory();
    \expect($untyped)->toBeDirectory();
    EXPECT(is_dir($untyped))->ToBeTrue();
    expect(IS_DIR($untyped))->toBeTrue();
    expect($untyped)->toBeDirectory();
    expect(is_dir($untyped))->toBeTruthy();
    expect(is_dir($untyped))->each->toBeTrue();
    $x->expect(is_dir($untyped))->toBeTrue();
    expect(is_dir($untyped))?->toBeTrue();
    expect('abc')->toBeDirectory();
    expect(is_dir([1]))->toBeTrue();
    expect($this->value)->toBeDirectory();
    expect(strtolower($untyped))->toBeDirectory();
    expect($args)->toBeDirectory();
    expect(is_dir())->toBeTrue();
    expect(is_dir($a, $b, $c))->toBeTrue();
});
