<?php

test('a', function (string $s, int $i, $untyped) {
    expect(filter_var($s, FILTER_VALIDATE_URL))->not->toBeFalse();
    expect(filter_var($s, FILTER_VALIDATE_URL) !== false)->toBeTrue();
    expect(filter_var($s, FILTER_VALIDATE_URL))->toBeFalse();
    expect(filter_var($s, FILTER_VALIDATE_URL) === false)->toBeTrue();
    expect(false !== filter_var($s, FILTER_VALIDATE_URL))->toBeTrue();
    expect(filter_var($s, FILTER_VALIDATE_URL) !== FALSE)->toBeTrue();
    expect(filter_var($s, \FILTER_VALIDATE_URL))->not->toBeFalse();
    expect(filter_var($s, filter_validate_url))->not->toBeFalse();
    expect(filter_var($s, FILTER_VALIDATE_EMAIL))->not->toBeFalse();
    expect(filter_var($s))->not->toBeFalse();
    expect(filter_var($s, FILTER_VALIDATE_URL, FILTER_FLAG_PATH_REQUIRED))->not->toBeFalse();
    expect(filter_var($i, FILTER_VALIDATE_URL))->not->toBeFalse();
    expect(filter_var($untyped, FILTER_VALIDATE_URL))->not->toBeFalse();
    expect(FILTER_VAR($s, FILTER_VALIDATE_URL))->not->toBeFalse();
    expect(filter_var($s, FILTER_VALIDATE_URL))->not->TOBEFALSE();
    expect(filter_var($s, FILTER_VALIDATE_URL))->each->not->toBeFalse();
    expect(filter_var($s, FILTER_VALIDATE_URL))->not->each->toBeFalse();
    expect(filter_var($s, FILTER_VALIDATE_URL))
        ->not
        ->toBeFalse();
    expect(filter_var($s, FILTER_VALIDATE_URL))->toBeString()->not->toBeFalse();
    expect($a)->and(filter_var($s, FILTER_VALIDATE_URL))->not->toBeFalse();
    expect(filter_var($s, FILTER_VALIDATE_URL), 'msg')->not->toBeFalse();
    expect(filter_var('http://x', FILTER_VALIDATE_URL))->not->toBeFalse();
    expect(filter_var(1, FILTER_VALIDATE_URL) !== false)->toBeTrue();
    expect(filter_var($s, FILTER_VALIDATE_URL) !== false)->not->toBeTrue();
});
-----
<?php

test('a', function (string $s, int $i, $untyped) {
    expect($s)->toBeUrl();
    expect($s)->toBeUrl();
    expect(filter_var($s, FILTER_VALIDATE_URL))->toBeFalse();
    expect(filter_var($s, FILTER_VALIDATE_URL) === false)->toBeTrue();
    expect(false !== filter_var($s, FILTER_VALIDATE_URL))->toBeTrue();
    expect($s)->toBeUrl();
    expect($s)->toBeUrl();
    expect($s)->toBeUrl();
    expect(filter_var($s, FILTER_VALIDATE_EMAIL))->not->toBeFalse();
    expect(filter_var($s))->not->toBeFalse();
    expect($s)->toBeUrl();
    expect(filter_var($i, FILTER_VALIDATE_URL))->not->toBeFalse();
    expect(filter_var($untyped, FILTER_VALIDATE_URL))->not->toBeFalse();
    expect($s)->toBeUrl();
    expect($s)->toBeUrl();
    expect($s)->each->toBeUrl();
    expect($s)->not->each->toBeUrl();
    expect($s)
        ->toBeUrl();
    expect($s)->toBeString()->toBeUrl();
    expect($a)->and($s)->toBeUrl();
    expect($s)->toBeUrl();
    expect('http://x')->toBeUrl();
    expect(filter_var(1, FILTER_VALIDATE_URL) !== false)->toBeTrue();
    expect($s)->not->toBeUrl();
});
