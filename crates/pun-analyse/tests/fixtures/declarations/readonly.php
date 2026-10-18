<?php

namespace ReadonlyClasses;

readonly class R
{
}

final readonly class S
{
}

function () {
	return new readonly class {
	};
};

function () {
	return new class {
	};
};
