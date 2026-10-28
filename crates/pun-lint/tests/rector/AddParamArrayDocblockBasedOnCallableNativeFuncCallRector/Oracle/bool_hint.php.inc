<?php

final class SomeClass
{
    public function run(array $items): void
    {
        array_map(fn (bool $item) => $item, $items);
    }
}
-----
<?php

final class SomeClass
{
    /**
     * @param bool[] $items
     */
    public function run(array $items): void
    {
        array_map(fn (bool $item) => $item, $items);
    }
}
