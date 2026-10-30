<?php

Foo\view('x');
view('x');
$x->view('y');
-----
<?php

Foo\view('x');
\Laravel\Templating\render('x');
$x->view('y');
