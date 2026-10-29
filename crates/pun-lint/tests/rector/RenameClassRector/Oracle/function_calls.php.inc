<?php

\foo();
\Countable(1);
$x = new \Countable();
-----
<?php

\foo();
\Countable(1);
$x = new \stdClass();
