<?php

if ($a)
    if (version_compare(PHP_VERSION, '7.0', '>=')) {
        echo 1;
        echo 2;
    }
-----
<?php

if ($a) {
    echo 1;
    echo 2;
}
