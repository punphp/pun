<?php

foreach ($a as $b) { if (-$b) { foo(); } }
-----
<?php

foreach ($a as $b) { if (!(-$b)) { continue; }

foo(); }
