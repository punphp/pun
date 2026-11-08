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
