<?php

test('a', function ($x) {
    expect(is_array($x))->toBeTrue();
    expect(is_string($x))->toBeTrue();
    expect(is_int($x))->toBeTrue();
    expect(is_integer($x))->toBeTrue();
    expect(is_float($x))->toBeTrue();
    expect(is_double($x))->toBeTrue();
    expect(is_bool($x))->toBeTrue();
    expect(is_numeric($x))->toBeTrue();
    expect(is_callable($x))->toBeTrue();
    expect(is_iterable($x))->toBeTrue();
    expect(is_object($x))->toBeTrue();
    expect(is_resource($x))->toBeTrue();
    expect(is_scalar($x))->toBeTrue();
    expect(is_null($x))->toBeTrue();
    expect(is_countable($x))->toBeTrue();
    expect(is_array($x))->toBeFalse();
    expect(is_array($x))->not->toBeTrue();
    expect(is_array($x))->toBeBool()->toBeTrue();
    expect(is_callable($x, true))->toBeTrue();
    expect(is_array(value: $x))->toBeTrue();
    expect($a)->and(is_int($x))->toBeTrue();
    expect(is_int($x))
        ->toBeTrue();
    expect(is_int($x))->ToBeTrue();
    expect(is_int())->toBeTrue();
});
-----
<?php

test('a', function ($x) {
    expect($x)->toBeArray();
    expect($x)->toBeString();
    expect($x)->toBeInt();
    expect($x)->toBeInt();
    expect($x)->toBeFloat();
    expect($x)->toBeFloat();
    expect($x)->toBeBool();
    expect($x)->toBeNumeric();
    expect($x)->toBeCallable();
    expect($x)->toBeIterable();
    expect($x)->toBeObject();
    expect($x)->toBeResource();
    expect($x)->toBeScalar();
    expect($x)->toBeNull();
    expect(is_countable($x))->toBeTrue();
    expect(is_array($x))->toBeFalse();
    expect($x)->not->toBeArray();
    expect($x)->toBeBool()->toBeArray();
    expect(is_callable($x, true))->toBeTrue();
    expect($x)->toBeArray();
    expect($a)->and($x)->toBeInt();
    expect($x)
        ->toBeInt();
    expect($x)->toBeInt();
    expect(is_int())->toBeTrue();
});
