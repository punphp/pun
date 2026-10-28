<?php

final class SomeClass
{
    /**
     * @param int
     */
    public function run(array $items, $other): void
    {
        array_map(fn (int $item) => $item, $items);
    }
}
-----
<?php

final class SomeClass
{
    /**
     * @param int
     * @param int[] $items
     */
    public function run(array $items, $other): void
    {
        array_map(fn (int $item) => $item, $items);
    }
}
