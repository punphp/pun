<?php

test('a', function (string $s, int $i, $untyped, ?string $maybe) {
    expect(strlen($s))->toBe(5);
    expect(mb_strlen($s))->toEqual(5);
    expect(mb_strlen($s, 'UTF-8'))->toBe(5);
    expect(strlen($i))->toBe(5);
    expect(strlen($untyped))->toBe(5);
    expect(strlen($maybe))->toBe(5);
    expect(STRLEN($s))->toBe(5);
    expect(\strlen($s))->toBe(5);
    expect(strlen())->toBe(5);
    expect(strlen($s))->TOBE(5);
    expect(strlen($s))->toBe(5, 'x');
    expect(strlen($s))->toBe(expected: 5);
    expect(strlen($s))->not->toBe(5);
    expect(strlen($s))->toBeInt()->toBe(5);
    expect($a)->and(strlen($s))->toBe(5);
    expect(strlen($s), 'x')->toBe(5);
    expect(strlen(string: $s))->toBe(5);
    expect(strlen($s))
        ->toBe(5);
    expect(strlen('abc'))->toBe(3);
    expect(strlen(123))->toBe(3);
});
-----
<?php

test('a', function (string $s, int $i, $untyped, ?string $maybe) {
    expect($s)->toHaveLength(5);
    expect($s)->toHaveLength(5);
    expect($s)->toHaveLength(5);
    expect(strlen($i))->toBe(5);
    expect(strlen($untyped))->toBe(5);
    expect(strlen($maybe))->toBe(5);
    expect(STRLEN($s))->toBe(5);
    expect($s)->toHaveLength(5);
    expect(strlen())->toBe(5);
    expect(strlen($s))->TOBE(5);
    expect(strlen($s))->toBe(5, 'x');
    expect($s)->toHaveLength(5);
    expect($s)->not->toHaveLength(5);
    expect($s)->toBeInt()->toHaveLength(5);
    expect($a)->and($s)->toHaveLength(5);
    expect($s, 'x')->toHaveLength(5);
    expect($s)->toHaveLength(5);
    expect($s)
        ->toHaveLength(5);
    expect('abc')->toHaveLength(3);
    expect(strlen(123))->toBe(3);
});
