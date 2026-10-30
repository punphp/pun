<?php

namespace App;

use stdClass;

class A
{
    public function a(stdClass $var, ?stdClass $maybe, \DateTime $d)
    {
        // lead comment
        if ($var instanceof stdClass) {
            // inner
            echo 1;

            echo 2;
        }

        if ($var instanceof \stdClass && ($maybe || $d)) {
            echo 3;
        }

        if ($maybe instanceof stdClass) {
            echo 4;
        }

        if ($d instanceof \DateTimeInterface) {
            echo 5;
        }

        if (!($x = new stdClass()) instanceof stdClass) {
            echo 6;
        }

        if (($y = new stdClass()) instanceof stdClass) {
            echo 7;
        }

        foreach ([1] as $i) if ($var instanceof stdClass) echo 8;

        if ($this instanceof A) {
            echo 9;
        }
    }
}
-----
<?php

namespace App;

use stdClass;

class A
{
    public function a(stdClass $var, ?stdClass $maybe, \DateTime $d)
    {
        // inner
        echo 1;
        echo 2;
        if ($maybe || $d) {
            echo 3;
        }

        if ($maybe instanceof stdClass) {
            echo 4;
        }
        echo 5;
        $x = new stdClass();
        echo 6;
        $y = new stdClass();
        echo 7;

        foreach ([1] as $i) echo 8;

        if ($this instanceof A) {
            echo 9;
        }
    }
}
