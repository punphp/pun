<?php

namespace CoverageUnpacking;

/**
 * @param array<string, int> $named
 * @param array<int|string, int> $mixedKeys
 * @param list<int> $list
 * @param array<int, string> $byInt
 */
function unpacking(array $named, array $mixedKeys, array $list, array $byInt, float $f, ?int $n): void
{
	$a = [...$named];
	$b = [...$mixedKeys];
	$c = [...$list];
	$d = [...new UnknownUnpacked()];
	echo $byInt[$f];
	echo $byInt[$n];
	echo $byInt[true];
}
