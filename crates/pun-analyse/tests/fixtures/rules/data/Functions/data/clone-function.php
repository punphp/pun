<?php // lint >= 8.5

namespace CloneFunction;

class Foo
{

}

function (): void {
	clone ($foo, []);
	clone ($foo, 1);
	clone ($foo, [], 1);
};
