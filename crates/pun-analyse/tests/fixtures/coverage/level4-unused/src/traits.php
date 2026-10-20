<?php

namespace Level4Unused;

trait Used
{
	use UsedThroughTrait;
}

trait UsedThroughTrait
{
}

trait NeverUsed
{
	use OnlyByUnused;
}

trait OnlyByUnused
{
}

#[\Attribute]
class Attr
{
}

#[Attr]
trait WithAttribute
{
}

class User
{
	use Used;
}

function anonymous(): object
{
	return new class {
		use UsedByAnonymous;
	};
}

trait UsedByAnonymous
{
}
