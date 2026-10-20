<?php

namespace CoverageGlobalAliases;

/**
 * @phpstan-type Local array<GlobalAlias>
 */
class Exporter
{
}

/**
 * @phpstan-type GlobalAlias int
 * @phpstan-import-type Local from Exporter as GlobalList
 * @template GlobalAlias
 */
class Duplicates
{
	/**
	 * @template GlobalList
	 * @param GlobalAlias $a
	 * @return GlobalList
	 */
	public function method($a)
	{
		return [];
	}
}

/**
 * @template GlobalAlias
 * @param GlobalList $list
 */
function generic(array $list): void
{
	/** @var GlobalAlias $value */
	$value = 1;
	/** @var MissingClass $missing */
	$missing = 2;
}

/**
 * @template GlobalAlias
 */
interface Templated
{
}
