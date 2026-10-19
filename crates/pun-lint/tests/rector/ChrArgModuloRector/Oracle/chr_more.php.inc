<?php

foo(300);
chr(-PHP_INT_MAX);
chr(+FOO);
chr(-300);
chr(99999999999999999999);
chr(~300);
chr(~1.5);
chr(-~300);
chr(~'a');
-----
<?php

foo(300);
chr(-PHP_INT_MAX);
chr(+FOO);
chr(-300 % 256);
chr(99999999999999999999);
chr(~300 % 256);
chr(~1.5 % 256);
chr(-~300 % 256);
chr(~'a');
