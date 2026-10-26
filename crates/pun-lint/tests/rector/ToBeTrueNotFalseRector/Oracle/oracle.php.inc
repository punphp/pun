<?php

function isGood(): bool
{
    return true;
}

test('a', function (bool $flag, ?bool $maybe, $untyped, string $s) {
    expect($flag)->not->toBeFalse();
    expect($flag)->not->toBeTrue();
    expect($maybe)->not->toBeFalse();
    expect($untyped)->not->toBeFalse();
    expect($a === $b)->not->toBeFalse();
    expect(is_string($untyped))->not->toBeTrue();
    expect((bool) $untyped)->not->toBeFalse();
    expect(!$untyped)->not->toBeFalse();
    expect(true)->not->toBeFalse();
    expect($untyped instanceof Foo)->not->toBeFalse();
    expect(in_array($s, ['a']))->not->toBeFalse();
    expect(isGood())->not->toBeFalse();
    expect(str_contains($s, 'a'))->not->toBeFalse();
    expect($flag)->not->toBefalse();
    expect($flag)->toBeFalse();
    expect($flag)->not->toBeTruthy();
    expect($a)->and($flag)->not->toBeFalse();
    expect($flag)
        ->not
        ->toBeFalse();
    expect($flag)->not->toBeFalse()->and($s)->not->toBeTrue();
    expect($s)->and($flag && $flag)->not->toBeTrue();
    expect($s === 'a' || $flag)->not->toBeTrue();
    expect(1 > 2)->not->toBeTrue();
});
-----
<?php

function isGood(): bool
{
    return true;
}

test('a', function (bool $flag, ?bool $maybe, $untyped, string $s) {
    expect($flag)->toBeTrue();
    expect($flag)->toBeFalse();
    expect($maybe)->not->toBeFalse();
    expect($untyped)->not->toBeFalse();
    expect($a === $b)->toBeTrue();
    expect(is_string($untyped))->toBeFalse();
    expect((bool) $untyped)->toBeTrue();
    expect(!$untyped)->toBeTrue();
    expect(true)->toBeTrue();
    expect($untyped instanceof Foo)->toBeTrue();
    expect(in_array($s, ['a']))->toBeTrue();
    expect(isGood())->toBeTrue();
    expect(str_contains($s, 'a'))->toBeTrue();
    expect($flag)->not->toBefalse();
    expect($flag)->toBeFalse();
    expect($flag)->not->toBeTruthy();
    expect($a)->and($flag)->toBeTrue();
    expect($flag)
        ->toBeTrue();
    expect($flag)->toBeTrue()->and($s)->not->toBeTrue();
    expect($s)->and($flag && $flag)->toBeFalse();
    expect($s === 'a' || $flag)->toBeFalse();
    expect(1 > 2)->toBeFalse();
});
