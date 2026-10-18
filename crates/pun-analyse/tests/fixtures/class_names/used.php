<?php

namespace ClassNamesUse;

use ClassNames\FOO;
use ClassNames\Foo as Aliased;
use ClassNames\FOO as Shouted;
use ClassNames\{BAR, Kind};
use function strlen as Len;

class UsesThem extends FOO implements BAR
{
    use \ClassNames\BAZ;

    public \ClassNames\FOO $typed;

    public function __construct(public ?FOO $promoted = null)
    {
    }

    public function sign(FOO $a, \ClassNames\KIND $b): Aliased
    {
        return new Aliased();
    }

    public function shouted(Shouted $s): ?Shouted
    {
        return null;
    }

    public function body(object $o): void
    {
        $o instanceof FOO;
        try {
        } catch (\ClassNames\FAILURE $e) {
        }
        new FOO();
        new \ClassNames\foo();
        \ClassNames\STATICS::make();
        \ClassNames\MAGIC::missing();
        echo FOO::class;
        echo \DATETIME::ATOM;
        new Shouted();
        new shouted();
        $f = function (FOO $x): FOO {
            return $x;
        };
        $g = fn (FOO $x): ?FOO => $x;
    }
}

enum Suit implements BAR
{
}

interface Extended extends BAR
{
}

function free(FOO $x): FOO
{
    return $x;
}

#[\ATTRIBUTE]
class Attributed
{
}
