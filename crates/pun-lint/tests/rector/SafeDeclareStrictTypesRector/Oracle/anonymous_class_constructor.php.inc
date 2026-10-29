<?php

namespace Pun\Oracle\SafeStrict;

$a = new class (1) {
    public function __construct(int $i)
    {
    }
};
-----
<?php

declare(strict_types=1);

namespace Pun\Oracle\SafeStrict;

$a = new class (1) {
    public function __construct(int $i)
    {
    }
};
