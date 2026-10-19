<?php

if (version_compare(PHP_VERSION, '7.0', '>=') === false) {
    echo 1;
}
if (version_compare(PHP_VERSION, '7.0', '>=') !== null) {
    echo 2;
}
if (version_compare(PHP_VERSION, -7.0, '>=')) {
    echo 3;
}
-----
<?php

echo 2;
echo 3;
