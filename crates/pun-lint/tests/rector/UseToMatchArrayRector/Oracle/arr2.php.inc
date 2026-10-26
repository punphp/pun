<?php

test('a', function () {
    expect($a['x'])->toBe(1);
    // second
    expect($a['y'])->toBe(2);
});
-----
<?php

test('a', function () {
    // second
    expect($a)->toMatchArray(['x' => 1, 'y' => 2]);
});
