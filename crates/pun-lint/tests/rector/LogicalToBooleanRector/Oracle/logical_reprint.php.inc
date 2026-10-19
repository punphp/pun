<?php

foo() and $a instanceof B;
foo() and ($a xor $b);
foo() and ++$a;
foo() and !$b = 1;
foo() or $b ? 1 : 2;
foo() and ($b ? 1 : 2);
-----
<?php

foo() && $a instanceof B;
foo() && ($a xor $b);
foo() && ++$a;
foo() && !($b = 1);
foo() || ($b ? 1 : 2);
foo() && ($b ? 1 : 2);
