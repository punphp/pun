<?php

test('a', function () {})->only();
it('a', function () {})->group('x')->only();
describe('a', function () {})->only();
todo('a')->only();
beforeEach(function () {})->only();
afterAll(function () {})->only();
\test('a', function () {})->only();
TEST('a', function () {})->only();
test('a', function () {})->ONLY();
expect($a)->only();
foo('a')->only();
test('a', function () {})->foo->only();
test('a', function () {})->a()->b->c()->only();
test('a', function () {})
    ->only()
    ->group('x');
test('a', function () {}) // why
    ->only();
test('a', function () {})->only('x');
$x->only();
test('a', function () {})?->only();
-----
<?php

test('a', function () {});
it('a', function () {})->group('x');
describe('a', function () {});
todo('a');
beforeEach(function () {});
afterAll(function () {});
\test('a', function () {});
TEST('a', function () {});
test('a', function () {})->ONLY();
expect($a)->only();
foo('a')->only();
test('a', function () {})->foo;
test('a', function () {})->a()->b->c()->only();
test('a', function () {})
    ->group('x');
test('a', function () {});
test('a', function () {});
$x->only();
test('a', function () {})?->only();
