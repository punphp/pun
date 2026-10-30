<?php

// an assignment to `$_SERVER` keeps its item, also in a chain (rector-laravel's ArrayDimFetchContextNodeVisitor)
$_SERVER['A'] = $_ENV['A'] = 'x';
$_ENV['B'] = $_SERVER['B'] = $_SERVER['C'];
$d = $_SERVER['D'] = 'y';
