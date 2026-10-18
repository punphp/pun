<?php

$_SERVER['A'] = $_SERVER['B'];
$x = $_SERVER['C'] . $_GET['D'];
$_GET['x'] = $_SERVER['E'];
$y = $_SERVER['F']['G'];
echo "$_SERVER[H]";
$z = empty($_SERVER['I']);
-----
<?php

$_SERVER['A'] = $_SERVER['B'];
$x = \Illuminate\Support\Facades\Request::server('C') . $_GET['D'];
$_GET['x'] = $_SERVER['E'];
$y = \Illuminate\Support\Facades\Request::server('F')['G'];
echo "$_SERVER[H]";
$z = empty(\Illuminate\Support\Facades\Request::server('I'));
