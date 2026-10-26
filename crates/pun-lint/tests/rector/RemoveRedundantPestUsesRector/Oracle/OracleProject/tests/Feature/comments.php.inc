<?php

use Illuminate\Foundation\Testing\RefreshDatabase;

// keep me
it('a', fn () => 1);

// why
uses(RefreshDatabase::class);

pest()->USE(
    // c1
    RefreshDatabase::class,
    /* c2 */ Keep::class,
)->group('x');

(pest())->uses(value: RefreshDatabase::class);
pest()->use(Keep::class, RefreshDatabase::class, Keep2::class, \Illuminate\Foundation\Testing\RefreshDatabase::class);
\USES(Keep::class, RefreshDatabase::CLASS);
uses(...);
function f() {
    uses(RefreshDatabase::class);
}
-----
<?php

use Illuminate\Foundation\Testing\RefreshDatabase;

// keep me
it('a', fn () => 1);

pest()->USE(
    // c1
    RefreshDatabase::class,
    /* c2 */ Keep::class,
)->group('x');
pest()->use(Keep::class, Keep2::class);
\USES(Keep::class);
uses(...);
function f() {
    uses(RefreshDatabase::class);
}
