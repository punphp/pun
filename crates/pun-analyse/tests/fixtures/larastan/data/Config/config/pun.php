<?php

return [
    'string' => 'bar',
    'list' => [1, 2, 3],
    'int' => 1,
    'float' => 1.5,
    'true' => true,
    'null' => null,
    'env' => env('PUN_DRIVER', 'file'),
    'path' => storage_path('pun'),
    'concat' => 'a'.'b',
    'class' => \stdClass::class,
    'interpolated' => "a {$x}",
    'cast' => (array) 'x',
    'nested' => [
        'string' => 'x',
        'array' => ['a' => 'b'],
        5 => 'five',
    ],
    'string' => 'last',
    7 => 'seven',
    'top' => 'top',
];
