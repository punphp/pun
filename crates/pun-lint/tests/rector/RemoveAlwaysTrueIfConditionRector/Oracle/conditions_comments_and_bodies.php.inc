<?php

namespace Oracle\AlwaysTrue;

trait Shared
{
    public function inTrait()
    {
        if (true) {
            echo 'trait';
        }
    }
}

final class Runner
{
    private bool $flag = true;

    public function run(int $n, array $items, \stdClass $o)
    {
        // before
        /** doc */
        if (1 === 1) {
            // inside
            echo 1;
            echo 2;
        }

        if (is_int($n) && $n > 2) {
            echo 3;
        } else {
            echo 4;
        }

        if (true) {
        }

        if (true) {
            echo 5;
        } else {
            echo 6;
        }

        if (true) {
            echo 7;
        } elseif ($n) {
            echo 8;
        }

        if (is_array($items)) {
            echo 9;
        }

        if ($this->flag) {
            echo 10;
        }

        if (function_exists('strlen')) {
            echo 11;
        }

        if (($x = true)) {
            echo $x;
        }

        if ($items) {
            if ($items) {
                echo 12;
            }
        }

        if ($o instanceof \stdClass) {
            echo 13;
        }

        if (is_object($o) && $this->flag) {
            echo 14;
        }

        if (true) echo 15;

        $closure = function () {
            if (true) {
                return 16;
            }
        };

        if (true) {
            echo 17;
        }
    }
}
-----
<?php

namespace Oracle\AlwaysTrue;

trait Shared
{
    public function inTrait()
    {
        if (true) {
            echo 'trait';
        }
    }
}

final class Runner
{
    private bool $flag = true;

    public function run(int $n, array $items, \stdClass $o)
    {
        // before
        /** doc */
        // inside
        echo 1;
        echo 2;
        if (is_int($n) && $n > 2) {
            echo 3;
        } else {
            echo 4;
        }

        if (true) {
            echo 5;
        } else {
            echo 6;
        }

        if (true) {
            echo 7;
        } elseif ($n) {
            echo 8;
        }

        if (is_array($items)) {
            echo 9;
        }

        if ($this->flag) {
            echo 10;
        }

        if (function_exists('strlen')) {
            echo 11;
        }

        if (($x = true)) {
            echo $x;
        }

        if ($items) {
            if ($items) {
                echo 12;
            }
        }

        echo 13;

        if ($this->flag) {
            echo 14;
        }

        echo 15;

        $closure = function () {
            return 16;
        };

        echo 17;
    }
}
