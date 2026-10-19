<?php

foreach ($a as $b): if ($b): /* c */ foo(); /* d */ endif; endforeach;
-----
<?php

foreach ($a as $b): if (!($b)){ continue; }
 /* c */ foo(); /* d */  endforeach;
