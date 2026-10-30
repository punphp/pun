<?php

namespace Rector\Tests\DeadCode\Rector\FunctionLike\NarrowWideUnionReturnTypeRector\Oracle;

use DateTime as Moment;
use DateTimeImmutable;

final class DocNames
{
    /**
     * @return Moment|string
     */
    public function aliased(): Moment|string|int
    {
        return new Moment();
    }

    /**
     * @return \DateTime|string
     */
    public function qualified(): \DateTime|string|int
    {
        return new \DateTime();
    }

    /**
     * @return Moment|DateTimeImmutable|string
     */
    public function union(bool $flag): Moment|DateTimeImmutable|string
    {
        return $flag ? new Moment() : new DateTimeImmutable();
    }
}
-----
<?php

namespace Rector\Tests\DeadCode\Rector\FunctionLike\NarrowWideUnionReturnTypeRector\Oracle;

use DateTime as Moment;
use DateTimeImmutable;

final class DocNames
{
    /**
     * @return Moment
     */
    public function aliased(): \DateTime
    {
        return new Moment();
    }

    /**
     * @return \DateTime
     */
    public function qualified(): \DateTime
    {
        return new \DateTime();
    }

    /**
     * @return Moment|DateTimeImmutable
     */
    public function union(bool $flag): \DateTime|\DateTimeImmutable
    {
        return $flag ? new Moment() : new DateTimeImmutable();
    }
}
