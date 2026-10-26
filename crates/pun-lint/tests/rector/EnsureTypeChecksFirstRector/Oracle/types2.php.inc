<?php

test('b', function () {
    expect($a)->toBe(1);
    // moved
    expect($a)->toBeInt();
});

test('c', function () {
    expect($x)->toBe(1)->not->and($y)->toBeInt()->toBe(2);
});
-----
<?php

test('b', function () {
    // moved
    expect($a)->toBeInt();
    expect($a)->toBe(1);
});

test('c', function () {
    expect($x)->toBe(1)->not->and($y)->toBeInt()->toBe(2);
});
