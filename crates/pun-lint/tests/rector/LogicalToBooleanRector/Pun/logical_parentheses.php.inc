<?php

f((($a = foo()) and ($x and $y)) or $z);
-----
<?php

f(($a = foo()) && ($x && $y) || $z);
