<?php

final class SomeClass
{
    public function run(array $items): void
    {
        array_map(fn (int|string $item) => $item, $items);
    }
}
-----
<?php

final class SomeClass
{
    /**
     * @param int[]|string[] $items
     */
    public function run(array $items): void
    {
        array_map(fn (int|string $item) => $item, $items);
    }
}
