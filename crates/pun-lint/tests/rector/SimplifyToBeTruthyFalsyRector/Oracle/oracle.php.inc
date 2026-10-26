<?php

test('a', function ($x) {
    expect((bool) $x)->toBeTrue();
    expect((boolean) $x)->toBeFalse();
    expect(boolval($x))->not->toBeTrue();
    expect(\boolval($x))->TOBETRUE();
    expect(BOOLVAL($x))->toBeFalse();
    expect(boolval())->toBeTrue();
    expect(boolval($x, $y))->toBeTrue();
    expect(boolval(value: $x))->toBeTrue();
    expect((bool) ($x + 1))->toBeTrue();
    expect($a)->and((bool) $x)->toBeTrue();
    expect($a)->and((bool) $x, 'msg')->toBeTrue();
    expect($a)
        ->and((bool) $x, 'msg')
        ->toBeTrue();
    expect((bool) $x, 'msg')->toBeTrue();
    expect(!$x)->toBeTrue();
    expect((int) $x)->toBeTrue();
    expect((bool) $x)->toBeInt()->toBeTrue();
    expect((bool) $x)
        ->toBeTrue();
    expect((bool) $x)->toBe(true);
});
-----
<?php

test('a', function ($x) {
    expect($x)->toBeTruthy();
    expect($x)->toBeFalsy();
    expect($x)->not->toBeTruthy();
    expect($x)->toBeTruthy();
    expect($x)->toBeFalsy();
    expect(boolval())->toBeTrue();
    expect(boolval($x, $y))->toBeTrue();
    expect($x)->toBeTruthy();
    expect($x + 1)->toBeTruthy();
    expect($a)->and($x)->toBeTruthy();
    expect($a)->and($x)->toBeTruthy();
    expect($a)->and($x)
        ->toBeTruthy();
    expect($x)->toBeTruthy();
    expect(!$x)->toBeTrue();
    expect((int) $x)->toBeTrue();
    expect($x)->toBeInt()->toBeTruthy();
    expect($x)
        ->toBeTruthy();
    expect((bool) $x)->toBe(true);
});
