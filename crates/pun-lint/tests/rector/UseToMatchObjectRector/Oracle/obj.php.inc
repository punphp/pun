<?php

final class User
{
}

test('a', function () {
    $user = new User();

    // c1
    expect($user)->toHaveProperty('name', 'Nuno');
    /* c2 */
    expect($user)->toHaveProperty('email', value: 'n@e.com');
    expect($user)->toHaveProperty('age', 3);

    expect(1)->toBe(1);
});

test('b', function () {
    $s = 'string';
    expect($s)->toHaveProperty('a', 1);
    expect($s)->toHaveProperty('b', 2);
});

test('c', function () {
    expect($u)->toBe($u)->toHaveProperty('a', 1);
    expect($u)->toHaveProperty('b', [
        'x' => 1,
    ]);
    $x = 1;
    expect($u)->toHaveProperty('a', 1);
    expect($u)->TOHAVEPROPERTY('b', ...$c);
    expect($u)->toHaveProperty('c');
});
-----
<?php

final class User
{
}

test('a', function () {
    $user = new User();
    // c1
    /* c2 */
    expect($user)->toMatchObject(['name' => 'Nuno', 'email' => 'n@e.com', 'age' => 3]);

    expect(1)->toBe(1);
});

test('b', function () {
    $s = 'string';
    expect($s)->toHaveProperty('a', 1);
    expect($s)->toHaveProperty('b', 2);
});

test('c', function () {
    expect($u)->toBe($u)->toHaveProperty('a', 1);
    expect($u)->toHaveProperty('b', [
        'x' => 1,
    ]);
    $x = 1;
    expect($u)->toHaveProperty('a', 1);
    expect($u)->TOHAVEPROPERTY('b', ...$c);
    expect($u)->toHaveProperty('c');
});
