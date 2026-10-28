<?php

final class SomeClass
{
    public function run(array ...$items): void
    {
        array_map(fn (int $item) => $item, $items);
    }
}
-----
<?php

final class SomeClass
{
    /**
     * @param int[] ...$items
     */
    public function run(array ...$items): void
    {
        array_map(fn (int $item) => $item, $items);
    }
}
