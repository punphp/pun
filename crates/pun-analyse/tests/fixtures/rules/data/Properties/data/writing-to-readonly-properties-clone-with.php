<?php // lint >= 8.5

namespace WritingToReadonlyPropertiesCloneWith;

interface Foo
{

	public int $i {
		// virtual, not writable
		get;
	}

}

function (Foo $f): void {
	clone ($f, [
		'i' => 1,
	]);
};
