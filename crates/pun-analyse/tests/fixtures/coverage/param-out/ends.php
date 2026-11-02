<?php

namespace ParamOutEnds;

/** @param-out string $p */
function neverWritten(?string &$p): void
{
}

/** @param-out string $p */
function maybeWritten(?string &$p): void
{
	if (rand(0, 1)) {
		$p = 'foo';
	}
}

/** @param-out string $p */
function earlyReturn(?string &$p): void
{
	if (rand(0, 1)) {
		return;
	}
	$p = 'foo';
}

/** @param-out string $p */
function throws(?string &$p): void
{
	if (rand(0, 1)) {
		throw new \Exception();
	}
	$p = 'foo';
}

/** @param-out string $p */
function returnValue(?string &$p): int
{
	if (rand(0, 1)) {
		$p = 'a';
		return 1;
	}
	return 2;
}

/** @param-out string $p */
function emptyBranches(?string &$p): void
{
	if (rand(0, 1)) {
	} elseif (rand(0, 1)) {
		$p = 'x';
	} else {
	}
}

/** @param-out string $p */
function trailingComment(?string &$p): void
{
	echo 1;
	// done
}

/** @param-out int $p */
function alwaysWritten(?string &$p): void
{
	$p = 'x';
}
