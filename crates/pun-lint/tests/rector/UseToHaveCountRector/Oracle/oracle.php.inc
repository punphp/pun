<?php

test('a', function (array $list, iterable $it, string $s, $untyped, \Countable $c, \ArrayObject $ao) {
    expect(count($list))->toBe(3);
    expect(count($it))->toBe(3);
    expect(count($s))->toBe(3);
    expect(count($untyped))->toEqual(3);
    expect(count($c))->toBe(3);
    expect(count($ao))->toBe(3);
    expect(sizeof($list))->toBe(3);
    expect(COUNT($list))->toBe(3);
    expect(\count($list))->toBe(3);
    expect(count($list, COUNT_RECURSIVE))->toBe(3);
    expect(count($list))->toBe(3, 'message');
    expect(count($list))->toBe();
    expect(count($list))->not->toBe(3);
    expect(count($list))->TOBE(3);
    expect(count($list))->toBeInt()->toBe(3);
    expect($a)->and(count($list))->toBe(3);
    expect(count($list), 'x')->toBe(3);
    expect(count(value: $list))->toBe(3);
    expect(count($list))
        ->toBe(3);
    expect(count([1, 2]))->toBe(2);
    expect(count($list))->toBeGreaterThan(3);
});
-----
<?php

test('a', function (array $list, iterable $it, string $s, $untyped, \Countable $c, \ArrayObject $ao) {
    expect($list)->toHaveCount(3);
    expect($it)->toHaveCount(3);
    expect(count($s))->toBe(3);
    expect($untyped)->toHaveCount(3);
    expect($c)->toHaveCount(3);
    expect($ao)->toHaveCount(3);
    expect($list)->toHaveCount(3);
    expect($list)->toHaveCount(3);
    expect($list)->toHaveCount(3);
    expect(count($list, COUNT_RECURSIVE))->toBe(3);
    expect(count($list))->toBe(3, 'message');
    expect(count($list))->toBe();
    expect($list)->not->toHaveCount(3);
    expect($list)->toHaveCount(3);
    expect($list)->toBeInt()->toHaveCount(3);
    expect($a)->and($list)->toHaveCount(3);
    expect($list, 'x')->toHaveCount(3);
    expect($list)->toHaveCount(3);
    expect($list)
        ->toHaveCount(3);
    expect([1, 2])->toHaveCount(2);
    expect(count($list))->toBeGreaterThan(3);
});
