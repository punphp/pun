<?php

test('a', function () {
    expect($a['x'])->toBe(1);
    // second
    expect($a['y'])->toBe(2);
});
