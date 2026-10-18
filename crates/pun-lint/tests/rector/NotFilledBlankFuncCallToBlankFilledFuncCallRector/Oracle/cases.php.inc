<?php

$a = !filled($x, 'y');
$b = ! (BLANK($x));
$c = !\filled($x);
$d = !Foo\filled($x);
-----
<?php

$a = blank($x, 'y');
$b = filled($x);
$c = blank($x);
$d = !Foo\filled($x);
