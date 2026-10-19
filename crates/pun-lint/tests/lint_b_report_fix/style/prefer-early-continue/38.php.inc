<?php

for (;;) { if ($b) { foo(); } }
-----
<?php

for (;;) { if (!($b)) { continue; }

foo(); }
