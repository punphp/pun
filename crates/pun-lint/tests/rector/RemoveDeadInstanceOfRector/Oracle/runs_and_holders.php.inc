<?php

namespace App;

use stdClass;

class C
{
    public function runs(stdClass $var)
    {
        echo 0;

        if ($var instanceof stdClass) {
            echo 1;
        }

        if (!$var instanceof stdClass) {
            echo 2;
        }

        echo 3;
    }

    public function nopAfter(stdClass $var)
    {
        if ($var instanceof stdClass) {
            echo 1;
            echo 2;
        }
        // trailing comment
    }

    public function nopInside(stdClass $var)
    {
        echo 0;
        if ($var instanceof stdClass) {
            echo 1;
            // inside trailing
        }
        echo 3;
    }

    public function multiline(stdClass $var)
    {
        echo 0;
        if ($var instanceof stdClass) {
            $a = [
                'x' => 1,
            ];
            $b = <<<TXT
                text
                TXT;
        }
    }

    public function whileHolder(stdClass $var)
    {
        while (true) {
            if ($var instanceof stdClass) {
                echo 1;
                echo 2;
            }
        }
    }

    public function foreachKey(stdClass $var, array $list)
    {
        foreach ($list as $key => $value) {
            if ($var instanceof stdClass) {
                echo $key;
                echo $value;
            }
        }
    }
}
-----
<?php

namespace App;

use stdClass;

class C
{
    public function runs(stdClass $var)
    {
        echo 0;
        echo 1;

        echo 3;
    }

    public function nopAfter(stdClass $var)
    {
        echo 1;
        echo 2;
        // trailing comment
    }

    public function nopInside(stdClass $var)
    {
        echo 0;
        echo 1;
        // inside trailing

        echo 3;
    }

    public function multiline(stdClass $var)
    {
        echo 0;
        $a = [
            'x' => 1,
        ];
        $b = <<<TXT
                text
                TXT;
    }

    public function whileHolder(stdClass $var)
    {
        while (true) {
            echo 1;
            echo 2;
        }
    }

    public function foreachKey(stdClass $var, array $list)
    {
        foreach ($list as $key => $value) {
            echo $key;
            echo $value;
        }
    }
}
