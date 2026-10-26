<?php

test('a', function (bool $flag, $untyped, array $list) {
    expect(!$flag)->toBeTrue();
    expect(!$flag)->toBeFalse();
    expect(!$flag)->not->toBeTrue();
    expect(!$untyped)->toBeTrue();
    expect(!($a === $b))->toBeTrue();
    expect(!is_string($untyped))->toBeFalse();
    expect(!$list)->toBeEmpty();
    expect(!$list)->not->toBeEmpty();
    expect(!$list)
        ->not
        ->toBeEmpty();
    expect(!$list)->toBeEmpty()->toBeBool();
    expect(!$flag)->toBeBool()->toBeTrue();
    expect($a)->and(!$flag)->toBeTrue();
    expect(!$flag)->and($a)->toBeTrue();
    expect(!$flag, 'msg')->toBeTrue();
    expect(value: !$flag)->toBeTrue();
    expect(!$flag)->TOBETRUE();
    expect(!$flag)->toBeTruthy();
    expect(!$list)->each->toBeEmpty();
    expect(!$list)->not->not->toBeEmpty();
});
-----
<?php

test('a', function (bool $flag, $untyped, array $list) {
    expect($flag)->toBeFalse();
    expect($flag)->toBeTrue();
    expect($flag)->not->toBeFalse();
    expect(!$untyped)->toBeTrue();
    expect($a === $b)->toBeFalse();
    expect(is_string($untyped))->toBeTrue();
    expect($list)->not->toBeEmpty();
    expect($list)->toBeEmpty();
    expect($list)
        ->toBeEmpty();
    expect($list)->not->toBeEmpty()->toBeBool();
    expect(!$flag)->toBeBool()->toBeTrue();
    expect($a)->and(!$flag)->toBeTrue();
    expect(!$flag)->and($a)->toBeTrue();
    expect($flag, 'msg')->toBeFalse();
    expect($flag)->toBeFalse();
    expect(!$flag)->TOBETRUE();
    expect(!$flag)->toBeTruthy();
    expect(!$list)->each->toBeEmpty();
    expect(!$list)->not->not->toBeEmpty();
});
