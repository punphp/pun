<?php

nope1(); // @phpstan-ignore function.notFound
nope2(); // @phpstan-ignore function.notFound (we know)
new Nope3(); // @phpstan-ignore function.notFound
new Nope4(); nope4(); // @phpstan-ignore class.notFound, function.notFound (both)
nope5(); nope5b(); // @phpstan-ignore function.notFound
nope6(); // @phpstan-ignore function.notFound, function.notFound

// @phpstan-ignore function.notFound
nope7();

/** @phpstan-ignore class.notFound (comment) */
new Nope8();

// @phpstan-ignore function.notFound

nope9();

$x = 1; // @phpstan-ignore unused.id
$y = 2; // @phpstan-ignore unused.a (why), unused.b
