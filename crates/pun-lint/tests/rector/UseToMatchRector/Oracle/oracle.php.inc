<?php

test('a', function (string $s, int $i, $untyped) {
    expect(preg_match('/a/', $s))->toBe(1);
    expect(preg_match('/a/', $s))->toEqual(1);
    expect(preg_match('/a/', $s))->toBe(0);
    expect(preg_match('/a/', $s))->toBe(true);
    expect(preg_match('/a/', $s))->toBe( 1 );
    expect(preg_match('/a/', $s))->toBe(expected: 1);
    expect(preg_match('/a/', $s))->toBe(1, 'x');
    expect(preg_match('/a/', $i))->toBe(1);
    expect(preg_match('/a/', $untyped))->toBe(1);
    expect(preg_match($pattern, $s, $m))->toBe(1);
    expect(preg_match(pattern: '/a/', subject: $s))->toBe(1);
    expect(PREG_MATCH('/a/', $s))->TOBE(1);
    expect(\preg_match('/a/', $s))->toBe(1);
    expect(preg_match('/a/'))->toBe(1);
    expect(preg_match('/a/', $s))->not->toBe(1);
    expect(preg_match('/a/', $s))->toBeInt()->toBe(1);
    expect($a)->and(preg_match('/a/', $s))->toBe(1);
    expect(preg_match('/a/', $s), 'x')->toBe(1);
    expect(preg_match('/a/', $s))
        ->toBe(1);
    expect(preg_match('/a/', 'abc'))->toBe(1);
    expect(preg_match_all('/a/', $s))->toBe(1);
});
-----
<?php

test('a', function (string $s, int $i, $untyped) {
    expect($s)->toMatch('/a/');
    expect($s)->toMatch('/a/');
    expect(preg_match('/a/', $s))->toBe(0);
    expect(preg_match('/a/', $s))->toBe(true);
    expect($s)->toMatch( '/a/' );
    expect($s)->toMatch('/a/');
    expect(preg_match('/a/', $s))->toBe(1, 'x');
    expect(preg_match('/a/', $i))->toBe(1);
    expect($untyped)->toMatch('/a/');
    expect($s)->toMatch($pattern);
    expect($s)->toMatch('/a/');
    expect($s)->toMatch('/a/');
    expect($s)->toMatch('/a/');
    expect(preg_match('/a/'))->toBe(1);
    expect($s)->not->toMatch('/a/');
    expect($s)->toBeInt()->toMatch('/a/');
    expect($a)->and($s)->toMatch('/a/');
    expect($s)->toMatch('/a/');
    expect($s)
        ->toMatch('/a/');
    expect('abc')->toMatch('/a/');
    expect(preg_match_all('/a/', $s))->toBe(1);
});
