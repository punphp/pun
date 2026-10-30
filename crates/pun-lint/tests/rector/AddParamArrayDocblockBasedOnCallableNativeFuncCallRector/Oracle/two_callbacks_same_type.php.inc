<?php

final class SomeClass
{
    public function run(array $items): void
    {
        array_map(fn (int $item) => $item, $items);
        array_filter($items, fn (int $item) => true);
    }
}
-----
<?php

final class SomeClass
{
    /**
     * @param int[] $items
     */
    public function run(array $items): void
    {
        array_map(fn (int $item) => $item, $items);
        array_filter($items, fn (int $item) => true);
    }
}
