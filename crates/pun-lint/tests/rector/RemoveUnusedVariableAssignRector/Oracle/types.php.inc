<?php

namespace App;

final class Lock
{
    public function __destruct()
    {
    }
}

final class Child extends Lock
{
}

#[\NoDiscard]
function important(): int
{
    return 1;
}

final class Types
{
    public function immutable(\DateTimeImmutable $d): void
    {
        $x = $d->modify('+1 day');
        $y = new \DateTime();
        $z = $y->modify('+1 day');
    }

    public function noDiscardFunction(): void
    {
        $a = important();
        $b = \strlen('x');
    }

    public function destructInherited(): void
    {
        $c = new Child();
        $d = new Lock();
        $e = new \stdClass();
    }

    public function nullResets(bool $flag): void
    {
        $f = new \ArrayObject([]);
        echo count($f);
        $f = null;
        $g = $flag ? new \ArrayObject([]) : null;
        echo $g;
        $g = null;
        $h = new Lock();
        $h = null;
        $i = new \SplObjectStorage();
        if ($flag) {
            $i = new \ArrayObject([]);
        }
        echo $i;
        $i = null;
    }

    public function staticCalls(): void
    {
        $j = self::make();
        $k = static::make();
    }

    #[\NoDiscard]
    public static function make(): int
    {
        return 1;
    }
}
-----
<?php

namespace App;

final class Lock
{
    public function __destruct()
    {
    }
}

final class Child extends Lock
{
}

#[\NoDiscard]
function important(): int
{
    return 1;
}

final class Types
{
    public function immutable(\DateTimeImmutable $d): void
    {
        $x = $d->modify('+1 day');
        $y = new \DateTime();
        $y->modify('+1 day');
    }

    public function noDiscardFunction(): void
    {
        $a = important();
    }

    public function destructInherited(): void
    {
        $c = new Child();
        $d = new Lock();
        new \stdClass();
    }

    public function nullResets(bool $flag): void
    {
        $f = new \ArrayObject([]);
        echo count($f);
        $f = null;
        $g = $flag ? new \ArrayObject([]) : null;
        echo $g;
        $h = new Lock();
        $i = new \SplObjectStorage();
        if ($flag) {
            $i = new \ArrayObject([]);
        }
        echo $i;
    }

    public function staticCalls(): void
    {
        $j = self::make();
        $k = static::make();
    }

    #[\NoDiscard]
    public static function make(): int
    {
        return 1;
    }
}
