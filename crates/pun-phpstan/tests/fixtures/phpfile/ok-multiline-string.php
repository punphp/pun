<?php declare(strict_types = 1);

$ignoreErrors = [];
$ignoreErrors[] = [
	'message' => '#^Line one
line two$#',
	'count' => 1,
	'path' => __DIR__ . '/src/a.php',
];

return ['parameters' => ['ignoreErrors' => $ignoreErrors]];
