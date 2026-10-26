<?php

test('t', function () {
    expect($i)->toBe(1);

    expect($i)->toBe(2) ?>
<?php
    expect($i)->toBe(3);
    $x = 1;

    expect($m)->toBe(2); // trailing
    expect($m)->toBe(3);
});
-----
<?php

test('t', function () {
    expect($i)->toBe(1)
        ->toBe(2)
        ->toBe(3);
    $x = 1;
    expect($m)->toBe(2);
    // trailing
    expect($m)->toBe(3);
});
