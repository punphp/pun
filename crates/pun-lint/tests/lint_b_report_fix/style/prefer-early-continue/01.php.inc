<?php

foreach ($a as $b) {
    if (!$b->ok()) {
        foo($b);
    }
}
-----
<?php

foreach ($a as $b) {
    if ($b->ok()) { continue; }

foo($b);
}
