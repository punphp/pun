<?php

$a = now('UTC')->startOfDay();
$b = NOW()->STARTOFDAY(true);
$c = \now()->startOfDay;
$d = (now())->startOfDay();
-----
<?php

$a = today();
$b = today();
$c = \now()->startOfDay;
$d = today();
