<?php

function (stdClass $std) {
	Foo::$fooProperty = 123;

	$bar = 'fooProperty';
	echo Foo::$$bar;
	Foo::$$bar = 123;

	echo Foo::${$bar};
	Foo::${$bar} = 123;

	$std::$$bar = 123;
};
-----
7: Variable static property access on Foo.
8: Variable static property access on Foo.
10: Variable static property access on Foo.
11: Variable static property access on Foo.
13: Variable static property access on stdClass.
