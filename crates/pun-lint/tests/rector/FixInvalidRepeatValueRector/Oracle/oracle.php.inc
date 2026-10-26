<?php

test('a', function () {})->repeat(0x0);
test('a', function () {})->repeat(00);
test('a', function () {})->repeat(0b0);
test('a', function () {})->repeat(0_0);
test('a', function () {})->repeat(/* c */ -2);
test('a', function () {})->repeat(- /* c */ 2);
test('a', function () {})->repeat(-0);
test('a', function () {})->repeat(times: 0);
test('a', function () {})->repeat(-2.5);
test('a', function () {})->repeat(0.0);
test('a', function () {})->repeat(-PHP_INT_MAX);
test('a', function () {})->repeat( 0 );
test('a', function () {})->repeat(-(2));
it('a', function () {})->repeat(0);
todo('a')->repeat(0);
describe('a', function () {})->repeat(0);
\test('a', function () {})->repeat(0);
TEST('a', function () {})->repeat(0);
test('a', function () {})->REPEAT(0);
test('a', function () {})->repeat();
test('a', function () {})->repeat($n);
test('a', function () {})->repeat(1);
test('a', function () {})->repeat(0, 'x');
test('a', function () {})->foo->repeat(0);
test('a', function () {})->group('x')->repeat(0)->group('y');
test('a', function () {})->repeat((0));
test('a', function () {})->repeat((/* c */ -2));
test('a', function () {})->repeat(-/* c */ (2));
test('a', function () {})->repeat(-(/* c */ 2));
test('a', function () {})->repeat(
    // why
    0
);
test('a', function () {})->repeat(+0);
expect($a)->repeat(0);
-----
<?php

test('a', function () {})->repeat(0x1);
test('a', function () {})->repeat(01);
test('a', function () {})->repeat(0b1);
test('a', function () {})->repeat(01);
test('a', function () {})->repeat(/* c */ 1);
test('a', function () {})->repeat(- /* c */ 2);
test('a', function () {})->repeat(1);
test('a', function () {})->repeat(times: 1);
test('a', function () {})->repeat(-2.5);
test('a', function () {})->repeat(0.0);
test('a', function () {})->repeat(-PHP_INT_MAX);
test('a', function () {})->repeat( 1 );
test('a', function () {})->repeat(1);
it('a', function () {})->repeat(1);
todo('a')->repeat(1);
describe('a', function () {})->repeat(0);
\test('a', function () {})->repeat(1);
TEST('a', function () {})->repeat(0);
test('a', function () {})->REPEAT(0);
test('a', function () {})->repeat();
test('a', function () {})->repeat($n);
test('a', function () {})->repeat(1);
test('a', function () {})->repeat(1, 'x');
test('a', function () {})->foo->repeat(1);
test('a', function () {})->group('x')->repeat(1)->group('y');
test('a', function () {})->repeat((1));
test('a', function () {})->repeat((/* c */ -2));
test('a', function () {})->repeat(1);
test('a', function () {})->repeat(-(/* c */ 2));
test('a', function () {})->repeat(
    // why
    1
);
test('a', function () {})->repeat(+0);
expect($a)->repeat(0);
