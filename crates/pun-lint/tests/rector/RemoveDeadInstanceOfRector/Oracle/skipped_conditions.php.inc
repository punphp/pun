<?php

namespace App\Skipped;

use stdClass;

class Other
{
}

class Hooked
{
    public stdClass $item {
        set(stdClass $value) {
            if ($value instanceof stdClass) {
                $this->item = $value;
            }
        }
    }
}

class C
{
    public function negations(stdClass $var, bool $flag)
    {
        if (!$flag) {
            echo 1;
        }

        if ($flag && $var instanceof stdClass) {
            echo 2;
        }
    }

    public function unions(stdClass|Other $var)
    {
        if ($var instanceof stdClass) {
            echo 3;
        }
    }
}
-----
<?php

namespace App\Skipped;

use stdClass;

class Other
{
}

class Hooked
{
    public stdClass $item {
        set(stdClass $value) {
            $this->item = $value;
        }
    }
}

class C
{
    public function negations(stdClass $var, bool $flag)
    {
        if (!$flag) {
            echo 1;
        }

        if ($flag && $var instanceof stdClass) {
            echo 2;
        }
    }

    public function unions(stdClass|Other $var)
    {
        if ($var instanceof stdClass) {
            echo 3;
        }
    }
}
