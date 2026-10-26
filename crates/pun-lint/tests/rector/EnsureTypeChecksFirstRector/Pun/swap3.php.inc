<?php

test('r', function () {
    expect($q)->toBe(1);
    // int
    expect($q)->toBeInt();
    /* string */
    expect($q)->toBeString();
});

test('s', function () {
    // value
    expect($q)->toBe(1);
    expect($q)->toBeInt();
    expect($q)->toBeString();
});
-----
<?php

test('r', function () {
    // int
    expect($q)->toBeInt();
    /* string */
    expect($q)->toBeString();
    expect($q)->toBe(1);
});

test('s', function () {
    expect($q)->toBeInt();
    expect($q)->toBeString();
    // value
    expect($q)->toBe(1);
});
