<?php

namespace Probe;

interface Left
{
}

interface Right
{
}

final class Reduce
{
    /**
     * @param  list<int>  $items
     */
    public function sameType(array $items, int $start)
    {
        return array_reduce($items, function ($carry, $item) {
            return (int) $item;
        }, $start);
    }

    /**
     * @param  list<int>  $items
     */
    public function falseInitial(array $items)
    {
        return array_reduce($items, function ($carry, $item) {
            return (int) $item;
        }, false);
    }

    /**
     * @param  list<int>  $items
     */
    public function intersection(array $items, (Left&Right)|null $start)
    {
        return array_reduce($items, function ($carry, $item) {
            return (int) $item;
        }, $start);
    }
}
-----
<?php

namespace Probe;

interface Left
{
}

interface Right
{
}

final class Reduce
{
    /**
     * @param  list<int>  $items
     */
    public function sameType(array $items, int $start)
    {
        return array_reduce($items, function (int $carry, int $item) {
            return (int) $item;
        }, $start);
    }

    /**
     * @param  list<int>  $items
     */
    public function falseInitial(array $items)
    {
        return array_reduce($items, function (int|false $carry, int $item) {
            return (int) $item;
        }, false);
    }

    /**
     * @param  list<int>  $items
     */
    public function intersection(array $items, (Left&Right)|null $start)
    {
        return array_reduce($items, function ($carry, int $item) {
            return (int) $item;
        }, $start);
    }
}
