<?php

describe('d', function () {
      test('x', function () {
          expect($a)->toBe(1);
          expect($b)->toBe(2);
          expect($b)->toBe(3);
      });
});

expect($a)->toBe(1); expect($a)->toBe(2);
if ($x) {
    expect($a)->toBe(1);
    expect($a)->toBe(2);
} else expect($b)->toBe(1);
-----
<?php

describe('d', function () {
      test('x', function () {
          expect($a)->toBe(1)
              ->and($b)->toBe(2)
              ->toBe(3);
      });
});

expect($a)->toBe(1)
    ->toBe(2);
if ($x) {
    expect($a)->toBe(1)
        ->toBe(2);
} else expect($b)->toBe(1);
