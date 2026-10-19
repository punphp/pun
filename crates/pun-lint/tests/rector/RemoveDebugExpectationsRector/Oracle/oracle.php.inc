<?php

test('x', function () {
    expect($a)->dump()->toBe(1);
    expect($a)->toBe(1)->dd();
    expect($a)->ray();
    expect($a)
        ->dump()
        ->toBe(1)
        ->ray()
        ->toBeTrue();
    expect($a)->not->dump()->toBe(1);
    expect($a)->and($b)->dump()->toBe(2);
    $x->dump()->toBe(1);
    expect($a)->Dump()->toBe(1);
    expect($a)->{'dump'}()->toBe(1);
    \expect($a)->dump()->dd()->toBe(1)->ray();
    $y = expect($a)->dump();
    expect(foo()->dump())->toBe(1);
    expect($a)->each->dump()->toBe(1);
    expect($a) // comment
        ->dump() // another
        ->toBe(1);
});
-----
<?php

test('x', function () {
    expect($a)->toBe(1);
    expect($a)->toBe(1);
    expect($a);
    expect($a)
        ->toBe(1)
        ->toBeTrue();
    expect($a)->not->toBe(1);
    expect($a)->and($b)->toBe(2);
    $x->dump()->toBe(1);
    expect($a)->Dump()->toBe(1);
    expect($a)->{'dump'}()->toBe(1);
    \expect($a)->toBe(1);
    $y = expect($a);
    expect(foo()->dump())->toBe(1);
    expect($a)->each->toBe(1);
    expect($a) // another
        ->toBe(1);
});
