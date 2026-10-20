<?php // lint >= 8.1

namespace Bug7880b;

enum Undefined: string
{
	case VALUE = '!@#$~~UNDEFINED~~$#@!';
}

class Test1 {}

class Test {
	/**
	 * @var Undefined|array<Test1>
	 */
	public array|Undefined $property = Undefined::VALUE;
}

function (): void {
	$a = new Test();
	$a->property = [];
	$a->property[] = new Test1();
};
