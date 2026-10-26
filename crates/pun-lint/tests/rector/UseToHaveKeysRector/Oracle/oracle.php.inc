<?php

test('a', function (array $list, string $s, $untyped, object $o) {
    expect($list)->toHaveKey('a')->toHaveKey('b');
    expect($s)->toHaveKey('a')->toHaveKey('b');
    expect($untyped)->toHaveKey('a')->toHaveKey('b');
    expect($o)->toHaveKey('a')->toHaveKey('b');
    expect($list)->toHaveKey('a');
    expect($list)->toHaveKey('a')->toHaveKey($b);
    expect($list)->toHaveKey(1)->toHaveKey('b');
    expect($list)->toHaveKey('a')->toHaveKey('b', 1);
    expect($list)->toHaveKey('a', 1)->toHaveKey('b')->toHaveKey('c');
    expect($list)->not->toHaveKey('a')->toHaveKey('b');
    expect($list)->toHaveKey('a')->not->toHaveKey('b');
    expect($list)->toHaveKey('a')->toHaveKey('b')->toHaveKey('c');
    expect($list)->TOHAVEKEY('a')->toHaveKey("b");
    expect($list)->toHaveKey(key: 'a')->toHaveKey('b');
    expect($list)
        ->toHaveKey('a') // first
        ->toHaveKey('b');
    expect($list)->toBeArray()->toHaveKey('a')->toHaveKey('b');
    expect($x)->and($list)->toHaveKey('a')->toHaveKey('b');
    expect($list)->toHaveKey('a')->toHaveKey('b')->toBeArray();
    expect($list)->toHaveKey('a')->toHaveKey("b$c");
    expect(['a' => 1])->toHaveKey('a')->toHaveKey('b');
    expect($list)->toHaveKey(('a'))->toHaveKey('b');
});
-----
<?php

test('a', function (array $list, string $s, $untyped, object $o) {
    expect($list)->toHaveKeys(['a', 'b']);
    expect($s)->toHaveKey('a')->toHaveKey('b');
    expect($untyped)->toHaveKeys(['a', 'b']);
    expect($o)->toHaveKey('a')->toHaveKey('b');
    expect($list)->toHaveKey('a');
    expect($list)->toHaveKey('a')->toHaveKey($b);
    expect($list)->toHaveKey(1)->toHaveKey('b');
    expect($list)->toHaveKey('a')->toHaveKey('b', 1);
    expect($list)->toHaveKey('a', 1)->toHaveKey('b')->toHaveKey('c');
    expect($list)->not->toHaveKey('a')->toHaveKey('b');
    expect($list)->toHaveKey('a')->not->toHaveKey('b');
    expect($list)->toHaveKeys(['a', 'b', 'c']);
    expect($list)->toHaveKeys(['a', "b"]);
    expect($list)->toHaveKeys(['a', 'b']);
    expect($list)
        ->toHaveKeys(['a', 'b']);
    expect($list)->toBeArray()->toHaveKeys(['a', 'b']);
    expect($x)->and($list)->toHaveKeys(['a', 'b']);
    expect($list)->toHaveKeys(['a', 'b'])->toBeArray();
    expect($list)->toHaveKey('a')->toHaveKey("b$c");
    expect(['a' => 1])->toHaveKeys(['a', 'b']);
    expect($list)->toHaveKeys(['a', 'b']);
});
