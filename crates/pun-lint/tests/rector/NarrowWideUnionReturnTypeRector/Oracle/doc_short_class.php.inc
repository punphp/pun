<?php

namespace Rector\Tests\DeadCode\Rector\FunctionLike\NarrowWideUnionReturnTypeRector\Oracle;

use DateTime;

final class DocShortClass
{
    /**
     * @return DateTime|string
     */
    public function run(): DateTime|string|int
    {
        return new DateTime();
    }
}
-----
<?php

namespace Rector\Tests\DeadCode\Rector\FunctionLike\NarrowWideUnionReturnTypeRector\Oracle;

use DateTime;

final class DocShortClass
{
    /**
     * @return DateTime
     */
    public function run(): \DateTime
    {
        return new DateTime();
    }
}
