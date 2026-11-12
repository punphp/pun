<?php

namespace Rector\Tests\CodeQuality\Rector\If_\ExplicitNullableObjectInstanceRector\Oracle;

final class OtherShapes
{
    public function ternaryValue(?\stdClass $first, ?\stdClass $second, bool $flag)
    {
        if ($first ?? $second) {
            return 2;
        }

        return 0;
    }

    public function negatedSide(?\stdClass $first, bool $flag)
    {
        return $first && ! $flag;
    }

    public function comparedSide(?\stdClass $first, int $count)
    {
        return $first || $count > 1;
    }

    public function doWhileLoop(?\stdClass $first)
    {
        do {
            $first = $this->next($first);
        } while ($first);
    }

    private function next(?\stdClass $item): ?\stdClass
    {
        return $item;
    }
}
-----
<?php

namespace Rector\Tests\CodeQuality\Rector\If_\ExplicitNullableObjectInstanceRector\Oracle;

final class OtherShapes
{
    public function ternaryValue(?\stdClass $first, ?\stdClass $second, bool $flag)
    {
        if ($first ?? $second) {
            return 2;
        }

        return 0;
    }

    public function negatedSide(?\stdClass $first, bool $flag)
    {
        return $first instanceof \stdClass && ! $flag;
    }

    public function comparedSide(?\stdClass $first, int $count)
    {
        return $first instanceof \stdClass || $count > 1;
    }

    public function doWhileLoop(?\stdClass $first)
    {
        do {
            $first = $this->next($first);
        } while ($first instanceof \stdClass);
    }

    private function next(?\stdClass $item): ?\stdClass
    {
        return $item;
    }
}
