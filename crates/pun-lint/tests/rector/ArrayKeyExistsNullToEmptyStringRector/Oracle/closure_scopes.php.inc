<?php

namespace Pun\Oracle\ArrayKeyExistsNull;

class Scopes
{
    public function run($arr)
    {
        $p = 1;
        $f = function ($p) use ($arr) {
            array_key_exists($p, $arr);
            array_key_exists($p->x->y, $arr);
        };
        $o = function () use ($p, $arr) {
            $c = function () use ($p, $arr) {
                array_key_exists($p, $arr);
            };
        };
    }
}
