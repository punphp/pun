<?php

print 'a.php';
require "a\tb\r\f\v.php";
require "a\nb.php";
-----
<?php

print 'a.php';
require __DIR__ . "/a\tb\r\f\v.php";
require __DIR__ . "/a\nb.php";
