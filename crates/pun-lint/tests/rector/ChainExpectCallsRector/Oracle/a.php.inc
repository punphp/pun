<?php

test('a', function () {
    expect($a)
        ->toBe(1);
    expect($a)
        ->toBeInt()
        ->toBe(function () {
            return 1;
        });
    expect($a)->toBe(
        // why
        1,
    );

    expect($b)->not->toBe(1);
    expect($b)->each->toBeInt()->{'toBe'}(1)->$m(2);
    expect($c)->toBe(1)->AND($d)->toBe(2);
    expect($c)->toBe(3);
    expect($e)->toBe(1);
    expect($f)->toBe(2)->and($g)->toBe(3);
    expect($f)->toBe(4);
    (expect($h)->toBe(1));
    expect($h)->toBe(2);
    expect($i,)->toBe(1);
    expect($i)->toBe(2);
    expect($i)->toBe(3);
    expect($j)?->toBe(1);
    expect($j)->toBe(2);
    expect(value: $k)->toBe(1);
    expect($k)->toBe(2);
    EXPECT($l)->toBe(1);
    expect($l)->toBe(2);
    expect($l)->not->not->toBe(2);
    $x = 1;
    expect($m)->toBe(1);
    expect($m)->toBe(2); // trailing
    expect($m)->toBe(3);
});
-----
<?php

test('a', function () {
    expect($a)
        ->toBe(1)
        ->toBeInt()
        ->toBe(function () {
        return 1;
    })
        ->toBe(
        // why
        1
    )
        ->and($b)->not->toBe(1)->each->toBeInt()
        ->{'toBe'}(1)
        ->{$m}(2)
        ->and($c)->toBe(1)
        ->AND($d)->toBe(2)
        ->toBe(3)
        ->and($e)->toBe(1)
        ->and($f)->toBe(2)
        ->and($g)->toBe(3)
        ->toBe(4)
        ->and($h)->toBe(1)
        ->toBe(2)
        ->and($i)->toBe(1)
        ->toBe(2)
        ->toBe(3);
    expect($j)?->toBe(1);
    expect($j)->toBe(2)
        ->and($k)->toBe(1)
        ->toBe(2)->and($l)->toBe(1)->toBe(2)->not->not->toBe(2);
    $x = 1;
    expect($m)->toBe(1)
        ->toBe(2); // trailing
    expect($m)->toBe(3);
});
