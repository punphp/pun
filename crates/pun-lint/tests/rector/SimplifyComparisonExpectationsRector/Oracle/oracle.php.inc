<?php

test('a', function ($x) {
    expect($x > 10)->toBeTrue();
    expect($x >= 10)->toBeTrue();
    expect($x < 10)->toBeTrue();
    expect($x <= 10)->toBeTrue();
    expect($x == 10)->toBeTrue();
    expect($x > 10)->toBeFalse();
    expect($x > 10)->not->toBeTrue();
    expect($x > 10)->toBeTrue('message');
    expect($x > 10)
        ->toBeTrue();
    expect($x > 10)
        ->toBeTrue('message');
    expect($x > 10, 'extra')->toBeTrue();
    expect($a)->and($x > 10)->toBeTrue();
    expect($a)
        ->and($x > 10, 'extra')
        ->toBeTrue();
    expect(($x + 1) > ($y * 2))->toBeTrue();
    expect($x > 10)->toBeInt()->toBeTrue();
    expect($x > 10)->TOBETRUE();
    expect(count($a) > 0)->toBeTrue();
});
-----
<?php

test('a', function ($x) {
    expect($x)->toBeGreaterThan(10);
    expect($x)->toBeGreaterThanOrEqual(10);
    expect($x)->toBeLessThan(10);
    expect($x)->toBeLessThanOrEqual(10);
    expect($x == 10)->toBeTrue();
    expect($x > 10)->toBeFalse();
    expect($x)->not->toBeGreaterThan(10);
    expect($x)->toBeGreaterThan(10);
    expect($x)
        ->toBeGreaterThan(10);
    expect($x)
        ->toBeGreaterThan(10);
    expect($x)->toBeGreaterThan(10);
    expect($a)->and($x)->toBeGreaterThan(10);
    expect($a)->and($x)
        ->toBeGreaterThan(10);
    expect($x + 1)->toBeGreaterThan($y * 2);
    expect($x)->toBeInt()->toBeGreaterThan(10);
    expect($x)->toBeGreaterThan(10);
    expect(count($a))->toBeGreaterThan(0);
});
