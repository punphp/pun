<?php

final class SomeClass
{
    public function run(array &$items, iterable $v)
    {
        $items[] = $v;
    }
}
-----
<?php

final class SomeClass
{
    /**
     * @param iterable[] $items
     */
    public function run(array &$items, iterable $v)
    {
        $items[] = $v;
    }
}
