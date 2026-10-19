<?php // lint >= 8.1

namespace Bug12379;

class HelloWorld
{
	use myTrait{
		myTrait::__construct as private __myTraitConstruct;
	}

	public function __construct(
		int $entityManager
	){
		$this->__myTraitConstruct($entityManager);
	}
}

trait myTrait{
	public function __construct(
		private readonly int $entityManager
	){}
}
