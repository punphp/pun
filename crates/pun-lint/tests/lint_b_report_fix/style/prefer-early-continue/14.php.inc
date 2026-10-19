<?php

foreach ($a as $b) if ($b) /* c */ foo();
-----
<?php

foreach ($a as $b) if (!($b)) { continue; }
 /* c */ foo();
