<?php
namespace App;
$a = sprintf('%s', 1) . 'x';
$b = 'x' . \sprintf('%s', 1);
$c = 'x' . 'y';
$d = sprintf('%s', 1) + 1;
$e = (sprintf('%s', 1)) . 'x';
$f = 'x' . SPRINTF('%s', 1) . 'y';
$g = $o->sprintf('a') . 'x';
