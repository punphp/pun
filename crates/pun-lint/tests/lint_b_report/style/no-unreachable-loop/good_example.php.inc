<?php

for ($i = 0; $i < 3; $i++) {
    echo $i;
    if ($some_condition) {
        break; // This break is conditional.
    }
}
-----
