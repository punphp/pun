<?php

array_search($a, $b);
array_search($a, $b,);
\array_search($a, $b);
array_search($a, $b, false);
array_search($a);
array_search(...$args);
Foo\array_search($a, $b);
-----
<?php

array_search($a, $b, true);
array_search($a, $b, true,);
\array_search($a, $b, true);
array_search($a, $b, false);
array_search($a);
array_search(...$args);
Foo\array_search($a, $b);
