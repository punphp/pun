<?php

$r = ($a xor $b) and $c;
-----
<?php

($r = ($a xor $b)) && $c;
