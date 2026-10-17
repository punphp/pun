<?php // lint >= 8.0

namespace Bug11424;

/**
 * @param object{hello?: string} $a
 */
function hello(object $a): void
{
	echo $a->hello;
	echo $a->hello ?? 'hello';
	if (isset($a->hello)) {
		echo 'hi';
	}
}

class Foo
{

	public int $i;

}

class Bar
{

}

function hello2(Foo|Bar $a): void
{
	echo $a->i;
	echo $a->i ?? 'hello';
	if (isset($a->i)) {
		echo 'hi';
	}
}
