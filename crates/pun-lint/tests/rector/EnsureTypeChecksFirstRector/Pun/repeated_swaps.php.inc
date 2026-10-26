<?php

test('a', function () {
    $x = 1;

    // first
    expect($x)->toBe(1);
    /* second */
    expect($x)->toBeInt();

    expect($y)->toBe(1)->not->toBeNull();
    expect($y)->toBe(1)
        ->toBeString(
            // c
            'x',
        );
    expect($z)->toBe(2);
    expect($z)->not->toBeInt();
    expect($z)->toBeFoo();
    expect($z)->toBeInt();
    expect($w)->dump()->toBe(1)->toBeInt();
    expect($w)->toBe(1)->each->toBeInt();
    expect($w)->toBe(1)->$m()->toBeInt();
    expect($w)->toBe(1)->{'toBeInt'}();
    expect($w)->TOBE(1)->toBeInt();
    expect($w)->toBe(1)->and($v)->toBe(2)->toBeArray();
    expect($w)->toBe(1)->toBeInt()->and($v)->toBe(2);
    expect($q)->toBe(1);
    expect($q)->toBeInt();
    expect($q)->toBeString();
    expect($p)->toBe(1);
    expect($p)->toBe(2);
    expect($p)->toBeInt();
});

expect($a)->toBe(1);
expect($a)->toBeInt();
-----
<?php

test('a', function () {
    $x = 1;
    /* second */
    expect($x)->toBeInt();
    // first
    expect($x)->toBe(1);

    expect($y)->not->toBeNull()->toBe(1);
    expect($y)->toBeString(
        // c
        'x'
    )->toBe(1);
    expect($z)->toBe(2);
    expect($z)->not->toBeInt();
    expect($z)->toBeInt();
    expect($z)->toBeFoo();
    expect($w)->dump()->toBe(1)->toBeInt();
    expect($w)->toBe(1)->each->toBeInt();
    expect($w)->toBe(1)->$m()->toBeInt();
    expect($w)->TOBE(1)->toBeInt();
    expect($w)->toBe(1)->and($v)->toBeArray()->toBe(2);
    expect($w)->toBeInt()->toBe(1)->and($v)->toBe(2);
    expect($w)->toBe(1)->{'toBeInt'}();
    expect($q)->toBeInt();
    expect($q)->toBeString();
    expect($q)->toBe(1);
    expect($p)->toBeInt();
    expect($p)->toBe(1);
    expect($p)->toBe(2);
});
expect($a)->toBeInt();
expect($a)->toBe(1);
