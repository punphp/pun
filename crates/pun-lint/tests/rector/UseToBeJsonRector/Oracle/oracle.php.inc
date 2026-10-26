<?php

test('a', function (string $s, int $i, $untyped) {
    expect(json_decode($s) !== null)->toBeTrue();
    expect(json_decode($s) === null)->toBeTrue();
    expect(json_decode($s) !== null)->toBeFalse();
    expect(json_decode($s) === null)->toBeFalse();
    expect(json_decode($s) !== null)->not->toBeTrue();
    expect(null !== json_decode($s))->toBeTrue();
    expect(json_decode($s) !== NULL)->toBeTrue();
    expect(json_decode($s, true) !== null)->toBeTrue();
    expect(json_decode($i) !== null)->toBeTrue();
    expect(json_decode($untyped) !== null)->toBeTrue();
    expect(\json_decode($s) !== null)->toBeTrue();
    expect(JSON_DECODE($s) !== null)->toBeTrue();
    expect(json_decode() !== null)->toBeTrue();
    expect(json_decode($s) != null)->toBeTrue();
    expect(json_decode($s) !== false)->toBeTrue();
    expect(json_decode($s) !== null)->TOBETRUE();
    expect(json_decode($s) !== null)->toBeBool()->toBeTrue();
    expect(json_decode($s) !== null, 'msg')->toBeTrue();
    expect(json_decode($s) !== null)
        ->toBeTrue();
    expect($a)->and(json_decode($s) !== null)->toBeTrue();
    expect(json_decode($s) !== null)->and($a)->toBeTrue();
    expect(json_decode('{}') !== null)->toBeTrue();
    expect(json_decode(json: $s) !== null)->toBeTrue();
});
-----
<?php

test('a', function (string $s, int $i, $untyped) {
    expect($s)->toBeJson();
    expect($s)->not->toBeJson();
    expect($s)->not->toBeJson();
    expect($s)->toBeJson();
    expect($s)->not->toBeJson();
    expect($s)->toBeJson();
    expect($s)->toBeJson();
    expect($s)->toBeJson();
    expect(json_decode($i) !== null)->toBeTrue();
    expect($untyped)->toBeJson();
    expect($s)->toBeJson();
    expect(JSON_DECODE($s) !== null)->toBeTrue();
    expect(json_decode() !== null)->toBeTrue();
    expect(json_decode($s) != null)->toBeTrue();
    expect(json_decode($s) !== false)->toBeTrue();
    expect(json_decode($s) !== null)->TOBETRUE();
    expect($s)->toBeJson();
    expect($s, 'msg')->toBeJson();
    expect($s)->toBeJson();
    expect($a)->and(json_decode($s) !== null)->toBeTrue();
    expect($s)->toBeJson();
    expect('{}')->toBeJson();
    expect($s)->toBeJson();
});
