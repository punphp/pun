<?php declare(strict_types = 1); // lint >= 8.0

namespace ParamCastableToNumberFunctionsNamedArgs;

function invalidUsages()
{
	var_dump(array_sum(array: [[0]]));
	var_dump(array_product(array: [[0]]));
}

function validUsages()
{
	var_dump(array_sum(array: [1]));
	var_dump(array_product(array: [1]));
}
