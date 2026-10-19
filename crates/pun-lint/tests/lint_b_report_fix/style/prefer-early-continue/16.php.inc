<?php

foreach ($a as $b): if ($b): foo(); endif; endforeach;
-----
<?php

foreach ($a as $b): if (!($b)){ continue; }

foo(); endforeach;
