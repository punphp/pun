<?php

namespace App;

final class Thing
{
    public static function make()
    {
    }
}

$a = 'prefix ' . 'App\Thing::make()';
$b = 'App\Thing::make()' . ' suffix';
$c = '\App\Thing::make() and App\Thing again';
$d = "App\\Thing::make()";
$e = 'App\Thing::';
$f = 'App\Missing::make()';
$g = 'Thing::make()';
$h = '::nothing';
$i = 'it\'s App\Thing::make()';
$j = ['App\Thing::make' => 1];
-----
<?php

namespace App;

final class Thing
{
    public static function make()
    {
    }
}

$a = 'prefix ' . (\App\Thing::class . '::make()');
$b = \App\Thing::class . '::make()' . ' suffix';
$c = \App\Thing::class . '::make() and App\Thing again';
$d = \App\Thing::class . '::make()';
$e = \App\Thing::class . '::';
$f = 'App\Missing::make()';
$g = 'Thing::make()';
$h = '::nothing';
$i = 'it\'s ' . \App\Thing::class . '::make()';
$j = [\App\Thing::class . '::make' => 1];
