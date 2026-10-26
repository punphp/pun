<?php

test('t', function () {
    // q value
    expect($q)->toBe(1);
    expect($q)->toBeInt();
    expect($q)->toBeString();
    expect($p)->toBe(1);
    expect($p)->toBe(2);
    expect($p)->toBeInt();
});
-----
<?php

test('t', function () {
    expect($q)->toBeInt();
    expect($q)->toBeString();
    // q value
    expect($q)->toBe(1);
    expect($p)->toBeInt();
    expect($p)->toBe(1);
    expect($p)->toBe(2);
});
