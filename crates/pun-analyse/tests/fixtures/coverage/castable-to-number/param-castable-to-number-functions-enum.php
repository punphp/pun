<?php declare(strict_types = 1); // lint >= 8.1

namespace ParamCastableToNumberFunctionsEnum;

enum FooEnum
{
	case A;
}

function invalidUsages()
{
	array_sum([FooEnum::A]);
	array_product([FooEnum::A]);
}
