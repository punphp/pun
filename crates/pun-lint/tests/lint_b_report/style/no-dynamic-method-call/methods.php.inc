<?php

function (stdClass $std) {
	$std->foo();

	$foo = 'bar';
	$std->$foo();
};
-----
7: Variable method call on stdClass.
