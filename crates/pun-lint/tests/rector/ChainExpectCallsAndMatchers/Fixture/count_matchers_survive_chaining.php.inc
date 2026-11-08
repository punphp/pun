<?php

test('counts', function () {
    expect(count($a))->toBe(1);
    expect(count($b))->toBe(2);
});

?>
-----
<?php

test('counts', function () {
    expect(count($a))->toBe(1)
        ->and(count($b))->toBe(2);
});

?>
