<?php

test('a', function (string $s, int $i, $untyped) {
    expect(strtolower($s) === $s)->toBeTrue();
    expect($s === strtolower($s))->toBeTrue();
    expect(strtolower($i) === $i)->toBeTrue();
    expect($i === strtolower($i))->toBeTrue();
    expect(strtolower($untyped) === $untyped)->toBeTrue();
    expect(strtolower($s) === $other)->toBeTrue();
    expect(strtolower($s) == $s)->toBeTrue();
    expect(strtolower($s) === $s)->toBeFalse();
    expect(strtolower($s) === $s)->not->toBeTrue();
    expect(STRTOLOWER($s) === $s)->toBeTrue();
    expect(\strtolower($s) === $s)->toBeTrue();
    expect(strtolower() === $s)->toBeTrue();
    expect(strtolower($s) === strtolower($s))->toBeTrue();
    expect(strtolower($a->b) === $a -> b)->toBeTrue();
    expect($a)->and(strtolower($s) === $s)->toBeTrue();
    expect(strtolower($s) === $s, 'x')->toBeTrue();
    expect(strtolower($s) === $s)->toBeBool()->toBeTrue();
    expect(strtolower($s) === $s)
        ->toBeTrue();
    expect(strtolower(1) === 1)->toBeTrue();
    expect(strtolower('a') === 'a')->toBeTrue();
});
-----
<?php

test('a', function (string $s, int $i, $untyped) {
    expect($s)->toBeLowercase();
    expect($s)->toBeLowercase();
    expect(strtolower($i) === $i)->toBeTrue();
    expect($i === strtolower($i))->toBeTrue();
    expect($untyped)->toBeLowercase();
    expect(strtolower($s) === $other)->toBeTrue();
    expect(strtolower($s) == $s)->toBeTrue();
    expect(strtolower($s) === $s)->toBeFalse();
    expect($s)->not->toBeLowercase();
    expect($s)->toBeLowercase();
    expect($s)->toBeLowercase();
    expect(strtolower() === $s)->toBeTrue();
    expect(strtolower($s) === strtolower($s))->toBeTrue();
    expect($a -> b)->toBeLowercase();
    expect($a)->and($s)->toBeLowercase();
    expect($s)->toBeLowercase();
    expect($s)->toBeBool()->toBeLowercase();
    expect($s)
        ->toBeLowercase();
    expect(strtolower(1) === 1)->toBeTrue();
    expect('a')->toBeLowercase();
});
