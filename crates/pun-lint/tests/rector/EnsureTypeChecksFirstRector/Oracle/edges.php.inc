<?php

test('a', function () {
    expect($x)->toBe(1)->toBeInt();
    // trailing
});

test('b', function () {
    expect($x)->toBe(1)->not->each->toBeInt();
    expect($y)->toBe(1)->toBeInt()->not;
});
-----
<?php

test('a', function () {
    expect($x)->toBeInt()->toBe(1);
    // trailing
});

test('b', function () {
    expect($x)->toBe(1)->not->each->toBeInt();
    expect($y)->toBe(1)->toBeInt()->not;
});
