<?php

test('b', function (array $list, string $s, $untyped, object $o) {
    expect($list)->toHaveProperty('a')->toHaveProperty('b');
    expect($s)->toHaveProperty('a')->toHaveProperty('b');
    expect($untyped)->toHaveProperty('a')->toHaveProperty('b');
    expect($o)->toHaveProperty('a')->toHaveProperty('b');
    expect($list)->toHaveProperty('a');
    expect($list)->toHaveProperty('a')->toHaveProperty($b);
    expect($list)->toHaveProperty(1)->toHaveProperty('b');
    expect($list)->toHaveProperty('a')->toHaveProperty('b', 1);
    expect($list)->toHaveProperty('a', 1)->toHaveProperty('b')->toHaveProperty('c');
    expect($list)->not->toHaveProperty('a')->toHaveProperty('b');
    expect($list)->toHaveProperty('a')->not->toHaveProperty('b');
    expect($list)->toHaveProperty('a')->toHaveProperty('b')->toHaveProperty('c');
    expect($list)->TOHAVEPROPERTY('a')->toHaveProperty("b");
    expect($list)->toHaveProperty(name: 'a')->toHaveProperty('b');
    expect($list)
        ->toHaveProperty('a') // first
        ->toHaveProperty('b');
    expect($list)->toBeArray()->toHaveProperty('a')->toHaveProperty('b');
    expect($x)->and($list)->toHaveProperty('a')->toHaveProperty('b');
    expect($list)->toHaveProperty('a')->toHaveProperty('b')->toBeArray();
    expect($list)->toHaveProperty('a')->toHaveProperty("b$c");
    expect([new Foo()])->toHaveProperty('a')->toHaveProperty('b');
    expect($list)->toHaveProperty(('a'))->toHaveProperty('b');
});
-----
<?php

test('b', function (array $list, string $s, $untyped, object $o) {
    expect($list)->toHaveProperty('a')->toHaveProperty('b');
    expect($s)->toHaveProperty('a')->toHaveProperty('b');
    expect($untyped)->toHaveProperties(['a', 'b']);
    expect($o)->toHaveProperties(['a', 'b']);
    expect($list)->toHaveProperty('a');
    expect($list)->toHaveProperty('a')->toHaveProperty($b);
    expect($list)->toHaveProperty(1)->toHaveProperty('b');
    expect($list)->toHaveProperty('a')->toHaveProperty('b', 1);
    expect($list)->toHaveProperty('a', 1)->toHaveProperty('b')->toHaveProperty('c');
    expect($list)->not->toHaveProperty('a')->toHaveProperty('b');
    expect($list)->toHaveProperty('a')->not->toHaveProperty('b');
    expect($list)->toHaveProperty('a')->toHaveProperty('b')->toHaveProperty('c');
    expect($list)->TOHAVEPROPERTY('a')->toHaveProperty("b");
    expect($list)->toHaveProperty(name: 'a')->toHaveProperty('b');
    expect($list)
        ->toHaveProperty('a') // first
        ->toHaveProperty('b');
    expect($list)->toBeArray()->toHaveProperty('a')->toHaveProperty('b');
    expect($x)->and($list)->toHaveProperty('a')->toHaveProperty('b');
    expect($list)->toHaveProperty('a')->toHaveProperty('b')->toBeArray();
    expect($list)->toHaveProperty('a')->toHaveProperty("b$c");
    expect([new Foo()])->toHaveProperty('a')->toHaveProperty('b');
    expect($list)->toHaveProperty(('a'))->toHaveProperty('b');
});
