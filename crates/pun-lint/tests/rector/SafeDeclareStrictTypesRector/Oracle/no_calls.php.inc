<?php

namespace Pun\Oracle\SafeStrict;

echo 1;
$a = 1;
$b = [];
-----
<?php

declare(strict_types=1);

namespace Pun\Oracle\SafeStrict;

echo 1;
$a = 1;
$b = [];
