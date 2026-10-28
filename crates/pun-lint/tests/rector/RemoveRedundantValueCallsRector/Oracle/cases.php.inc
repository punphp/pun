<?php

namespace App;

interface Shape
{
}

final class Square implements Shape
{
    public function area(): int
    {
        return 1;
    }
}

enum Suit
{
    case Hearts;
}

function run(int $int, array $list, ?Square $square, Shape $shape, callable $callable, \Closure $closure, object $object, $mixed, Suit $suit, iterable $items, bool $flag)
{
    $a = value($int) + 1;
    $b = value($list)[0];
    $c = value($square)?->area();
    $d = value($shape);
    $e = value($callable);
    $f = value($closure);
    $g = value($object);
    $h = value($mixed);
    $i = value($suit);
    $j = value($items);
    $k = value(null);
    $m = value(new Square())->area();
    $n = value(new Square)::class;
    $o = value($int, 1);
    $q = \value('x');
    $r = value(fn () => 1);
    $s = value($flag ? $int : $list);
    $t = value($flag ? $int : $closure);
}
-----
<?php

namespace App;

interface Shape
{
}

final class Square implements Shape
{
    public function area(): int
    {
        return 1;
    }
}

enum Suit
{
    case Hearts;
}

function run(int $int, array $list, ?Square $square, Shape $shape, callable $callable, \Closure $closure, object $object, $mixed, Suit $suit, iterable $items, bool $flag)
{
    $a = $int + 1;
    $b = $list[0];
    $c = $square?->area();
    $d = $shape;
    $e = value($callable);
    $f = value($closure);
    $g = value($object);
    $h = value($mixed);
    $i = $suit;
    $j = $items;
    $k = null;
    $m = (new Square())->area();
    $n = (new Square)::class;
    $o = value($int, 1);
    $q = 'x';
    $r = value(fn () => 1);
    $s = $flag ? $int : $list;
    $t = value($flag ? $int : $closure);
}
