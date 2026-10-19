<?php

namespace PunDeprecations3;

class HasOld
{
	/** @deprecated */
	public $magic;

	/** @deprecated */
	public static $st;

	/** @deprecated */
	public $inst;

	public $holder;
}

/** @property int $magic */
class WithMagic
{
	public function __get($name)
	{
	}
}

class Sub extends HasOld
{
	public function f(): void
	{
		echo parent::$st;
	}
}

class NoString
{
}

function g(WithMagic $w, HasOld $h, NoString $none): void
{
	echo $w->magic;
	echo $h->st;
	echo HasOld::$inst;
	echo (string) $none;
	$h->holder->magic = 1;
	echo $h->holder->magic;
}
