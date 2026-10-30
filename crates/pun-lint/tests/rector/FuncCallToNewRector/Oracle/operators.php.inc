<?php

$a = collection() . 'x';
$b = collection()[0];
$c = $f();
-----
<?php

$a = new \Collection() . 'x';
$b = (new \Collection())[0];
$c = $f();
