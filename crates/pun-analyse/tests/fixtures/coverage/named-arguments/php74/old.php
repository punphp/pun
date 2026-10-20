<?php

namespace CoverageNamedArgumentsOld;

#[\Attribute]
class Attr
{
	public function __construct(int $value)
	{
	}
}

function fixed(int $a): void
{
}

#[Attr(value: 1)]
function attributed(): void
{
	fixed(a: 1);
}
