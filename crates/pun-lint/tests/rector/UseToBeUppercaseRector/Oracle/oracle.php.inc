<?php

test('a', function (string $s, int $i, $untyped) {
    expect(strtoupper($s) === $s)->toBeTrue();
    expect($s === strtoupper($s))->toBeTrue();
    expect(strtoupper($i) === $i)->toBeTrue();
    expect($i === strtoupper($i))->toBeTrue();
    expect(strtoupper($untyped) === $untyped)->toBeTrue();
    expect(strtoupper($s) === $other)->toBeTrue();
    expect(strtoupper($s) == $s)->toBeTrue();
    expect(strtoupper($s) === $s)->toBeFalse();
    expect(strtoupper($s) === $s)->not->toBeTrue();
    expect(STRTOUPPER($s) === $s)->toBeTrue();
    expect(\strtoupper($s) === $s)->toBeTrue();
    expect(strtoupper() === $s)->toBeTrue();
    expect(strtoupper($s) === strtoupper($s))->toBeTrue();
    expect(strtoupper($a->b) === $a -> b)->toBeTrue();
    expect($a)->and(strtoupper($s) === $s)->toBeTrue();
    expect(strtoupper($s) === $s, 'x')->toBeTrue();
    expect(strtoupper($s) === $s)->toBeBool()->toBeTrue();
    expect(strtoupper($s) === $s)
        ->toBeTrue();
    expect(strtoupper(1) === 1)->toBeTrue();
    expect(strtoupper('a') === 'a')->toBeTrue();
});
-----
<?php

test('a', function (string $s, int $i, $untyped) {
    expect($s)->toBeUppercase();
    expect($s)->toBeUppercase();
    expect(strtoupper($i) === $i)->toBeTrue();
    expect($i === strtoupper($i))->toBeTrue();
    expect($untyped)->toBeUppercase();
    expect(strtoupper($s) === $other)->toBeTrue();
    expect(strtoupper($s) == $s)->toBeTrue();
    expect(strtoupper($s) === $s)->toBeFalse();
    expect($s)->not->toBeUppercase();
    expect($s)->toBeUppercase();
    expect($s)->toBeUppercase();
    expect(strtoupper() === $s)->toBeTrue();
    expect(strtoupper($s) === strtoupper($s))->toBeTrue();
    expect($a -> b)->toBeUppercase();
    expect($a)->and($s)->toBeUppercase();
    expect($s)->toBeUppercase();
    expect($s)->toBeBool()->toBeUppercase();
    expect($s)
        ->toBeUppercase();
    expect(strtoupper(1) === 1)->toBeTrue();
    expect('a')->toBeUppercase();
});
