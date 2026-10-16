<?php

nope1(); // @phpstan-ignore-line
nope2(); // @phpstan-ignore-line

// @phpstan-ignore-next-line
nope3();

// @phpstan-ignore-next-line
$ok = 1;

/** @phpstan-ignore-next-line */
nope4(); nope5();

/*
 * @phpstan-ignore-next-line
 */
nope6();

$a = 1; // @phpstan-ignore-line
