<?php

SLEEP(1);
\usleep(2, 3);
(sleep(1));
Foo\sleep(1);
$a->sleep(1);
@sleep(1);
-----
<?php

\Illuminate\Support\Sleep::sleep(1);
\Illuminate\Support\Sleep::usleep(2, 3);
(\Illuminate\Support\Sleep::sleep(1));
Foo\sleep(1);
$a->sleep(1);
@sleep(1);
