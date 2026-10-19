<?php

$e = function ($p, $s, $o) {
    preg_match($p, $s, $o->m);
    return 1;
};
-----
<?php

$e = function ($p, $s, $o): int {
    preg_match($p, $s, $o->m);
    return 1;
};
