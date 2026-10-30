<?php
function f($a, $b, $c) {
    if ($a) {
        echo 1;
    } /* x */ elseif ($b) {
    } /* y */ elseif ($c) {
        echo 2;
    } /* z */ elseif ($d) {
    } // w
    else {
    }
}
-----
<?php
function f($a, $b, $c) {
    if ($a) {
        echo 1;
    } /* y */ elseif ($c) {
        echo 2;
    }
}
