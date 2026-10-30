<?php
function f($a, $b, $c) {
    if ($a): echo 1; elseif ($b): else: echo 2; endif;
    if ($a): elseif ($b): echo 1; elseif ($c): echo 2; endif;
    if ($a): elseif ($b): echo 1; else: echo 2; endif;
}
-----
<?php
function f($a, $b, $c) {
    if ($a) {
        echo 1;
    } else {
        echo 2;
    }
    if (!$a && $b) {
        echo 1;
    } elseif ($c) {
        echo 2;
    }
    if (!$a && $b) {
        echo 1;
    } else {
        echo 2;
    }
}
