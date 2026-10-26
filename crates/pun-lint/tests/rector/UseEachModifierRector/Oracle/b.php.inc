<?php

test('b', function () {
    foreach ($items as $item) {
        expect($item)->toBe(
            // why
            1,
            /* other */ 2
        );
    }

    foreach ($items as $item) {
        expect(
            $item
        )->toBe(
            [
                'a' => 1,
            ]
        );
    }

    $x = 1; foreach (foo(
        1,
    ) as $item) {
        expect($item)->toBeInt();
    }
});
-----
<?php

test('b', function () {
    expect($items)->each->toBe(
        // why
        1,
        /* other */
        2
    );

    expect(
        $items
    )->each->toBe([
        'a' => 1,
    ]);

    $x = 1; expect(foo(
        1,
    ))->each->toBeInt();
});
