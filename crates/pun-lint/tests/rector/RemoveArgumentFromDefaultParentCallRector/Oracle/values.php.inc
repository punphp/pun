<?php

namespace Pun\Oracle\DefaultArg2;

class Base
{
    const A = 1;

    public function neg($a, $b = -1)
    {
    }

    public function legacy($a, $b = array())
    {
    }

    public function classConst($a, $b = 1)
    {
    }

    public function arr($a, $b = [1 => 'a'])
    {
    }

    public function float($a, $b = 1.5)
    {
    }
}

abstract class Child extends Base
{
    const B = 2;

    abstract public function abs();

    public function run($k, $s)
    {
        parent::neg($k, -1);
        parent::legacy($k, array());
        parent::classConst($k, self::B);
        parent::classConst($k, static::A);
        parent::arr($k, [1 => 'b']);
        parent::arr($k, [$k => 'a']);
        parent::arr($k, [...$s]);
        parent::float($k, 1.5);
        parent::nothing($k, 1);
        self::neg($k, -1);

        return parent::neg($k, -1);
    }
}
-----
<?php

namespace Pun\Oracle\DefaultArg2;

class Base
{
    const A = 1;

    public function neg($a, $b = -1)
    {
    }

    public function legacy($a, $b = array())
    {
    }

    public function classConst($a, $b = 1)
    {
    }

    public function arr($a, $b = [1 => 'a'])
    {
    }

    public function float($a, $b = 1.5)
    {
    }
}

abstract class Child extends Base
{
    const B = 2;

    abstract public function abs();

    public function run($k, $s)
    {
        parent::neg($k);
        parent::legacy($k);
        parent::classConst($k, self::B);
        parent::classConst($k, static::A);
        parent::arr($k, [1 => 'b']);
        parent::arr($k, [$k => 'a']);
        parent::arr($k, [...$s]);
        parent::float($k);
        parent::nothing($k, 1);
        self::neg($k, -1);

        return parent::neg($k, -1);
    }
}
