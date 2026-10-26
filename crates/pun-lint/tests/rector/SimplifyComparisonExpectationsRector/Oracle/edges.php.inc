<?php

test('edges', function () {
    $x->toBeTrue();
    $x->toBeFalse();
    $x->not->toBeTrue();
    $x->not->toBeFalse();
    $x->toBe(0);
    $x->toEqual(0);
    $x->toBe(1);
    $x->toBeGreaterThan(0);
    $x->toHaveMethod('a');
    $x->toHaveKey('a')->toHaveKey('b');
    $x->toHaveProperty('a')->toHaveProperty('b');
    $x->toHaveCount(0);
    expect()->toBeTrue();
    expect()->toBeFalse();
    expect()->not->toBeTrue();
    expect()->not->toBeFalse();
    expect()->toBe(0);
    expect()->toEqual(0);
    expect()->toBe(1);
    expect()->toBeGreaterThan(0);
    expect()->toHaveMethod('a');
    expect()->toHaveKey('a')->toHaveKey('b');
    expect()->toHaveProperty('a')->toHaveProperty('b');
    expect($a)->and()->toBeTrue();
    expect($a)->and()->toBe(0);
    expect($a)->and()->toBe(1);
    expect($a)->and()->toEqual(0);
    expect($a)->and()->toBeGreaterThan(0);
    (expect(1))->and($a > 1, 'x')->toBeTrue();
    expect(new stdClass())->toHaveProperty('a')->toHaveProperty($k);
    expect(filter_var($u, 273) !== false)->toBeTrue();
});

/**
 * @template T
 *
 * @param numeric $n
 * @param scalar $s
 * @param array-key $k
 * @param T $t
 */
function lengths($n, $s, $k, $t)
{
    expect(strlen($n))->toBe(1);
    expect(strlen($s))->toBe(1);
    expect(strlen($k))->toBe(1);
    expect(strlen($t))->toBe(1);
}
-----
<?php

test('edges', function () {
    $x->toBeTrue();
    $x->toBeFalse();
    $x->not->toBeTrue();
    $x->not->toBeFalse();
    $x->toBe(0);
    $x->toEqual(0);
    $x->toBe(1);
    $x->toBeGreaterThan(0);
    $x->toHaveMethod('a');
    $x->toHaveKey('a')->toHaveKey('b');
    $x->toHaveProperty('a')->toHaveProperty('b');
    $x->toHaveCount(0);
    expect()->toBeTrue();
    expect()->toBeFalse();
    expect()->not->toBeTrue();
    expect()->not->toBeFalse();
    expect()->toBe(0);
    expect()->toEqual(0);
    expect()->toBe(1);
    expect()->toBeGreaterThan(0);
    expect()->toHaveMethod('a');
    expect()->toHaveKey('a')->toHaveKey('b');
    expect()->toHaveProperty('a')->toHaveProperty('b');
    expect($a)->and()->toBeTrue();
    expect($a)->and()->toBe(0);
    expect($a)->and()->toBe(1);
    expect($a)->and()->toEqual(0);
    expect($a)->and()->toBeGreaterThan(0);
    expect(1)->and($a)->toBeGreaterThan(1);
    expect(new stdClass())->toHaveProperty('a')->toHaveProperty($k);
    expect(filter_var($u, 273) !== false)->toBeTrue();
});

/**
 * @template T
 *
 * @param numeric $n
 * @param scalar $s
 * @param array-key $k
 * @param T $t
 */
function lengths($n, $s, $k, $t)
{
    expect(strlen($n))->toBe(1);
    expect(strlen($s))->toBe(1);
    expect(strlen($k))->toBe(1);
    expect(strlen($t))->toBe(1);
}
