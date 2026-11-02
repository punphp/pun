<?php

namespace TooWideThrowsExplicit;

final class Foo
{

	/**
	 * @throws \Exception
	 */
	public function doFoo(): void
	{
		$a = 1 + 1;
		$this->doBar();
	}

	public function doBar(): void
	{

	}

}
