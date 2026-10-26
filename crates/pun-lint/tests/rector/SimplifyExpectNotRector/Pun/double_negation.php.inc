<?php

test('a', function (bool $flag) {
    expect(!!$flag)->toBeTrue();
    expect(!!!$flag)->toBeFalse();
});
-----
<?php

test('a', function (bool $flag) {
    expect($flag)->toBeTrue();
    expect($flag)->toBeTrue();
});
