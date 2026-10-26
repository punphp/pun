<?php

test('a', function ($x) {
    expect($x === 1)->toBeTrue();
    expect($x !== 1)->toBeTrue();
    expect($x === 1)->toBeFalse();
    expect($x !== 1)->toBeFalse();
    expect($x === 1)->not->toBeTrue();
    expect($x !== 1)->not->toBeFalse();
    expect($x == 1)->toBeTrue();
    expect($x === 1)->toBeBool()->toBeTrue();
    expect($x === 1)
        ->toBeTrue();
    expect($x === 1)->and($y)->toBeTrue();
    expect($a)->and($x === 1)->toBeTrue();
    expect($x === 1, 'msg')->toBeTrue();
    expect(value: $x === 1)->toBeTrue();
    expect($x === 1)->TOBETRUE();
    expect(($x) === ($y + 1))->toBeTrue();
    expect($x === 1)->toBeTrue()->and($y)->toBe(2);
    expect($x === 1)->toBeTruthy();
});
-----
<?php

test('a', function ($x) {
    expect($x)->toBe(1);
    expect($x)->not->toBe(1);
    expect($x)->not->toBe(1);
    expect($x)->toBe(1);
    expect($x)->not->toBe(1);
    expect($x)->not->toBe(1);
    expect($x == 1)->toBeTrue();
    expect($x)->toBe(1);
    expect($x)->toBe(1);
    expect($x)->toBe(1);
    expect($a)->and($x === 1)->toBeTrue();
    expect($x, 'msg')->toBe(1);
    expect($x)->toBe(1);
    expect($x === 1)->TOBETRUE();
    expect($x)->toBe($y + 1);
    expect($x)->toBe(1)->and($y)->toBe(2);
    expect($x === 1)->toBeTruthy();
});
