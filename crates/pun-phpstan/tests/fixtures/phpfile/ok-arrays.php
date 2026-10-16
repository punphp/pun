<?php

$a = array(
	'k' => NULL,
	'yes' => true,
	'no' => false,
	'neg' => -5,
	7 => 'int key',
	'7b' => 'string key',
	'8' => 'numeric string key',
	'nested' => array(1, 2, array()),
	'dir' => __DIR__ . '/x' . '/y',
	'trailing' => [1, 2,],
);
$b = $a;
return ['parameters' => ['a' => $a, 'b' => $b, 'short' => [true, false,], 'empty' => []]];
