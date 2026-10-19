<?php

for (;;): if ($b) { foo(); } endfor;
-----
<?php

for (;;): if (!($b)) { continue; }

foo(); endfor;
