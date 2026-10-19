<?php

namespace App;

use Other\{Thing, function helper};
use App\{Run as R};
use App as Ns;

final class Run
{
    public function go()
    {
        R::walk(1, 2);
        Ns\Run::walk(1, 2);
        helper();
        return new Thing();
    }

    private static function walk($a, $b)
    {
        return $a;
    }
}
-----
<?php

namespace App;

use Other\{Thing, function helper};
use App\{Run as R};
use App as Ns;

final class Run
{
    public function go()
    {
        R::walk(1);
        Ns\Run::walk(1);
        helper();
        return new Thing();
    }

    private static function walk($a)
    {
        return $a;
    }
}
