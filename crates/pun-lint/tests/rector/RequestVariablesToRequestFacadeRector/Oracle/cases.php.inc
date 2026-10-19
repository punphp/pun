<?php

namespace App;

$a = $_GET['a'][$x];
$b = $_GET[$x]['a'];
$c = isset($_GET['a']['b']);
$d = isset($c['a']);
$e = foo($_GET);
$f = !isset($_GET['a']);
$g = $_REQUEST;
$h = $_GET['it\'s'];
$i = $_COOKIE['a'];
$j = $_GET['a'] ?? 'b';
$k = isset($_COOKIE['a']);
$l = isset($_GET[$key]);
$m = $_GET[10];
$n = $_POST['a'][1]['b'];
$o = isset($_GET);
$p = $_GET['a'] . $_POST['b'];
$q = isset($_POST['a']) && isset($_REQUEST['b']);
$r = $_GET['back\\slash'];
-----
<?php

namespace App;

$a = \Illuminate\Support\Facades\Request::query('a')[$x];
$b = $_GET[$x]['a'];
$c = isset($_GET['a']['b']);
$d = isset($c['a']);
$e = foo(\Illuminate\Support\Facades\Request::query());
$f = !(\Illuminate\Support\Facades\Request::query('a') !== null);
$g = \Illuminate\Support\Facades\Request::all();
$h = \Illuminate\Support\Facades\Request::query('it\'s');
$i = $_COOKIE['a'];
$j = \Illuminate\Support\Facades\Request::query('a') ?? 'b';
$k = isset($_COOKIE['a']);
$l = isset($_GET[$key]);
$m = \Illuminate\Support\Facades\Request::query('10');
$n = \Illuminate\Support\Facades\Request::post('a.1.b');
$o = isset($_GET);
$p = \Illuminate\Support\Facades\Request::query('a') . \Illuminate\Support\Facades\Request::post('b');
$q = \Illuminate\Support\Facades\Request::post('a') !== null && \Illuminate\Support\Facades\Request::exists('b');
$r = \Illuminate\Support\Facades\Request::query('back\slash');
