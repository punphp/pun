<?php

test('a', function () {
    expect($a)->tap(fn () => 1)->toBe(1);
    expect($a)->not->TAP(fn () => 1);
    expect($a)
        ->toBe(1)
        ->tap(function () {})
        ->tap(fn () => 2);
    expect($a)->and($b)->tap($c);
    $a->tap($b);
    tap($a)->tap($b);
    expect($a)->{'tap'}($b);
    expect($a)?->tap($b);
    expect($a)->tap;
    expect(fn () => expect($b)->tap($c))->tap($d);
    \expect($a)->tap($b);
});
-----
<?php

test('a', function () {
    expect($a)->defer(fn () => 1)->toBe(1);
    expect($a)->not->defer(fn () => 1);
    expect($a)
        ->toBe(1)
        ->defer(function () {})
        ->defer(fn () => 2);
    expect($a)->and($b)->defer($c);
    $a->tap($b);
    tap($a)->tap($b);
    expect($a)->{'tap'}($b);
    expect($a)?->tap($b);
    expect($a)->tap;
    expect(fn () => expect($b)->defer($c))->defer($d);
    \expect($a)->defer($b);
});
