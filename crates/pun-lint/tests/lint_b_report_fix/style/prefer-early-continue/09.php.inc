<?php

foreach ($a as $b) { if ($b > 1) { foo(); } }
-----
<?php

foreach ($a as $b) { if ($b <= 1) { continue; }

foo(); }
