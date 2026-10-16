<?php declare(strict_types = 1);

$ignoreErrors = [];
$ignoreErrors[] = [
	'message' => '#^Foo$#',
	'identifier' => 'foo.bar',
	'count' => 2,
	'path' => __DIR__ . '/src/a.php',
];

return ['parameters' => ['ignoreErrors' => $ignoreErrors]];
