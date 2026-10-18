<?php

use Illuminate\Support\Facades\Redirect as R;

$a = redirect()->BACK(302);
$b = R::back(302, [], fallback: '/');
$c = \Illuminate\Support\Facades\Redirect::route('x', ['a' => 1]);
$d = redirect('/x')->route('home');
$e = \Redirect::route('home');
-----
<?php

use Illuminate\Support\Facades\Redirect as R;

$a = back(302);
$b = back(302, [], fallback: '/');
$c = \Illuminate\Support\Facades\Redirect::route('x', ['a' => 1]);
$d = redirect('/x')->route('home');
$e = \Redirect::route('home');
