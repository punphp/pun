<?php

// Pun: Rector repeats a compound assignment in every new statement, so its effect would run twice; that chain
// stays. Rector also drops the parentheses around `and`, which binds looser than `=`; Pun keeps them.
$a = $b = $c .= 6;
$a = $b = ($c and $d);
-----
<?php

// Pun: Rector repeats a compound assignment in every new statement, so its effect would run twice; that chain
// stays. Rector also drops the parentheses around `and`, which binds looser than `=`; Pun keeps them.
$a = $b = $c .= 6;
$a = ($c and $d);
$b = ($c and $d);
