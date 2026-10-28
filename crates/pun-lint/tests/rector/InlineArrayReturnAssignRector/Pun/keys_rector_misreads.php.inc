<?php

namespace Pun\InlineArray;

const TOP = 'top';

enum Suit: string
{
    case Hearts = 'h';
}

final class Keys
{
    public function globalConstant()
    {
        $a = [];
        $a[TOP] = 1;
        $a['k'] = 2;

        return $a;
    }

    public function magicConstant()
    {
        $a = [];
        $a[__LINE__] = 1;
        $a['k'] = 2;

        return $a;
    }

    public function enumCase()
    {
        $a = [];
        $a[Suit::Hearts] = 1;
        $a['k'] = 2;

        return $a;
    }

    public function unknownConstant()
    {
        $a = [];
        $a[Missing::NOPE] = 1;
        $a['k'] = 2;

        return $a;
    }

    public function heredocKey()
    {
        $a = [];
        $a[<<<'K'
        now
        K] = 1;
        $a['k'] = 2;

        return $a;
    }

    public function nestedAfterScalar($v)
    {
        $a = [];
        $a['k'] = $v;
        $a['k']['m'] = 2;

        return $a;
    }
}
