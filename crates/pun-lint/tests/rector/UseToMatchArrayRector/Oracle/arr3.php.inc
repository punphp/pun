<?php

test('c1', function () {
    // first
    expect($a['x'])->toBe(1);
    expect($a['y'])->toBe(2);
    foo();
});

test('c2', function () {
    expect($a['x'])->toBe(1);
    // second
    expect($a['y'])->toBe(2);
    foo();
});

test('c3', function () {
    foo();
    expect($a['x'])->toBe(1);
    // third
    expect($a['y'])->toBe(2);
});

test('c4', function () {
    expect($b['x'])->toBe(1);
    expect($b['y'])->toBe(2);
    // fourth
    expect($a['x'])->toBe(1);
    expect($a['y'])->toBe(2);
    foo();
});
-----
<?php

test('c1', function () {
    // first
    expect($a)->toMatchArray(['x' => 1, 'y' => 2]);
    foo();
});

test('c2', function () {
    // second
    expect($a)->toMatchArray(['x' => 1, 'y' => 2]);
    foo();
});

test('c3', function () {
    foo();
    // third
    expect($a)->toMatchArray(['x' => 1, 'y' => 2]);
});

test('c4', function () {
    expect($b)->toMatchArray(['x' => 1, 'y' => 2]);
    // fourth
    expect($a)->toMatchArray(['x' => 1, 'y' => 2]);
    foo();
});
