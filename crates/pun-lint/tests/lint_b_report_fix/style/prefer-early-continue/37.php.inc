<?php

foreach ($a as $b) { if ($b) { echo 1; } }
-----
<?php

foreach ($a as $b) { if (!($b)) { continue; }

echo 1; }
