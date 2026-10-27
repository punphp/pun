<?php

namespace Pun\Oracle\UnusedPrivate;

trait OtherTrait
{
    public function x()
    {
    }
}

trait UsedTrait
{
    public $prop;

    abstract public function abs();

    public function caller($x)
    {
        TraitUser::byName();
        $x::byVar();
        Elsewhere::notOurs();
    }
}

class TraitUser
{
    use UsedTrait;

    public function abs()
    {
    }

    private static function byName()
    {
    }

    private static function byVar()
    {
    }

    private static function notOurs()
    {
    }
}
-----
<?php

namespace Pun\Oracle\UnusedPrivate;

trait OtherTrait
{
    public function x()
    {
    }
}

trait UsedTrait
{
    public $prop;

    abstract public function abs();

    public function caller($x)
    {
        TraitUser::byName();
        $x::byVar();
        Elsewhere::notOurs();
    }
}

class TraitUser
{
    use UsedTrait;

    public function abs()
    {
    }

    private static function byName()
    {
    }
}
