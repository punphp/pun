<?php

namespace App;

final class Fin {}
class Open {}
enum E { case A; }

final class FinalThis
{
    public function run(Fin $fin, Open $open, E $e, object $o)
    {
        $a = $this::class == 'x';
        $b = $fin::class == 'x';
        $c = $open::class == 'x';
        $d = $e::class == 'x';
        $f = $o::class == 'x';
        $g = static::class == 'x';
        $h = self::class == 'x';
        $i = Open::class == 'x';
        $j = Nope::class == 'x';
        $k = 'App\Open' == 'x';
        $l = 'App\open' == 'x';
        $m = get_class($open) == 'x';
    }
}
-----
<?php

namespace App;

final class Fin {}
class Open {}
enum E { case A; }

final class FinalThis
{
    public function run(Fin $fin, Open $open, E $e, object $o)
    {
        $a = $this::class == 'x';
        $b = $fin::class == 'x';
        $c = $open::class === 'x';
        $d = $e::class == 'x';
        $f = $o::class === 'x';
        $g = static::class == 'x';
        $h = self::class == 'x';
        $i = Open::class == 'x';
        $j = Nope::class === 'x';
        $k = 'App\Open' == 'x';
        $l = 'App\open' === 'x';
        $m = get_class($open) == 'x';
    }
}
