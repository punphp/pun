<?php

namespace App;

use stdClass;

class B
{
    public function first(stdClass $var)
    {
        if ($var instanceof stdClass) {
            echo 1;
            echo 2;
        }
        // after
        echo 3;
    }

    public function only(stdClass $var, int $count = 0): void
    {
        if ($var instanceof stdClass) {
            // first
            echo 1;

            echo 2;
            // trailing
        }
    }

    public function onlyOne(stdClass $var)
    {
        // before
        if ($var instanceof stdClass) {
            // one
            echo 1;
        }
    }

    public function middle(stdClass $var)
    {
        echo 0;

        /** doc */
        if ($var instanceof stdClass) {
            echo 1;
            echo 2;
        }

        echo 3;
    }

    public function colon(stdClass $var)
    {
        echo 0;
        if ($var instanceof stdClass):
            echo 1;
            echo 2;
        endif;
        echo 3;
    }

    public function nested(stdClass $var)
    {
        foreach ([1, 2] as $i) {
            if ($var instanceof stdClass) {
                if ($i) {
                    echo $i;
                }
                echo 'x';
            }
        }
    }

    public function negated(stdClass $var)
    {
        echo 0;
        if (!$var instanceof stdClass) {
            return;
        }
        if (!($var instanceof stdClass)) {
            return;
        }
        if ($var instanceof stdClass) {
        }
        echo 1;
    }

    public function assigns()
    {
        echo 0;
        if (!($a = new stdClass()) instanceof stdClass) {
            echo 1;
        }
        if (($b = new stdClass()) instanceof stdClass) {
        }
        echo 2;
    }

    public function closures(stdClass $var)
    {
        $f = function () use ($var) {
            if ($var instanceof stdClass) {
                echo 1;
                echo 2;
            }
        };
        return $f;
    }

    public function interfaces(\ArrayObject $o, \DateTimeImmutable $d)
    {
        if ($o instanceof \Countable) {
            echo 1;
        }
        if ($d instanceof \DateTimeInterface && $d > 1) {
            echo 2;
        }
        if ($o instanceof \DateTime) {
            echo 3;
        }
    }
}

function lone(stdClass $var)
{
    if ($var instanceof stdClass) {
        echo 1;
        echo 2;
    }
}
-----
<?php

namespace App;

use stdClass;

class B
{
    public function first(stdClass $var)
    {
        echo 1;
        echo 2;
        // after
        echo 3;
    }

    public function only(stdClass $var, int $count = 0): void
    {
        // first
        echo 1;
        echo 2;
        // trailing
    }

    public function onlyOne(stdClass $var)
    {
        // one
        echo 1;
    }

    public function middle(stdClass $var)
    {
        echo 0;
        echo 1;
        echo 2;

        echo 3;
    }

    public function colon(stdClass $var)
    {
        echo 0;
        echo 1;
        echo 2;
        echo 3;
    }

    public function nested(stdClass $var)
    {
        foreach ([1, 2] as $i) {
            if ($i) {
                echo $i;
            }
            echo 'x';
        }
    }

    public function negated(stdClass $var)
    {
        echo 0;
        echo 1;
    }

    public function assigns()
    {
        echo 0;
        $a = new stdClass();
        echo 1;
        $b = new stdClass();
        echo 2;
    }

    public function closures(stdClass $var)
    {
        $f = function () use ($var) {
            echo 1;
            echo 2;
        };
        return $f;
    }

    public function interfaces(\ArrayObject $o, \DateTimeImmutable $d)
    {
        echo 1;
        if ($d > 1) {
            echo 2;
        }
        if ($o instanceof \DateTime) {
            echo 3;
        }
    }
}

function lone(stdClass $var)
{
    echo 1;
    echo 2;
}
