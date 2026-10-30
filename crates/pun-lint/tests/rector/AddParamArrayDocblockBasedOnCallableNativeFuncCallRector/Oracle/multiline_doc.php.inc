<?php

final class SomeClass
{
    /**
     * @param array<
     *     int
     * > $other
     */
    public function run(array $items): void
    {
        array_map(fn (int $item) => $item, $items);
    }
}
-----
<?php

final class SomeClass
{
    /**
     * @param array<
     *     int
     * > $other
     * @param int[] $items
     */
    public function run(array $items): void
    {
        array_map(fn (int $item) => $item, $items);
    }
}
