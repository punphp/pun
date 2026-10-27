<?php

class A
{
    public function run($a, $b, $c)
    {
        if ($a) {
        } elseif ($b) {
            echo 1;

            // note
            echo 2;
        } elseif ($c) {
            echo 3;
        } else {
            echo 4;
        }

        if ($a) {
        } else {
            echo 1;

            echo 2;
        }

        if ($a) {
            echo 1;
        } elseif ($b) {
        } elseif ($c) {
        }

        if ($a): else: echo 1; endif;
        if ($a): elseif ($b): echo 2; endif;
        if ($a) echo 1; else {}
        if ($a) {} else if ($b) {echo 5;}
    }
}
-----
<?php

class A
{
    public function run($a, $b, $c)
    {
        if (!$a && $b) {
            echo 1;
            // note
            echo 2;
        } elseif ($c) {
            echo 3;
        } else {
            echo 4;
        }

        if (!$a) {
            echo 1;
            echo 2;
        }

        if ($a) {
            echo 1;
        }

        if (!$a) {
            echo 1;
        }
        if (!$a && $b) {
            echo 2;
        }
        if ($a) echo 1;
        if (!$a) {
            if ($b) {echo 5;}
        }
    }
}
