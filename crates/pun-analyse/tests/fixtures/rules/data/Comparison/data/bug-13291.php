<?php // lint >= 8.2

namespace Bug13291;

function test(bool $someBool): true {
	var_dump($someBool);
	return true;
}

test(someBool: true);
