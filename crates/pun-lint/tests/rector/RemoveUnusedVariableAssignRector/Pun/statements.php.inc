<?php

namespace App;

final class Statements
{
    public function run($a, $db)
    {
        // first
        /** @var int $one */
        $one = (int) (string) $db->query();
        $two = ($db->query());
        $three = [$db->query()];
        $four = $a['k'];
        $five = foo()['k'];
        $six = fn () => $db->query();
        $seven = function () use (&$ref) {
            $ref = 1;
        };
        $ref = 2;
        $eight = new \stdClass();
        $nine = $db->query(
            1,
            2
        );
        $ten = $a;
        $ten = $a;
        echo 'done';
    }

    public function last($a)
    {
        $one = 1;
        $two = 2;
    }

    public function staticAndGlobal()
    {
        static $s;
        global $g;
        $s = 1;
        $g = 2;
        $GLOBALS = [];
    }

    public function byRef(&$p)
    {
        $p = 1;
        $q = 1;
        $q = &$p;
        $q = 2;
    }

    public function caseInsensitive()
    {
        $Name = 1;
        echo $name;
    }

    public function nested()
    {
        if (true) {
            $inner = 1;
        }
        $x = function () {
            $inClosure = 1;
        };
    }
}

function withCompact()
{
    $a = 1;
    $f = fn () => COMPACT('b');
}
-----
<?php

namespace App;

final class Statements
{
    public function run($a, $db)
    {
        // first
        /** @var int $one */
        $db->query();
        $db->query();
        [$db->query()];
        foo()['k'];
        fn () => $db->query();
        new \stdClass();
        $db->query(
            1,
            2
        );
        echo 'done';
    }

    public function last($a)
    {
    }

    public function staticAndGlobal()
    {
        static $s;
        global $g;
        $s = 1;
        $g = 2;
        $GLOBALS = [];
    }

    public function byRef(&$p)
    {
        $p = 1;
        $q = 1;
        $q = &$p;
        $q = 2;
    }

    public function caseInsensitive()
    {
        $Name = 1;
        echo $name;
    }

    public function nested()
    {
        if (true) {
            $inner = 1;
        }
    }
}

function withCompact()
{
    $a = 1;
    $f = fn () => COMPACT('b');
}
