<?php

foreach ($a as $b) { if ($b) { { continue; foo(); } } }
-----
<?php

foreach ($a as $b) { if (!($b)) { continue; }

{ continue; foo(); } }
