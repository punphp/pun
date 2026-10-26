<?php

test('a', #[Attr] static function &(int $a, string $b): int {
  expect($i)->toBe(1);
  expect($i)->toBe(2) ?>
<?php
  expect($i)->toBe(3);
  /**
       * doc
       */
  foo(
    $x,
  );
  /*
      block
      */
  bar();
  /* one
     two */
  baz();
  /*    wide
     narrow */
  qux();
  /*
        open
      end */
  quux();
});
-----
<?php

test('a', #[Attr] static function &(int $a, string $b): int {
    expect($i)->toBe(1)
        ->toBe(2)
        ->toBe(3);
    /**
     * doc
     */
    foo(
      $x,
    );
    /*
    block
    */
    bar();
    /* one
       two */
    baz();
    /*    wide
         narrow */
    qux();
    /*
            open
          end */
    quux();
});
