<?php

nope1(); // @phpstan-ignore
nope2(); // @phpstan-ignore function.notFound,
nope3(); // @phpstan-ignore function.notFound (unclosed
nope4(); // @phpstan-ignore -dash

/**
 * @phpstan-ignore function.notFound,,
 */
nope5();
