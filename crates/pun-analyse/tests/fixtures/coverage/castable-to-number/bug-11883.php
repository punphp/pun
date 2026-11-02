<?php declare(strict_types = 1); // lint >= 8.1

namespace Bug11883;

enum SomeEnum: int
{
	case A = 1;
	case B = 2;
}

$enums1 = [SomeEnum::A, SomeEnum::B];

var_dump(array_sum($enums1));
var_dump(array_product($enums1));
