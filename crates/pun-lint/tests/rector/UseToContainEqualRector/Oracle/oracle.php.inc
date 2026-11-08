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

test('b', function (array $good, string $bad, iterable $it) {
    expect(in_array('x', $good, false))->toBeTrue();
    expect(in_array('x', $bad, false))->toBeTrue();
    expect(in_array('x', $it, false))->toBeTrue();
    expect(in_array('x', $untyped, false))->not->toBeFalse();
});
-----
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

test('b', function (array $good, string $bad, iterable $it) {
    expect($good)->toContainEqual('x');
    expect(in_array('x', $bad, false))->toBeTrue();
    expect(in_array('x', $it, false))->toBeTrue();
    expect(in_array('x', $untyped, false))->not->toBeFalse();
});
