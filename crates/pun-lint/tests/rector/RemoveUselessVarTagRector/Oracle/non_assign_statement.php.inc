<?php

namespace App;

final class NonAssignStatement
{
    public function run(int $x): int
    {
        /** @var int $x */
        $x += 1;

        /** @var int $x */
        strlen((string) $x);

        /** @var int $y */
        $y = (int) $x;

        return $x + $y;
    }
}
-----
<?php

namespace App;

final class NonAssignStatement
{
    public function run(int $x): int
    {
        /** @var int $x */
        $x += 1;

        /** @var int $x */
        strlen((string) $x);

        $y = (int) $x;

        return $x + $y;
    }
}
