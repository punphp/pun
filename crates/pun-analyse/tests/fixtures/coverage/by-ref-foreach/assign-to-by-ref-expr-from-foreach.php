<?php

namespace AssignToByRefExprFromForeach;

class Foo
{

	/** @param list<array<string, mixed>> $input */
	public function sayHello(array $input): void
	{
		foreach ($input as &$item) {
			$item = 2;
		}
		unset($item);

		$item = 'foo';
	}

	/** @param list<array<string, mixed>> $input */
	public function sayHello2(array $input): void
	{
		foreach ($input as &$item) {
			$item = 2;
		}

		$item = 'foo';
	}

	public function doFoo(): void
	{
		$array = [0, 1, 2, 3];
		foreach ($array as &$item) {
			$item = 2;
		}

		unset($item);

		foreach ($array as $item) {

		}
	}

	public function doFoo2(): void
	{
		$array = [0, 1, 2, 3];
		foreach ($array as &$item) {
			$item = 2;
		}

		foreach ($array as $item) {
			$item = 2;
		}
	}

	public function doFoo3(): void
	{
		$array = [0, 1, 2, 3];
		foreach ($array as &$item) {

		}

		foreach ($array as $item) {

		}
	}

	public function doFoo4(): void
	{
		while (true) {
			$array = [0, 1, 2, 3];
			foreach ($array as &$item) {

			}
		}
	}

}
