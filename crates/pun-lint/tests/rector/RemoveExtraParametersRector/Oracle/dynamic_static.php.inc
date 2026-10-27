<?php

namespace App;

class S
{
    public static function one($a) {}
}

function f(string $m)
{
    S::$m(1, 2);
    S::{$m}(1, 2);
    S::one(1, 2);
}
-----
<?php

namespace App;

class S
{
    public static function one($a) {}
}

function f(string $m)
{
    S::$m(1, 2);
    S::{$m}(1, 2);
    S::one(1);
}
