<?php

test('a', function (/* first */ int $a, string $b) {
    expect($i)->toBe(1);
    expect($i)->toBe(2) ?>
<?php
    expect($i)->toBe(3);
});
-----
<?php

test('a', function (
    /* first */
    int $a,
    string $b
) {
    expect($i)->toBe(1)
        ->toBe(2)
        ->toBe(3);
});
