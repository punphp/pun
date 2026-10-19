<?php

foreach ($a as $b) { if ($b) { /* c */ foo(); /* d */ } }
-----
<?php

foreach ($a as $b) { if (!($b)) { continue; }
 /* c */ foo(); /* d */  }
