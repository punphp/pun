<?php

namespace App;

$a = ($_GET['a'])['b'];
$b = ($_POST)['a'];
$c = ($_GET)['a']['b'];
foo($_GET[]);
foo($_GET[]['a']);
-----
<?php

namespace App;

$a = \Illuminate\Support\Facades\Request::query('a.b');
$b = \Illuminate\Support\Facades\Request::post('a');
$c = \Illuminate\Support\Facades\Request::query('a.b');
foo($_GET[]);
foo($_GET[]['a']);
