<?php

namespace SwitchConditionInTrait;

trait FooTrait
{

	public function doFoo(int $i): void
	{
		switch (true) {
			case $i > 0:
				break;
		}
	}

	public function doFoo2(): void
	{
		$a = false;
		switch (true) {
			case $a:
				break;
		}
	}

	public function doFoo3(): void
	{
		$b = true;
		switch (true) {
			case $b:
				break;
			case $b:
				break;
			default:
				break;
		}
	}

}

class Foo
{

	use FooTrait;

}

class FooAnother
{

	use FooTrait;

}
