<?php

function (stdClass $std) {
	Foo::doFoo();

	$foo = 'doBar';
	Foo::$foo();

	$std::$foo();
};
-----
7: Variable static method call on Foo.
9: Variable static method call on stdClass.
