<?php

// Rector writes `"{\Illuminate\Support\Env::get('B')}"`, `\Illuminate\Support\Env::get('E') .= 'x';` and
// `isset(\Illuminate\Support\Env::get('F'));`, which PHP rejects.
$b = "{$_ENV['B']}";
$_ENV['E'] .= 'x';
isset($_ENV['F']);
$_ENV['J']['K'] = 1;
$r = &$_ENV['L'];
$_ENV['M']++;
