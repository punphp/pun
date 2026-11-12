<?php

function inlineIfTab(bool $flag, int $n)
{
	$flag || $n = [
		1,
	];
}
-----
<?php

function inlineIfTab(bool $flag, int $n)
{
	if (!$flag) {
	    $n = [
	    	1,
	    ];
	}
}
