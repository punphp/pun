<?php

namespace Bug13135;

/**
 * @template Tk
 * @template Tv
 *
 * @param iterable<Tk, Tv> $iterable
 */
function my_to_array(iterable $iterable): void
{
	$result = [];
	foreach ($iterable as $k => $v) {
		$result[$k] = $v;
	}

	var_dump($result);
}
