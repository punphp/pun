<?php

final class SomeClass
{
    public function run(array $items): void
    {
        array_map(fn (self $item) => $item, $items);
    }
}
-----
<?php

final class SomeClass
{
    /**
     * @param $this[] $items
     */
    public function run(array $items): void
    {
        array_map(fn (self $item) => $item, $items);
    }
}
