<?php

final class FirstStatement
{
    public function first()
    {
        if (version_compare(PHP_VERSION, '8.4.0', '>=')) {
            $mode = 1;

            echo $mode;

            return;
        }

        echo 2;
    }

    public function middle()
    {
        echo 0;

        if (version_compare(PHP_VERSION, '8.4.0', '>=')) {
            $mode = 1;

            echo $mode;
        }

        echo 2;
    }

    public function single()
    {
        echo 0;

        if (version_compare(PHP_VERSION, '8.4.0', '>=')) {
            echo 1;
        }

        echo 2;
    }

    public function firstSingle()
    {
        if (version_compare(PHP_VERSION, '8.4.0', '>=')) {
            echo 1;
        }

        echo 2;
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
        return;
        echo 2;
    }

    public function middle()
    {
        echo 0;
        $mode = 1;
        echo $mode;

        echo 2;
    }

    public function single()
    {
        echo 0;

        echo 1;

        echo 2;
    }

    public function firstSingle()
    {
        echo 1;

        echo 2;
    }
}
