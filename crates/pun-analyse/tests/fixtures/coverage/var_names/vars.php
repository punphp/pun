<?php

namespace VarNames;

/** @var int $top */
$fromTop = 1;

/** @var int $missing */
echo $fromTop;

/** @var int */
interface Shape
{
}

/** @var int */
enum Suit
{
    /** @var int */
    case Hearts;
}

trait Helper
{
}

/** @var int $nope */
function placed(): void
{
}

class Holder
{
    /** @var int */
    use Helper;

    /** @var int $a */
    public function method(int $a): void
    {
        /** @var int $a */
        echo $a;

        /** @var int $later */
        echo 1;

        $later = 1;

        /** @var string $_GET */
        echo 2;

        /** @var Holder $this */
        echo 3;
    }

    public static function statics(): void
    {
        /** @var Holder $this */
        echo 1;
    }

    /**
     * @param list<int> $list
     */
    public function loops(array $list): void
    {
        /** @var int */
        foreach ($list as $value) {
        }

        /** @var int */
        foreach ($list as $key => $value) {
        }

        /** @var int $nothing */
        foreach ($list as $key => [$first, $second]) {
        }

        /** @var int $x */
        foreach (range(1, 3) as $value) {
        }

        while (true) {
            /** @var int $inLoop */
            echo 1;
            $inLoop = 1;
            if ($inLoop) {
                break;
            }
        }

        for ($i = 0; $i < 3; $i++) {
        }

        do {
        } while (false);
    }

    public function assignments(): void
    {
        /**
         * @var int
         * @var string
         */
        $a = 1;

        /** @var int */
        [$b, $c] = [1, 2];

        /** @var int $other */
        $d = 1;

        /**
         * @var int $other
         * @var int $another
         */
        [$e, $f] = [1, 2];

        /** @var int $e */
        $g = &$e;

        /** @var int */
        $h['k'] = 1;
    }

    public function statements(\Throwable $t): mixed
    {
        /** @var int */
        static $one;

        /** @var int */
        static $two, $three;

        /** @var int $four */
        static $five = 1;

        /** @var int */
        global $g1;

        /** @var int */
        global $g2, $g3, $g2;

        /** @var int $g4 */
        global $g5;

        /** @var int */
        global $g6, $$g5;

        /** @var int $k1 */
        ['x' => $k1, 'y' => $k2] = ['x' => 1, 'y' => 2];

        /** @var int $k4 */
        [, $k3] = [1, 2];

        /** @var int $t */
        return 1;
    }

    public function throws(\Throwable $t): void
    {
        /** @var \Exception */
        throw $t;
    }

    public function throwsTwice(\Throwable $t): void
    {
        /**
         * @var int
         * @var int
         */
        throw $t;
    }

    public function returns(): void
    {
        /** @var int */
        return;
    }

    public function writes(string $s, \stdClass $o): void
    {
        preg_match('/a/', $s, $matches);
        unknown_function($unknownArg);
        $o->call($methodArg);
        self::staticCall($staticArg);
        $k++;
        --$j;
        try {
            $this->returns();
        } catch (\Exception $caught) {
        }
        $fn = function () use (&$byRef, $byValue) {
            /** @var int $byValue */
            echo $byValue;

            /** @var int $outer */
            echo 1;
        };
        $arrow = fn() => $arrowOnly = 1;
        list('a' => $listed) = ['a' => 1];

        /**
         * @var int $matches
         * @var int $unknownArg
         * @var int $methodArg
         * @var int $staticArg
         * @var int $k
         * @var int $j
         * @var int $caught
         * @var int $byRef
         * @var int $arrowOnly
         * @var int $listed
         */
        echo 1;

        /** @var int $end */
    }

    public static function staticCall(mixed &$x): void
    {
    }

    public function extracted(array $a): void
    {
        /** @var int $before */
        echo 1;

        extract($a);

        /** @var int $after */
        echo 2;
    }

    public function inner(): void
    {
        function nested(): void
        {
        }

        new class {
        };
    }

    public function hook(): void
    {
        /** @var int $x */
        /** @var int $y */
        echo 1;
    }
}

/** @var int */
class Plain
{
}

abstract class WithHooks
{
    /** @var int $x */
    abstract public function nothing(): void;

    public int $q {
        get {
            /** @var int $value */
            return 1;
        }
    }

    public int $r {
        set {
            /** @var int $value */
            echo $value;
        }
    }

    public int $s {
        get => 1;
    }

    public int $p {
        set(int $value) {
            /** @var int $anything */
            echo $value;
        }
    }
}
