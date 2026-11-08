<?php

test('a', function ($x) {
    expect($x)->toHaveCount(0);
    expect($x)->toHaveCount( 0 );
    expect($x)->toHaveCount(0x0);
    expect($x)->toHaveCount(1);
    expect($x)->toHaveCount('0');
    expect($x)->toHaveCount(0, 'msg');
    expect($x)->not->toHaveCount(0);
    expect($x)
        ->toHaveCount(0);
    expect($x)->TOHAVECOUNT(0);
    expect(count($x))->toBe(0);
    expect(sizeof($x))->toEqual(0);
    expect(COUNT($x))->toBe(0);
    expect(count($x, 1))->toBe(0);
    expect(count($x))->toBe(1);
    expect(count($x))->not->toBe(0);
    expect(count($x), 'msg')->toBe(0);
    expect($a)->and(count($x))->toBe(0);
    expect(count($x))
        ->toBe(0);
    expect(count($x))->toBeInt()->toBe(0);
    expect(empty($x))->toBeTrue();
    expect(empty($x))->toBeFalse();
    expect(empty($x))->not->toBeTrue();
    expect(empty($x))->not->toBeFalse();
    expect(empty($x))->TOBETRUE();
    expect(empty($x))->TOBEFALSE();
    expect(empty($x))
        ->toBeTrue();
    expect(empty($x))->toBeBool()->toBeTrue();
    expect($a)->and(empty($x))->toBeTrue();
    expect($a)->toBeInt()->and(empty($x))->toBeTrue();
    expect(empty($x), 'msg')->toBeTrue();
    expect(empty($x['k']))->toBeTrue('msg');
    expect(!empty($x))->toBeTrue();
    expect($x)->toBe(0);
    expect(empty($x))->toBeTrue()->and($y)->toBe(1);
});
-----
<?php

test('a', function ($x) {
    expect($x)->toBeEmpty();
    expect($x)->toBeEmpty();
    expect($x)->toBeEmpty();
    expect($x)->toHaveCount(1);
    expect($x)->toHaveCount('0');
    expect($x)->toHaveCount(0, 'msg');
    expect($x)->not->toBeEmpty();
    expect($x)->toBeEmpty();
    expect($x)->toBeEmpty();
    expect($x)->toBeEmpty();
    expect($x)->toBeEmpty();
    expect($x)->toBeEmpty();
    expect(count($x, 1))->toBe(0);
    expect(count($x))->toBe(1);
    expect($x)->not->toBeEmpty();
    expect($x)->toBeEmpty();
    expect($a)->and($x)->toBeEmpty();
    expect($x)->toBeEmpty();
    expect($x)->toBeInt()->toBeEmpty();
    expect(empty($x))->toBeTrue();
    expect(empty($x))->toBeFalse();
    expect(empty($x))->not->toBeTrue();
    expect(empty($x))->not->toBeFalse();
    expect(empty($x))->TOBETRUE();
    expect(empty($x))->TOBEFALSE();
    expect(empty($x))
        ->toBeTrue();
    expect(empty($x))->toBeBool()->toBeTrue();
    expect($a)->and(empty($x))->toBeTrue();
    expect($a)->toBeInt()->and(empty($x))->toBeTrue();
    expect(empty($x), 'msg')->toBeTrue();
    expect(empty($x['k']))->toBeTrue('msg');
    expect(!empty($x))->toBeTrue();
    expect($x)->toBe(0);
    expect(empty($x))->toBeTrue()->and($y)->toBe(1);
});
