<?php

test('a', function (bool $flag, array $list, string $s, $untyped, ?array $maybe) {
    expect($untyped)->toBe(true);
    expect($untyped)->toBe(false);
    expect($untyped)->toBe(null);
    expect($untyped)->toBe(TRUE);
    expect($untyped)->toBe(\false);
    expect($untyped)->toBe(\NULL);
    expect($untyped)->toEqual(true);
    expect($flag)->toEqual(true);
    expect($flag)->toEqual(False);
    expect($untyped)->toEqual(null);
    expect($list)->toBe([]);
    expect($list)->toEqual(array());
    expect($untyped)->toBe([]);
    expect($maybe)->toBe([]);
    expect($list)->toBe([1]);
    expect($s)->toBe('');
    expect($s)->toEqual("");
    expect($untyped)->toBe('');
    expect($s)->toBe('a');
    expect($untyped)->toBe(true, 'message');
    expect($untyped)->toBe(expected: true);
    expect($untyped)->toBe(
        true,
    );
    expect($untyped)->not->toBe(true);
    expect($a)->and($flag)->toEqual(true);
    expect($a)->and($untyped)->toEqual(true);
    expect($untyped)->TOBE(true);
    expect($untyped)->toBe(1);
    expect($untyped)->toBe(SOME_CONSTANT);
    expect($untyped)
        ->toBe(true)
        ->toBe(null);
});
-----
<?php

test('a', function (bool $flag, array $list, string $s, $untyped, ?array $maybe) {
    expect($untyped)->toBeTrue();
    expect($untyped)->toBeFalse();
    expect($untyped)->toBeNull();
    expect($untyped)->toBeTrue();
    expect($untyped)->toBeFalse();
    expect($untyped)->toBeNull();
    expect($untyped)->toEqual(true);
    expect($flag)->toBeTrue();
    expect($flag)->toBeFalse();
    expect($untyped)->toEqual(null);
    expect($list)->toBeEmpty();
    expect($list)->toBeEmpty();
    expect($untyped)->toBe([]);
    expect($maybe)->toBe([]);
    expect($list)->toBe([1]);
    expect($s)->toBeEmpty();
    expect($s)->toBeEmpty();
    expect($untyped)->toBe('');
    expect($s)->toBe('a');
    expect($untyped)->toBe(true, 'message');
    expect($untyped)->toBeTrue();
    expect($untyped)->toBeTrue();
    expect($untyped)->not->toBeTrue();
    expect($a)->and($flag)->toBeTrue();
    expect($a)->and($untyped)->toEqual(true);
    expect($untyped)->toBeTrue();
    expect($untyped)->toBe(1);
    expect($untyped)->toBe(SOME_CONSTANT);
    expect($untyped)->toBeTrue()->toBeNull();
});
