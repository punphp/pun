<?php

namespace SignatureNoParent;

class Base {}

class WithParent extends Base
{
	public function native(parent $p): parent {}
}

class WithoutParent
{
	public function nullDefault(parent $p = null): void {}

	public function variadic(parent ...$p): void {}

	/**
	 * @param parent ...$p
	 * @param parent|null $q
	 */
	public function phpDocVariadic($q, ...$p) {}

	/**
	 * @param Base $p
	 * @return Base
	 */
	public function phpDocClass($p) {}
}

class UnknownParent extends NonexistentBase
{
	public function native(parent $p): void {}
}
