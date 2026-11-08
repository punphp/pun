<?php

/** @phpstan-ignore function.notFound */
$a = 1;

/**
 * Text.
 * @phpstan-ignore function.notFound
 */
$b = 1;

/**
 * @phpstan-ignore function.notFound, class.notFound
 */
nope1();

/**
 * @phpstan-ignore-next-line
 * @return void
 */
$c = 1;

/** @phpstan-ignore-next-line */
$d = 1;

$e = 1; // @phpstan-ignore function.notFound, class.notFound (why)
$f = 1; // other @phpstan-ignore function.notFound
$g = 1; /* @phpstan-ignore function.notFound */ $h = 1;
$i = 1; // @phpstan-ignore-line text
nope2(); // @phpstan-ignore-next-line
nope3();
nope4(); /* @phpstan-ignore-next-line */ nope5();
nope6();
/* @phpstan-ignore-next-line (unbalanced */
nope7();
/* @phpstan-ignore-next-line @see x */
nope8();
# @phpstan-ignore-next-line
nope9(); // trailing
