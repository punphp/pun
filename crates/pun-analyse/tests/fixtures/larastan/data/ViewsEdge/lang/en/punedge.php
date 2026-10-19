<?php

$unused = 1;

return array(
    'legacy' => array('a' => 'x'),
    'list' => ['a', 'b'],
    'int' => [5 => 'five', 'six'],
    'nested' => ['deep' => ['x' => 'y']],
    PHP_EOL => 'eol',
    ...['spread' => 'z'],
);
