<?php

namespace N;

use function A\b as view;
use A as AA;

function sizeof() {}

view('x');
\view('x');
VIEW('x');
AA\sprintf('x');
\A\sprintf('x');
sizeof($a);
\sizeof($a);
Abc2();
$f = abc2(...);
$g = 'view';
$g();
-----
<?php

namespace N;

use function A\b as view;
use A as AA;

function sizeof() {}

view('x');
\Laravel\Templating\render('x');
VIEW('x');
AA\sprintf('x');
\A\sprintf('x');
sizeof($a);
count($a);
abc();
$f = abc(...);
$g = 'view';
$g();
