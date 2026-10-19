<?php

while (1): if ($b) { foo(); } endwhile;
-----
<?php

while (1): if (!($b)) { continue; }

foo(); endwhile;
