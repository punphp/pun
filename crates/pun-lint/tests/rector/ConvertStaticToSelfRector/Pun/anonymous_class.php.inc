<?php

// Pun: in a nested anonymous class `static` is that class, so its `static::` stays (Rector checks it against the
// outer class).
class Base
{
    private static function m() {}

    public function run()
    {
        $o = new class {
            public function x() { return static::m(); }
        };
    }
}
