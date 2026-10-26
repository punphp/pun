<?php

test('a', function ($a, $b) {
    expect(sort($a))->toBe(sort($b));
    expect(sort($a))->toEqual(sort($b));
    expect(sort($a))->toBe($b);
    expect($a)->toBe(sort($b));
    expect(sort($a, SORT_STRING))->toBe(sort($b));
    expect(sort($a))->toBe(sort($b, SORT_STRING));
    expect(SORT($a))->TOBE(\sort($b));
    expect(sort($a))->toBe(sort($b), 'x');
    expect(sort($a))->not->toBe(sort($b));
    expect(sort($a))->toBeBool()->toBe(sort($b));
    expect($x)->and(sort($a))->toBe(sort($b));
    expect(sort($a), 'x')->toBe(sort($b));
    expect(sort($a))
        ->toBe(sort($b));
    expect(sort($a))->toBe(expected: sort($b));
    expect(rsort($a))->toBe(rsort($b));
    expect(sort(array: $a))->toBe(sort(array: $b));
});
-----
<?php

test('a', function ($a, $b) {
    expect($a)->toEqualCanonicalizing($b);
    expect($a)->toEqualCanonicalizing($b);
    expect(sort($a))->toBe($b);
    expect($a)->toBe(sort($b));
    expect(sort($a, SORT_STRING))->toBe(sort($b));
    expect(sort($a))->toBe(sort($b, SORT_STRING));
    expect($a)->toEqualCanonicalizing($b);
    expect(sort($a))->toBe(sort($b), 'x');
    expect($a)->not->toEqualCanonicalizing($b);
    expect($a)->toBeBool()->toEqualCanonicalizing($b);
    expect($x)->and($a)->toEqualCanonicalizing($b);
    expect($a)->toEqualCanonicalizing($b);
    expect($a)
        ->toEqualCanonicalizing($b);
    expect($a)->toEqualCanonicalizing($b);
    expect(rsort($a))->toBe(rsort($b));
    expect($a)->toEqualCanonicalizing($b);
});
