<?php

function f(): void
{
    nope1(); // @phpstan-ignore function.notFound (reason)
    // @phpstan-ignore-next-line
    nope2();
    /** @phpstan-ignore function.notFound */
    nope3();
}

$a = [
    nope4(), // @phpstan-ignore function.notFound
    nope5(),
];

nope6(); /* @phpstan-ignore function.notFound */ nope7();
