<?php

namespace Bug12645;

class Foo
{
	private int $id = 1;

	public function getId(): int
	{
		return $this->id;
	}
}

function (): void {
	$foo = new Foo();

	var_dump($foo->id ?? 2);
	var_dump($foo->id);
};

function (Foo $foo): void {
	var_dump($foo->id ?? 2);
	var_dump($foo->id);
};
