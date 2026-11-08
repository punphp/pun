<?php

test('a', function (array $good, string $bad, iterable $it, $untyped) {
    expect(in_array('x', $good))->toBeTrue();
    expect(in_array('x', $bad))->toBeTrue();
    expect(in_array('x', $it))->toBeTrue();
    expect(in_array('x', $untyped))->toBeFalse();
    expect(in_array('x', $untyped, true))->not->toBeFalse();
    expect(in_array('x', $untyped, false))->toBeTrue();
    expect(in_array('x', $untyped, FALSE))->toBeTrue();
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
-----
<?php

test('a', function (array $good, string $bad, iterable $it, $untyped) {
    expect($good)->toContain('x');
    expect(in_array('x', $bad))->toBeTrue();
    expect(in_array('x', $it))->toBeTrue();
    expect(in_array('x', $untyped))->toBeFalse();
    expect(in_array('x', $untyped, true))->not->toBeFalse();
    expect(in_array('x', $untyped, false))->toBeTrue();
    expect(in_array('x', $untyped, FALSE))->toBeTrue();
    expect(in_array('x', $untyped, \false))->toBeTrue();
    expect(in_array('x', $untyped, 0))->toBeTrue();
    expect(in_array('x', $untyped, $strict))->toBeTrue();
    expect(in_array('x', $untyped, true, 4))->toBeTrue();
    expect(in_array('x'))->toBeTrue();
    expect(in_array(needle: 'x', haystack: $untyped))->toBeTrue();
    expect($a)->and([1, 2])->not->toContain('x');
    expect(in_array('x', $untyped))
        ->toBeTrue()
        ->and($b)->toBeFalse();
});
