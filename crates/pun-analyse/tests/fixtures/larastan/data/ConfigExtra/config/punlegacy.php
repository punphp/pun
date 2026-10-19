<?php

$x = 'x';

return array(
    'a' => 'x',
    'b' => array('c' => 1),
    1.5 => 'float key',
    __LINE__ => 'magic key',
    "k{$x}" => 'interpolated key',
    'var' => $x,
    'dynamic' => $x(),
    'unknown' => pun_unknown_function(),
    'object' => pun_config_object(),
    'enum' => pun_config_enum(),
    'closure' => pun_config_closure(),
    'callable' => pun_config_callable(),
    'mixed' => pun_config_mixed(),
    'envarray' => env('PUN', ['a']),
    'envnull' => env('PUN'),
    'intcast' => (int) $x,
    'not' => !$x,
    'after' => 'x',
);
