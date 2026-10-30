<?php

final class SomeClass
{
    /**
     * @template T
     * @param T $v
     */
    public function run(array &$items, $v)
    {
        $items[] = $v;
    }
}
-----
<?php

final class SomeClass
{
    /**
     * @template T
     * @param T $v
     * @param T[] $items
     */
    public function run(array &$items, $v)
    {
        $items[] = $v;
    }
}
