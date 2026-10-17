<?php // lint >= 8.5

namespace DeprecatedAttributeOnTrait;

#[\Deprecated]
trait DeprTrait
{

}

class Foo
{

	use DeprTrait;

}
