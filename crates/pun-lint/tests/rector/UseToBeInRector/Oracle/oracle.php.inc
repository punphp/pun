<?php

test('a', function (array $good, string $bad, iterable $it, $untyped) {
    expect(in_array('x', $good))->toBeTrue();
    expect(in_array('x', $bad))->toBeTrue();
    expect(in_array('x', $it))->toBeTrue();
    expect(in_array('x', $untyped))->toBeFalse();
    expect(in_array('x', $untyped, true))->not->toBeFalse();
    expect(in_array('x', $untyped, false))->toBeTrue();
    expect(in_array('x', $untyped, FALSE))->toBeFalse();
    expect(in_array('x', $untyped, \false))->toBeTrue();
    expect(in_array('x', $untyped, 0))->toBeTrue();
    expect(in_array('x', $untyped, $strict))->toBeTrue();
    expect(in_array('x', $untyped, true, 4))->toBeTrue();
    expect(in_array('x'))->toBeTrue();
    expect(in_array(needle: 'x', haystack: $untyped))->toBeTrue();
    expect($a)->and(in_array('x', [1, 2]))->not->toBeTrue();
    expect(in_array('x', $untyped))
        ->toBeTrue()
        ->and($b)->toBeFalse();
});

test('b', function (array $good, string $bad, iterable $it, \Traversable $t, \Countable $c, $untyped) {
    expect(in_array('x', $good, true))->toBeTrue();
    expect(in_array('x', $bad, true))->toBeTrue();
    expect(in_array('x', $it, true))->toBeTrue();
    expect(in_array('x', $t, true))->toBeTrue();
    expect(in_array('x', $c, true))->toBeTrue();
    expect(in_array('x', 'str', true))->toBeTrue();
    expect(in_array('x', $untyped, TRUE))->toBeTrue();
    expect(in_array('x', $untyped, true))->toBeFalse();
    expect(in_array($x, $untyped, true))->not->toBeTrue();
});
-----
<?php

test('a', function (array $good, string $bad, iterable $it, $untyped) {
    expect(in_array('x', $good))->toBeTrue();
    expect(in_array('x', $bad))->toBeTrue();
    expect(in_array('x', $it))->toBeTrue();
    expect(in_array('x', $untyped))->toBeFalse();
    expect('x')->toBeIn($untyped);
    expect(in_array('x', $untyped, false))->toBeTrue();
    expect(in_array('x', $untyped, FALSE))->toBeFalse();
    expect(in_array('x', $untyped, \false))->toBeTrue();
    expect(in_array('x', $untyped, 0))->toBeTrue();
    expect(in_array('x', $untyped, $strict))->toBeTrue();
    expect(in_array('x', $untyped, true, 4))->toBeTrue();
    expect(in_array('x'))->toBeTrue();
    expect(in_array(needle: 'x', haystack: $untyped))->toBeTrue();
    expect($a)->and(in_array('x', [1, 2]))->not->toBeTrue();
    expect(in_array('x', $untyped))
        ->toBeTrue()
        ->and($b)->toBeFalse();
});

test('b', function (array $good, string $bad, iterable $it, \Traversable $t, \Countable $c, $untyped) {
    expect('x')->toBeIn($good);
    expect(in_array('x', $bad, true))->toBeTrue();
    expect('x')->toBeIn($it);
    expect('x')->toBeIn($t);
    expect('x')->toBeIn($c);
    expect(in_array('x', 'str', true))->toBeTrue();
    expect('x')->toBeIn($untyped);
    expect('x')->not->toBeIn($untyped);
    expect($x)->not->toBeIn($untyped);
});
