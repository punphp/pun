<?php

namespace App;

final class Plain
{
    public function run(): int
    {
        return 1;
    }

    public static function make(): self
    {
        return new self();
    }
}

final class Invokable
{
    public function __invoke()
    {
    }
}

class Open
{
}

function helper()
{
}

function run(Plain $plain, Invokable $invokable, Open $open, string $string, array $array, $mixed, ?int $maybe)
{
    $a = with($plain, 1)->run();
    $b = with($plain, null);
    $c = with($plain, 'not a function');
    $d = with($plain, 'strlen');
    $e = with($plain, 'App\helper');
    $f = with($plain, 'App\Plain::run');
    $f2 = with($plain, 'App\Plain::make');
    $f3 = with($plain, '\App\Plain::make');
    $f4 = with($plain, 'App\Plain::missing');
    $f5 = with($plain, 'Missing::make');
    $f6 = with($plain, 'App\Open::missing');
    $f7 = with($plain, '');
    $f8 = with($plain, '1A::make');
    $f9 = with($plain, 'App\Plain::');
    $f10 = with($plain, 'App\Plain::m-n');
    $g = with($plain, $plain);
    $h = with($plain, $invokable);
    $i = with($plain, $open);
    $j = with($plain, $string);
    $k = with($plain, $array);
    $l = with($plain, $mixed);
    $m = with($plain, $maybe);
    $n = with($plain, 1.5, 2);
    $o = with();
    $p = with(new Plain)->run();
    $r = with($plain, true);
    $s = with($plain, fn () => 1);
}
-----
<?php

namespace App;

final class Plain
{
    public function run(): int
    {
        return 1;
    }

    public static function make(): self
    {
        return new self();
    }
}

final class Invokable
{
    public function __invoke()
    {
    }
}

class Open
{
}

function helper()
{
}

function run(Plain $plain, Invokable $invokable, Open $open, string $string, array $array, $mixed, ?int $maybe)
{
    $a = $plain->run();
    $b = $plain;
    $c = $plain;
    $d = with($plain, 'strlen');
    $e = with($plain, 'App\helper');
    $f = $plain;
    $f2 = with($plain, 'App\Plain::make');
    $f3 = with($plain, '\App\Plain::make');
    $f4 = $plain;
    $f5 = with($plain, 'Missing::make');
    $f6 = with($plain, 'App\Open::missing');
    $f7 = $plain;
    $f8 = $plain;
    $f9 = $plain;
    $f10 = $plain;
    $g = $plain;
    $h = with($plain, $invokable);
    $i = with($plain, $open);
    $j = with($plain, $string);
    $k = with($plain, $array);
    $l = with($plain, $mixed);
    $m = $plain;
    $n = with($plain, 1.5, 2);
    $o = with();
    $p = (new Plain)->run();
    $r = $plain;
    $s = with($plain, fn () => 1);
}
