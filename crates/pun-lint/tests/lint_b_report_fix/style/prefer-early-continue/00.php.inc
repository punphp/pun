<?php

foreach ($a as $b) {
    if ($b) {
        foo($b);
        bar($b);
    }
}
-----
<?php

foreach ($a as $b) {
    if (!($b)) { continue; }

foo($b);
        bar($b);
}
