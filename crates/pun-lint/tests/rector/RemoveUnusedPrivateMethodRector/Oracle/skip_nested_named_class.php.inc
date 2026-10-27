<?php

namespace Pun\Oracle\UnusedPrivate;

final class SkipNestedNamedClass
{
    public function run()
    {
        class NestedNamedInner
        {
            public function go()
            {
            }
        }
    }

    private static function other()
    {
    }
}
