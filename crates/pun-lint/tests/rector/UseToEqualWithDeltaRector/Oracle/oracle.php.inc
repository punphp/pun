<?php

test('a', function ($a, $b) {
    expect(abs($a - $b) < 0.01)->toBeTrue();
    expect(abs($a - $b) <= 0.01)->toBeTrue();
    expect(abs($a - $b) > 0.01)->toBeTrue();
    expect(abs($a - $b) < 0.01)->toBeFalse();
    expect(abs($a - $b) < 0.01)->not->toBeTrue();
    expect(abs($a + $b) < 0.01)->toBeTrue();
    expect(abs($a) < 0.01)->toBeTrue();
    expect(abs($a - $b, 1) < 0.01)->toBeTrue();
    expect(ABS($a - $b) < 0.01)->toBeTrue();
    expect(\abs($a - $b) < 0.01)->toBeTrue();
    expect(abs($a - $b) < 0.01)->TOBETRUE();
    expect(0.01 > abs($a - $b))->toBeTrue();
    expect(abs(($a - $b)) < (0.01))->toBeTrue();
    expect(abs($a - $b - $c) < $d)->toBeTrue();
    expect(abs($a - ($b - $c)) < $d)->toBeTrue();
    expect(abs($a - $b) < 0.01)->toBeFloat()->toBeTrue();
    expect(abs($a - $b) < 0.01)->and($x)->toBeTrue();
    expect($x)->and(abs($a - $b) < 0.01)->toBeTrue();
    expect(abs($a - $b) < 0.01, 'msg')->toBeTrue();
    expect(abs($a - $b) < 0.01)
        ->toBeTrue();
    expect(abs($a - $b) < 0.01)->toBeTrue('msg');
});
-----
<?php

test('a', function ($a, $b) {
    expect($a)->toEqualWithDelta($b, 0.01);
    expect($a)->toEqualWithDelta($b, 0.01);
    expect(abs($a - $b) > 0.01)->toBeTrue();
    expect(abs($a - $b) < 0.01)->toBeFalse();
    expect($a)->not->toEqualWithDelta($b, 0.01);
    expect(abs($a + $b) < 0.01)->toBeTrue();
    expect(abs($a) < 0.01)->toBeTrue();
    expect(abs($a - $b, 1) < 0.01)->toBeTrue();
    expect($a)->toEqualWithDelta($b, 0.01);
    expect($a)->toEqualWithDelta($b, 0.01);
    expect(abs($a - $b) < 0.01)->TOBETRUE();
    expect(0.01 > abs($a - $b))->toBeTrue();
    expect($a)->toEqualWithDelta($b, 0.01);
    expect($a - $b)->toEqualWithDelta($c, $d);
    expect($a)->toEqualWithDelta($b - $c, $d);
    expect($a)->toEqualWithDelta($b, 0.01);
    expect($a)->toEqualWithDelta($b, 0.01);
    expect($x)->and(abs($a - $b) < 0.01)->toBeTrue();
    expect($a, 'msg')->toEqualWithDelta($b, 0.01);
    expect($a)->toEqualWithDelta($b, 0.01);
    expect($a)->toEqualWithDelta($b, 0.01);
});
