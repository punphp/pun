<?php

enum PunSuit
{
    case Hearts;

    public function label(): string
    {
        return 'h';
    }
}

class PunWithCtor
{
    public function __construct($a)
    {
    }

    public function go()
    {
    }
}

class PunWithDefaultCtor
{
    public function __construct($a = 1)
    {
    }

    public function go()
    {
    }
}

class PunParent
{
}

#[Attribute]
class PunAttr
{
    public function __construct($callable)
    {
    }
}

final class PunArrayCallables extends PunParent
{
    const NAME = 'abc';

    #[PunAttr([self::class, 'inAttribute'])]
    public function run(PunWithCtor|PunWithDefaultCtor $either, $fn, $obj)
    {
        $m = 'viaVariable';
        $list = [
            array($this, 'legacy'),
            [0 => $this, 1 => 'keyed'],
            ['a' => $this, 'b' => 'named'],
            [$this, 'conc' . 'at'],
            [$this, '__construct'],
            [$this, 'missing'],
            [$this, null],
            [$this, $m],
            [self::NAME, 'constName'],
            [parent::class, 'parentClass'],
            [$obj::class, 'objClass'],
            [$either, 'union'],
            [PunSuit::Hearts, 'label'],
            [PunWithCtor::class, 'go'],
            [PunWithDefaultCtor::class, 'go'],
        ];
        [, $second] = $list;
        $fn([$this, 'viaClosureCall']);
        PunSuit::Hearts->label();
        $this->firstClass(...);
        SELF::upperSelf();
        $this->fail()->afterNever();

        return $second;
    }

    private function fail(): never
    {
        throw new \RuntimeException();
    }

    private function afterNever()
    {
    }

    private function inAttribute()
    {
    }

    private function legacy()
    {
    }

    private function keyed()
    {
    }

    private function named()
    {
    }

    private function concat()
    {
    }

    private function viaVariable()
    {
    }

    private function constName()
    {
    }

    private function parentClass()
    {
    }

    private function objClass()
    {
    }

    private function union()
    {
    }

    private function viaClosureCall()
    {
    }

    private function firstClass()
    {
    }

    private static function upperSelf()
    {
    }

    private function unused()
    {
    }
}
-----
<?php

enum PunSuit
{
    case Hearts;

    public function label(): string
    {
        return 'h';
    }
}

class PunWithCtor
{
    public function __construct($a)
    {
    }

    public function go()
    {
    }
}

class PunWithDefaultCtor
{
    public function __construct($a = 1)
    {
    }

    public function go()
    {
    }
}

class PunParent
{
}

#[Attribute]
class PunAttr
{
    public function __construct($callable)
    {
    }
}

final class PunArrayCallables extends PunParent
{
    const NAME = 'abc';

    #[PunAttr([self::class, 'inAttribute'])]
    public function run(PunWithCtor|PunWithDefaultCtor $either, $fn, $obj)
    {
        $m = 'viaVariable';
        $list = [
            array($this, 'legacy'),
            [0 => $this, 1 => 'keyed'],
            ['a' => $this, 'b' => 'named'],
            [$this, 'conc' . 'at'],
            [$this, '__construct'],
            [$this, 'missing'],
            [$this, null],
            [$this, $m],
            [self::NAME, 'constName'],
            [parent::class, 'parentClass'],
            [$obj::class, 'objClass'],
            [$either, 'union'],
            [PunSuit::Hearts, 'label'],
            [PunWithCtor::class, 'go'],
            [PunWithDefaultCtor::class, 'go'],
        ];
        [, $second] = $list;
        $fn([$this, 'viaClosureCall']);
        PunSuit::Hearts->label();
        $this->firstClass(...);
        SELF::upperSelf();
        $this->fail()->afterNever();

        return $second;
    }

    private function fail(): never
    {
        throw new \RuntimeException();
    }

    private function afterNever()
    {
    }

    private function legacy()
    {
    }

    private function keyed()
    {
    }

    private function viaClosureCall()
    {
    }

    private function firstClass()
    {
    }
}
