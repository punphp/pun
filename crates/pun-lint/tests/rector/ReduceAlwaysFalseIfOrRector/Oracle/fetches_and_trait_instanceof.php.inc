<?php

namespace Oracle\AlwaysFalseFetches;

trait Checks
{
    public function check(int $n)
    {
        if ($n instanceof \Countable || $n > 5) {
            return 1;
        }
        if (!is_int($n) || $n > 5) {
            return 2;
        }
        return 0;
    }
}

final class Holder
{
    private int $count = 0;

    private static int $total = 0;

    public function run(int $n)
    {
        if (!is_array([$this->count]) || $n > 1) {
            echo 1;
        }
        if (!is_array([self::$total]) || $n > 1) {
            echo 2;
        }
        if (!is_array([[1][0]]) || $n > 1) {
            echo 3;
        }
    }
}
-----
<?php

namespace Oracle\AlwaysFalseFetches;

trait Checks
{
    public function check(int $n)
    {
        if ($n instanceof \Countable || $n > 5) {
            return 1;
        }
        if ($n > 5) {
            return 2;
        }
        return 0;
    }
}

final class Holder
{
    private int $count = 0;

    private static int $total = 0;

    public function run(int $n)
    {
        if (!is_array([$this->count]) || $n > 1) {
            echo 1;
        }
        if (!is_array([self::$total]) || $n > 1) {
            echo 2;
        }
        if (!is_array([[1][0]]) || $n > 1) {
            echo 3;
        }
    }
}
