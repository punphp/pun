<?php

test('a', function (int $i, float $f, string $s, $untyped, int|string $mixed) {
    expect($i >= 1 && $i <= 10)->toBeTrue();
    expect($f >= 1.5 && $f <= 2.5)->toBeTrue();
    expect($s >= 'a' && $s <= 'z')->toBeTrue();
    expect($untyped >= 1 && $untyped <= 10)->toBeTrue();
    expect($mixed >= 1 && $mixed <= 10)->toBeTrue();
    expect($i >= 1 and $i <= 10)->toBeTrue();
    expect($i > 1 && $i <= 10)->toBeTrue();
    expect($i >= 1 && $i < 10)->toBeTrue();
    expect($i <= 10 && $i >= 1)->toBeTrue();
    expect($i >= 1 && $j <= 10)->toBeTrue();
    expect($i >= 1 && $i <= 10)->toBeFalse();
    expect($i >= 1 && $i <= 10)->not->toBeTrue();
    expect($a)->and($i >= 1 && $i <= 10)->toBeTrue();
    expect($i >= 1 && $i <= 10, 'x')->toBeTrue();
    expect(($i >= 1) && ($i <= 10))->toBeTrue();
    expect($i >= 1 && $i <= 10)
        ->toBeTrue();
    expect($i >= 1 && $i <= 10)
        ->toBeTrue('message');
    expect($i + 1 >= $min && $i + 1 <= $max)->toBeTrue();
    expect(1 >= 1 && 1 <= 2)->toBeTrue();
    expect('1' >= 1 && '1' <= 2)->toBeTrue();
});
-----
<?php

test('a', function (int $i, float $f, string $s, $untyped, int|string $mixed) {
    expect($i)->toBeBetween(1, 10);
    expect($f)->toBeBetween(1.5, 2.5);
    expect($s >= 'a' && $s <= 'z')->toBeTrue();
    expect($untyped)->toBeBetween(1, 10);
    expect($mixed)->toBeBetween(1, 10);
    expect($i >= 1 and $i <= 10)->toBeTrue();
    expect($i > 1 && $i <= 10)->toBeTrue();
    expect($i >= 1 && $i < 10)->toBeTrue();
    expect($i <= 10 && $i >= 1)->toBeTrue();
    expect($i >= 1 && $j <= 10)->toBeTrue();
    expect($i >= 1 && $i <= 10)->toBeFalse();
    expect($i)->not->toBeBetween(1, 10);
    expect($a)->and($i)->toBeBetween(1, 10);
    expect($i)->toBeBetween(1, 10);
    expect($i)->toBeBetween(1, 10);
    expect($i)
        ->toBeBetween(1, 10);
    expect($i)->toBeBetween(1, 10);
    expect($i + 1)->toBeBetween($min, $max);
    expect(1)->toBeBetween(1, 2);
    expect('1' >= 1 && '1' <= 2)->toBeTrue();
});
