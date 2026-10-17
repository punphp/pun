<?php

namespace Bug11241;

/** @property-read int $id */
class MagicProp
{
	/** @return mixed */
	public function __get(string $name) { echo $this->id; }
	/** @param mixed $v */
	public function __set(string $name, $v): void {}
}

/** @property-read int $id */
class ActualProp
{
	private int $id = 0;

	/** @return mixed */
	public function __get(string $name) { echo $this->id; }
	/** @param mixed $v */
	public function __set(string $name, $v): void {}
}

function (): void {
	$x = new MagicProp;
	$x->id = 1;

	$x = new ActualProp;
	$x->id = 1;
};
