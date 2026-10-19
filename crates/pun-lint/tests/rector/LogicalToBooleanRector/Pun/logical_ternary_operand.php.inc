<?php

foo() and $c . ($b ? 1 : 2);
-----
<?php

foo() && $c . ($b ? 1 : 2);
