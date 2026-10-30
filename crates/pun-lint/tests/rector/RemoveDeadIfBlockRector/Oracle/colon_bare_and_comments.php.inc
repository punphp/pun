<?php
function f($a, $b, $c) {
    if ($a): echo 1; else: endif;
    if ($a): echo 1; elseif ($b): endif;
    if ($a): echo 1; elseif ($b): elseif ($c): echo 3; else: endif;
    if ($a) echo 1; elseif ($b) {} else echo 2;
    if ($a) {
        echo 1;
    } /* x */ elseif ($b) {
    } // y
    else {
    }
    // keep
    if ($a) {
    }
    if ($a) {
    } elseif ($b = f()) {
    }
    if ($a) {
    } elseif ($b) {
        echo 1;
    } elseif ($c) {
        echo 2;
    }
    if ($a) {
    } elseif ($b and $c) {
        echo 1;
    }
    if ($a || $b) {
    } elseif ($c) {
        echo 1;
    }
    if ($a == 1) {
    } elseif ($c ? 1 : 2) {
        echo 1;
    }
    if ($a instanceof X) {
    } else {
        echo 1;
    }
    while ($a) if ($b) {} else {echo 1;}
    while ($a) if ($b) {}
}
-----
<?php
function f($a, $b, $c) {
    if ($a): echo 1; endif;
    if ($a) {
        echo 1;
    }
    if ($a): echo 1; elseif ($c): echo 3; endif;
    if ($a) {
        echo 1;
    } else echo 2;
    if ($a) {
        echo 1;
    }
    // keep
    if ($a) {
    }
    if (!$a && $b = f()) {
    }
    if (!$a && $b) {
        echo 1;
    } elseif ($c) {
        echo 2;
    }
    if (!$a && ($b and $c)) {
        echo 1;
    }
    if (!($a || $b) && $c) {
        echo 1;
    }
    if ($a != 1 && ($c ? 1 : 2)) {
        echo 1;
    }
    if (!$a instanceof X) {
        echo 1;
    }
    while ($a) if (!$b) {
        echo 1;
    }
    while ($a) {
    }
}
