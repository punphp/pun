<?php

test('a', function () {
    expect($a)->toBe(1) ?>
<?php
    expect($a)->toBeInt();
});
-----
<?php

test('a', function () {
    expect($a)->toBeInt();
    expect($a)->toBe(1) ?>

});
