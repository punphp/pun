<?php

$spread = ['s' => 's'];

$key = 'k';

return [
    'a' => 'x',
    $key => 'y',
    ...$spread,
];
