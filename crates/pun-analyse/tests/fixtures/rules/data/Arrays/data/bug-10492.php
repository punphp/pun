<?php

namespace Bug10492;

/**
 * @return '02'|'04'
 */
function getIndex(int $input): string
{
	if ($input > 5) {
		return '02';
	}

	return '04';
}

function (): void {
	$arr = [0 => [0], 1 => [1], 2 => [2]];
	echo print_r($arr[(int) getIndex(1)], true);
};
