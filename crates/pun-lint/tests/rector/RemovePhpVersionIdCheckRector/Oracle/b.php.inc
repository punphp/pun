<?php

final class FirstStatement
{
    public function first()
    {
        // first
        if (PHP_VERSION_ID >= 80000) {
            $mode = 1;

            echo $mode;
        }

        // next
        echo 2;
    }

    public function middle()
    {
        echo 0;

        // middle
        if (PHP_VERSION_ID >= 80000) {
            $mode = 1;
            echo $mode;
        }

        echo 2;
    }

    public function last()
    {
        echo 0;

        if (PHP_VERSION_ID >= 80000) {
            $mode = 1;
            echo $mode;
        }
    }
}
-----
<?php

final class FirstStatement
{
    public function first()
    {
        $mode = 1;
        echo $mode;
        // next
        echo 2;
    }

    public function middle()
    {
        echo 0;
        $mode = 1;
        echo $mode;

        echo 2;
    }

    public function last()
    {
        echo 0;
        $mode = 1;
        echo $mode;
    }
}
