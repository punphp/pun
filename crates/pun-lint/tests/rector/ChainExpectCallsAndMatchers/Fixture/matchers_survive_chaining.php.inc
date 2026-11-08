<?php

test('user', function () {
    expect(strlen($user->name))->toBe(7);
    expect(count($roles))->toBe(3);
    expect(in_array('admin', $roles))->toBeTrue();
    expect($user->id > 0)->toBeTrue();
    expect($user->created_at instanceof DateTimeInterface)->toBeTrue();
});

?>
-----
<?php

test('user', function () {
    expect(strlen($user->name))->toBe(7)
        ->and(count($roles))->toBe(3)
        ->and(in_array('admin', $roles))->toBeTrue()
        ->and($user->id > 0)->toBeTrue()
        ->and($user->created_at instanceof DateTimeInterface)->toBeTrue();
});

?>
