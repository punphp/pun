<?php

test('a', function () {
    $data = ['a' => 1, 'b' => 2];

    expect($data['a'])->toBe(1);
    expect($data['b'])->toEqual(2);
    expect($data['c'])->toBe(3, 'msg');
    expect($data['d'])->toBe(4);
    expect($data['e'])->toBe(5);
    // end
});

test('b', function () {
    expect($r->json()['a'], 'm')->toBe(1);
    expect($r->json()['b'])->TOBE(2);
});

test('c', function () {
    $n = 5;
    expect($n['a'])->toBe(1);
    expect($n['b'])->toBe(2);
});

test('d', function () {
    expect($data['a'])->not->toBe(1);
    expect($data['a'])->toBe(1);
    expect($data['b'])->toBe(2);
});
-----
<?php

test('a', function () {
    $data = ['a' => 1, 'b' => 2];

    expect($data['a'])->toBe(1);
    expect($data['b'])->toEqual(2);
    expect($data['c'])->toBe(3, 'msg');
    expect($data)->toMatchArray(['d' => 4, 'e' => 5]);
    // end
});

test('b', function () {
    expect($r->json()['a'], 'm')->toBe(1);
    expect($r->json()['b'])->TOBE(2);
});

test('c', function () {
    $n = 5;
    expect($n['a'])->toBe(1);
    expect($n['b'])->toBe(2);
});

test('d', function () {
    expect($data['a'])->not->toBe(1);
    expect($data['a'])->toBe(1);
    expect($data['b'])->toBe(2);
});
