<?php
$a = $a = foo();
$a = ($a = foo());
$a = $A = foo();
$this->a = $this->a = foo();
$a = $a = &$b;
$a = $a += 1;
$a = $a = $a = foo();
$$a = $$a = 1;
$a = /* c */ $a = foo();
-----
<?php
$a = foo();
$a = foo();
$a = $A = foo();
$this->a = $this->a = foo();
$a = $a = &$b;
$a = $a += 1;
$a = foo();
$$a = 1;
$a = foo();
