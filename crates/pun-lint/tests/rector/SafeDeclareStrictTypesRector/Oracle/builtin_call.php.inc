<?php

namespace Pun\Oracle\SafeStrict;

strlen("a");
\strlen("b");
-----
<?php

declare(strict_types=1);

namespace Pun\Oracle\SafeStrict;

strlen("a");
\strlen("b");
