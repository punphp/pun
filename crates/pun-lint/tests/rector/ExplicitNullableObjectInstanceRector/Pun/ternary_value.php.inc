<?php

namespace Rector\Tests\CodeQuality\Rector\If_\ExplicitNullableObjectInstanceRector\Pun;

final class TernaryValue
{
    public function run(?\stdClass $first, ?\stdClass $second, bool $flag)
    {
        // Rector 2.8.4 writes `$flag ? $first : $second instanceof \stdClass`, which checks only `$second`; Pun keeps the parentheses
        if ($flag ? $first : $second) {
            return 1;
        }

        return 0;
    }
}
-----
<?php

namespace Rector\Tests\CodeQuality\Rector\If_\ExplicitNullableObjectInstanceRector\Pun;

final class TernaryValue
{
    public function run(?\stdClass $first, ?\stdClass $second, bool $flag)
    {
        // Rector 2.8.4 writes `$flag ? $first : $second instanceof \stdClass`, which checks only `$second`; Pun keeps the parentheses
        if (($flag ? $first : $second) instanceof \stdClass) {
            return 1;
        }

        return 0;
    }
}
