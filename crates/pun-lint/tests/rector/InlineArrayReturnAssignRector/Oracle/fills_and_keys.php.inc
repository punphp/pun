<?php

namespace Oracle\InlineArray;

const TOP = 'top';

final class Keys
{
    public const FOO = 'foo';
    public const NUM = 7;

    public function comments()
    {
        // first
        $a = [];
        /** about b */
        $a['b'] = 1;
        $a['c'] = 2; // trailing

        // before return
        return $a;
    }

    public function multiline()
    {
        $a = [];
        $a['x'] = foo(
            1,
            2
        );
        $a['y'] = [
            'z' => 1,
        ];

        return $a;
    }

    public function keyKinds()
    {
        $a = [];
        $a["dq"] = 1;
        $a['1'] = 2;
        $a[3] = 3;
        $a[0x10] = 4;
        $a[-2] = 5;
        $a[1.7] = 6;
        $a[true] = 7;
        $a[null] = 8;
        $a[self::FOO] = 10;
        $a[self::NUM] = 11;
        $a[Keys::FOO] = 12;
        $a['it\'s'] = 14;

        return $a;
    }

    public function appendAfterKey()
    {
        $a = [];
        $a['k'] = 1;
        $a[] = 2;
        $a[] = 3;

        return $a;
    }

    public function overwrite()
    {
        $a = [];
        $a['k'] = 1;
        $a['m'] = 2;
        $a['k'] = 3;

        return $a;
    }

    public function nestedAfterArray()
    {
        $a = [];
        $a['k'] = [];
        $a['k']['m'] = 2;

        return $a;
    }

    public function noInit()
    {
        $a['k'] = 1;
        $a['m'] = new \stdClass();

        return $a;
    }

    public function newWithArgs()
    {
        $a = [];
        $a['k'] = new \ArrayObject([]);
        $a['m'] = 1;

        return $a;
    }
}

function oracle_inline_array_closure()
{
    $f = function () use (&$a) {
        $a['k'] = 1;
        $a['m'] = 2;

        return $a;
    };
    $g = function () {
        $b = array();
        $b['k'] = 1;
        return $b;
    };
}
-----
<?php

namespace Oracle\InlineArray;

const TOP = 'top';

final class Keys
{
    public const FOO = 'foo';
    public const NUM = 7;

    public function comments()
    {
        return ['b' => 1, 'c' => 2];
    }

    public function multiline()
    {
        return ['x' => foo(
            1,
            2
        ), 'y' => [
            'z' => 1,
        ]];
    }

    public function keyKinds()
    {
        return ['dq' => 1, 7, 3 => 3, 16 => 4, -2 => 5, '' => 8, 'foo' => 12, 11, 'it\'s' => 14];
    }

    public function appendAfterKey()
    {
        return ['k' => 1, 2, 3];
    }

    public function overwrite()
    {
        return ['k' => 3, 'm' => 2];
    }

    public function nestedAfterArray()
    {
        $a = [];
        $a['k'] = [];
        $a['k']['m'] = 2;

        return $a;
    }

    public function noInit()
    {
        return ['k' => 1, 'm' => new \stdClass()];
    }

    public function newWithArgs()
    {
        $a = [];
        $a['k'] = new \ArrayObject([]);
        $a['m'] = 1;

        return $a;
    }
}

function oracle_inline_array_closure()
{
    $f = function () use (&$a) {
        $a['k'] = 1;
        $a['m'] = 2;

        return $a;
    };
    $g = function () {
        return ['k' => 1];
    };
}
