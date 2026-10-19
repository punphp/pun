<?php

$a = version_compare(PHP_VERSION, '8.1', '>=');
$b = version_compare('5.6.0', PHP_VERSION, 'lt');
$c = !version_compare(PHP_VERSION, '7.4.0-dev', 'ne');
$d = version_compare(\PHP_VERSION, '8', '==') . 'x';
$e = version_compare(PHP_VERSION, $v, '>=');
$g = version_compare(PHP_VERSION, '8.1');
$h = version_compare(php_version, '8.1', '>');
$i = version_compare(PHP_VERSION, PHP_VERSION, '<=');
$j = version_compare('1.0', '2.0', '<');
-----
<?php

$a = PHP_VERSION_ID >= 80100;
$b = 50600 < PHP_VERSION_ID;
$c = !(PHP_VERSION_ID !== 70400);
$d = (PHP_VERSION_ID === 8) . 'x';
$e = version_compare(PHP_VERSION, $v, '>=');
$g = version_compare(PHP_VERSION, '8.1');
$h = version_compare(php_version, '8.1', '>');
$i = PHP_VERSION_ID <= PHP_VERSION_ID;
$j = version_compare('1.0', '2.0', '<');
