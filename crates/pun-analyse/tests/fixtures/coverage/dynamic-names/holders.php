<?php declare(strict_types = 1);

namespace DynamicNameHolders;

class ParentHolder
{

}

class Holder extends ParentHolder
{

	public function test(object $object, ?Holder $holder): void
	{
		echo Holder::{$object};
		echo static::{$object};
		echo parent::{$object};
		$holder?->$object();
		echo $holder?->$object;
	}

}
