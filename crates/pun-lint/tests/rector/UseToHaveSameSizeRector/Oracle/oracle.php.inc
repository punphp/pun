<?php

test('a', function (array $a, array $b, string $s, $untyped, iterable $it) {
    expect(count($a))->toBe(count($b));
    expect(sizeof($a))->toEqual(count($b));
    expect(count($s))->toBe(count($b));
    expect(count($untyped))->toBe(count($b));
    expect(count($it))->toBe(count($b));
    expect(count($a))->toBe(count($s));
    expect(count($a))->toBe(3);
    expect(count($a))->toBe(count($b), 'x');
    expect(count($a))->toBe(count());
    expect(count())->toBe(count($b));
    expect(count($a, 1))->toBe(count($b, 1));
    expect(COUNT($a))->TOBE(\count($b));
    expect(count($a))->not->toBe(count($b));
    expect(count($a))->toBeInt()->toBe(count($b));
    expect($x)->and(count($a))->toBe(count($b));
    expect(count($a), 'x')->toBe(count($b));
    expect(count($a))
        ->toBe(count($b));
    expect($a)->toHaveCount(count($b));
    expect($s)->toHaveCount(count($b));
    expect($untyped)->toHaveCount(count($b));
    expect($a)->toHaveCount(sizeof($b));
    expect($a)->toHaveCount(3);
    expect($a)->toHaveCount(count($b), 'x');
    expect($a)->toHaveCount(count());
    expect($a)->not->toHaveCount(count($b));
    expect($a)->TOHAVECOUNT(count($b));
    expect()->toHaveCount(count($b));
    expect($a)
        ->toHaveCount(count($b));
    expect($x)->and($a)->toHaveCount(count($b));
    expect($a)->toHaveCount(count(value: $b));
});
-----
<?php

test('a', function (array $a, array $b, string $s, $untyped, iterable $it) {
    expect($a)->toHaveSameSize($b);
    expect($a)->toHaveSameSize($b);
    expect(count($s))->toBe(count($b));
    expect(count($untyped))->toBe(count($b));
    expect($it)->toHaveSameSize($b);
    expect($a)->toHaveSameSize($s);
    expect(count($a))->toBe(3);
    expect(count($a))->toBe(count($b), 'x');
    expect(count($a))->toBe(count());
    expect(count())->toBe(count($b));
    expect($a)->toHaveSameSize($b);
    expect($a)->toHaveSameSize($b);
    expect($a)->not->toHaveSameSize($b);
    expect($a)->toBeInt()->toHaveSameSize($b);
    expect($x)->and($a)->toHaveSameSize($b);
    expect($a)->toHaveSameSize($b);
    expect($a)
        ->toHaveSameSize($b);
    expect($a)->toHaveSameSize($b);
    expect($s)->toHaveCount(count($b));
    expect($untyped)->toHaveCount(count($b));
    expect($a)->toHaveSameSize($b);
    expect($a)->toHaveCount(3);
    expect($a)->toHaveCount(count($b), 'x');
    expect($a)->toHaveCount(count());
    expect($a)->not->toHaveSameSize($b);
    expect($a)->toHaveSameSize($b);
    expect()->toHaveSameSize($b);
    expect($a)
        ->toHaveSameSize($b);
    expect($x)->and($a)->toHaveSameSize($b);
    expect($a)->toHaveSameSize($b);
});
