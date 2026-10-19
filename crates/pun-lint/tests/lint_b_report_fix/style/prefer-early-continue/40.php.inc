<?php

while (1) { if ($b) { foo(); } }
-----
<?php

while (1) { if (!($b)) { continue; }

foo(); }
