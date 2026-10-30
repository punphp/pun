<?php

namespace Oracle\InlineArray2;

const TOP = 'top';

interface Shape
{
    public const SIDES = 4;
}

enum Suit: string
{
    case Hearts = 'h';
}

class Base
{
    public const PARENT_KEY = 'p';
    public const REF = TOP;
    public const EXPR = 'a' . 'b';
}

final class Keys extends Base implements Shape
{
    public function classConstants()
    {
        $a = [];
        $a[parent::PARENT_KEY] = 1;
        $a[self::SIDES] = 2;
        $a[Base::REF] = 3;
        $a[Base::EXPR] = 4;
        $a[\PDO::PARAM_INT] = 8;
        $a[Base::class] = 9;
        $a[self::class] = 10;

        return $a;
    }

    public function heredoc()
    {
        $a = [];
        $a["1"] = 2;
        $a["01"] = 3;
        $a[-0] = 4;
        $a["-5"] = 5;
        $a[0b11] = 6;
        $a[1_000] = 7;
        $a[+3] = 8;

        return $a;
    }

    public function indent()
    {
        $a = [];
        $a['k'] =
            foo(
                1
            );

        return $a;
    }
}
-----
<?php

namespace Oracle\InlineArray2;

const TOP = 'top';

interface Shape
{
    public const SIDES = 4;
}

enum Suit: string
{
    case Hearts = 'h';
}

class Base
{
    public const PARENT_KEY = 'p';
    public const REF = TOP;
    public const EXPR = 'a' . 'b';
}

final class Keys extends Base implements Shape
{
    public function classConstants()
    {
        return ['p' => 1, 4 => 2, 'top' => 3, 'ab' => 4, 1 => 8, 'Oracle\InlineArray2\Base' => 9, 'Oracle\InlineArray2\Keys' => 10];
    }

    public function heredoc()
    {
        return [1 => 2, '01' => 3, 0 => 4, -5 => 5, 3 => 8, 1000 => 7];
    }

    public function indent()
    {
        return ['k' => foo(
            1
        )];
    }
}
