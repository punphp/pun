<?php

function g($a = 1, $b = 2) {}

g(b: 1, a: 2);
\g(b: 1, a: 2);
-----
<?php

function g($a = 1, $b = 2) {}

g(a: 2, b: 1);
\g(a: 2, b: 1);
