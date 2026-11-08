<?php

namespace Pun\Oracle\ArrayKeyExistsNull;

class Magic
{
    public $real;

    public function __get($name)
    {
        return 1;
    }
}

class MagicChild extends Magic
{
}

class Plain
{
    public $p;
    public ?string $s = null;
    public ?int $i = null;

    public function m()
    {
        return 1;
    }

    public function mixedReturn(): mixed
    {
        return 1;
    }
}

trait ArrayKeyTrait
{
    public $tp;
    public mixed $tp2;
    public static $sp;
    public static ?int $si = null;

    public function run($k, mixed $x, $arr, Plain $pl)
    {
        array_key_exists($k, $arr);
        array_key_exists($x, $arr);
        array_key_exists($this->tp, $arr);
        array_key_exists($this->tp2, $arr);
        array_key_exists($this?->tp, $arr);
        array_key_exists($this->m(), $arr);
        array_key_exists($this->m2(), $arr);
        array_key_exists($pl->p, $arr);
        array_key_exists($pl->m(), $arr);
        array_key_exists($pl->mixedReturn(), $arr);
        array_key_exists(self::f(), $arr);
        array_key_exists(static::f(), $arr);
        array_key_exists(self::$sp, $arr);
        array_key_exists(static::$sp, $arr);
        array_key_exists(self::$si, $arr);
        array_key_exists(Plain::$nope, $arr);
    }

    public function m2(): mixed
    {
        return 1;
    }

    public static function f()
    {
        return 1;
    }
}

class Typed
{
    public function run(Magic $mg, MagicChild $mc, Plain $pl, Plain|Magic $either, $arr, $items, $abcs)
    {
        array_key_exists($mg->real, $arr);
        array_key_exists($mg->other, $arr);
        array_key_exists($mc->other, $arr);
        array_key_exists($mg?->other, $arr);
        array_key_exists($either->p, $arr);
        array_key_exists($pl->p, $arr);
        array_key_exists($pl->s, $arr);
        array_key_exists($pl->i, $arr);
        array_key_exists($pl->m(), $arr);
        array_key_exists($items, $arr);
        array_key_exists($abcs, $arr);
        array_key_exists($undefineds, $arr);
        array_key_exists(1.5, $arr);
        array_key_exists(true, $arr);
        array_key_exists($pl, $arr);
        array_key_exists(PHP_EOL, $arr);
        array_key_exists(<<<TXT
            a{$items}
            TXT, $arr);
        array_key_exists(`ls`, $arr);
    }
}
