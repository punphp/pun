<?php

$a = $_ENV['A'] ?? 'x';
foo($_ENV['C'], (int) $_ENV['D']);
$c = $_ENV['G']['H'];
[$_ENV['I']] = [1];
-----
<?php

$a = \Illuminate\Support\Env::get('A') ?? 'x';
foo(\Illuminate\Support\Env::get('C'), (int) \Illuminate\Support\Env::get('D'));
$c = \Illuminate\Support\Env::get('G')['H'];
[$_ENV['I']] = [1];
