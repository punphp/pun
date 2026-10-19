<?php

do { if ($b) { foo(); } } while (1);
-----
<?php

do { if (!($b)) { continue; }

foo(); } while (1);
